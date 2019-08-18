---
layout: post
title: 'Sekiro practice tool: the architecture'
date: 2019-08-18 14:30
---

I have recently (well, since *forever*) been working on a number of practice
tools for speedruns. I must have rewritten the [Dark Souls III practice tool](https://github.com/veeenu/DarkSoulsIII-Mods)
from scratch no less than three or four times, and not a single time have I
been completely satisfied with the results, in terms of features, user experience
and reliability. Though, as I've also been working on [practice tools for Sekiro](https://github.com/veeenu/sekiro-practice-tool),
I believe I have finally found a satisfying setup after much experimentation.

I place much value on developing software which is self-contained, has as few
external dependencies as possible (zero, ideally), is unobtrusive to the user
both in terms of installation and usability. My ideal software project has a
single-file output, can be launched from its own folder, creates as little
on-screen clutter as possible and fails gracefully (i.e. reverting to sane
defaults on exceptions). The last iteration of my practice tool for Sekiro
definitely checks all of those marks, hence I decided to talk about its
architecture in this blog post. 

The most relevant, eye-catching feature is user-experience related: as the
tool is meant to be used in-game, no external windows are created, and the UI
is simply drawn over the game, in a fashion similar to Steam's overlay.
Let us start discussing that.

## How does injecting DirectX11 overlays work?

**Disclaimer:** I'm taking little to no credit for what's going on here, as the
idea was sourced from [timb3r's excellent, in-depth tutorial](https://gamephreakers.com/2018/12/hooking-directx-11-ue4-dauntless/).
I wanted to re-formulate the concepts because I believe that, at this level
of complexity, reading stuff from different sources can help clarify some
ideas, and to fill in some blanks for people completely new to the practice.
It's what I would have wanted to read when I was trying to figure this out.

In simple terms, our goal is to draw our interface over the game's own
rendered output, and show that to the user. Peeling off a layer of
abstraction, we want to find a drawable surface and the code where said
surface is shown to the user, and there inject code which will be tasked with
doing its own rendering and showing the surface. Hopping off another layer of abstraction
from there and straight into DirectX11 land, we find the [`IDXGISwapChain` interface](https://docs.microsoft.com/en-us/windows/win32/api/dxgi/nn-dxgi-idxgiswapchain)
and its [`Present()` method](https://docs.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiswapchain-present).

As per the documentation, the `IDXGISwapChain` interface "*implements one or
more surfaces for storing rendered data before presenting it to an output*",
and the `Present()` method "*presents a rendered image to the user*". That
is, the `Present()` method holds rendered data and shows output.

We want to access the rendered data in order to manipulate it, by injecting
code right before it is presented. This is accomplished as follows:

- Find the address of the DX11 function `IDXGISwapChain::Present()`, and set it aside.
- Create another function with the same signature, and hook it in place of the aforementioned function;
  this way, the game will call the new function instead of the old one.
- In the new function, perform all the UI-related actions and then call the original `IDXGISwapChain::Present()`
  function with the same parameters in order to replicate the rendering engine's intended behavior.

In other words, we want to go from this

```
                .---------------------.
  .--------.    |    .-----------.    |
  | Sekiro |----`--->| Present() |----'
  '--------'         '-----------'
```

to this

```
                 .------------------------.
  .--------.     |       .-----------.    |
  | Sekiro |-.   |       | Present() |----'
  '--------' |   |       '-----------'
             |   |                ^    
             |   |    .---------. |    
             `---`--->| Tool UI |-'
                      '---------'
```

### Hooking `IDXGISwapChain::Present`

Now let's get into the hooking specifics. In order to reroute the `Present()`
function over to our own implementation, we are going to rely on the
[MinHook](https://github.com/TsudaKageyu/minhook) framework. The interesting
bits of the process follow. First of all, we load the `dxgi.dll` library,
then compute and store the offset of the `Present()` function.

```cpp
// Callback typedef
typedef HRESULT(__fastcall *IDXGISwapChainPresent)(IDXGISwapChain *pSwapChain,
                                                   UINT SyncInterval,
                                                   UINT Flags);
// Get a handle to the library
DWORD_PTR hDxgi = (DWORD_PTR)GetModuleHandle(L"dxgi.dll");
// Store IDXGISwapChain::Present()'s address in presentOriginal
LPVOID presentOriginal = reinterpret_cast<LPVOID>(
      (IDXGISwapChainPresent)((DWORD_PTR)hDxgi + 0x5070));
```

After that, we want to perform the following actions:

- Patch the original `Present()` function with unconditional `JMP` instructions
  pointing to our implementation, `PresentImpl()`.
- Build a *trampoline* function: write a copy of the original `Present()`
  function's initial instructions over to a newly allocated memory block, and
  append to it an unconditional `JMP` pointing to the rest of the original
  `Present()` function.

Confusing? How about this:

```
.-------------------------.
| IDXGISwapChain::Present |
|-------------------------|
| instruction1            |
| instruction2            |
| instruction3            |
| ...                     |
'-------------------------'
```

becomes (simplified):

```
.----------------------------.       .-------------------------.
| Trampoline                 |       | IDXGISwapChain::Present |
|----------------------------|       |-------------------------|
| instruction1               |       | JMP PresentImpl         |--.
| JMP Present @ instruction2 |------>| instruction2            |  |
|                            |       | instruction3            |  |
| ...                        |       | ...                     |  |
'----------------------------'       '-------------------------'  |
                                                                  |
          .-------------.                                         |
          | PresentImpl |<----------------------------------------'
          |-------------|
          | ...         |
          '-------------'
```

Basically, when we call our *trampoline* we will get the same effect that we
would've had if we called the non-patched `IDXGISwapChain::Present()`
function, and when we call `IDXGISwapChain::Present()` we will simply be
calling `PresentImpl` in its stead. Further details can be found in
[MinHook's codeproject page](https://www.codeproject.com/Articles/44326/MinHook-The-Minimalistic-x-x-API-Hooking-Libra).

This can be done by relying on MinHook's `MH_CreateHook` and `MH_EnableHook`
functions. Our trampoline's address will be stored into the
`presentTrampoline` variable, and the `IDXGISwapChain::Present()` function,
previously stored at `presentOriginal`, will be patched with a `JMP` to our
implementation, the `PresentImpl()` function.

```cpp
IDXGISwapChainPresent presentTrampoline;
MH_CreateHook(presentOriginal, &PresentImpl,
              reinterpret_cast<LPVOID *>(&presentTrampoline));
MH_EnableHook(presentOriginal);
```

### Writing the `PresentImpl()` function

After the hook is in place, we can move on to implement our `PresentImpl()` function (full code [here](https://github.com/veeenu/sekiro-practice-tool/blob/346a5cff805a1ba014fa46441b803cc3e400fb47/src/dll/dllmain.cpp#L68)):

```cpp
HRESULT __fastcall PresentImpl(IDXGISwapChain *pChain, UINT SyncInterval,
                           UINT Flags) {
  if (!initialized) {
    if (FAILED(GetDeviceAndCtxFromSwapchain(pChain, &pDevice, &pContext)))
      return presentTrampoline(pChain, SyncInterval, Flags);

    ImGui::CreateContext();
    ImGuiIO &io = ImGui::GetIO(); // ...

    ImGui_ImplDX11_Init(pDevice, pContext); // ...

    ID3D11Texture2D *pBackBuffer;

    pChain->GetBuffer(0, __uuidof(ID3D11Texture2D), (LPVOID *)&pBackBuffer);
    pDevice->CreateRenderTargetView(pBackBuffer, NULL, &mainRenderTargetView);
    pBackBuffer->Release();

    initialized = true;
  }

  ImGui_ImplDX11_NewFrame(); // ...

  ImGui::NewFrame();
  // --> ImGui rendering code here <--
  ImGui::EndFrame();
  ImGui::Render();

  pContext->OMSetRenderTargets(1, &mainRenderTargetView, NULL);
  ImGui_ImplDX11_RenderDrawData(ImGui::GetDrawData());

  return presentTrampoline(pChain, SyncInterval, Flags);
}
```

When the function is first run, an attempt is done to retrieve the context
from the swap chain. If none is found, we let the original
`IDXGISwapChain::Present()` implementation do its magic without intervening
by summoning the trampoline function, until we finally get one. At that
point, we run our own `ImGui` initialization code, which relies on the official
[Win32](https://github.com/ocornut/imgui/blob/master/examples/imgui_impl_win32.cpp) +
[DX11](https://github.com/ocornut/imgui/blob/master/examples/imgui_impl_dx11.cpp)
sample implementations, and set a flag so we know the initialization has
taken place. After that, it is enough to wrap our code in between calls to
`ImGui::NewFrame()` and `ImGui::EndFrame()`, summon `ImGui::Render()`,
retrieve the data, paint it over our drawing context, and finally yielding
control to the original `IDXGISwapChain::Present()` function via the trampoline
so it can present the whole of the rendered data to the user.

## Performing DLL Injection

The tool's core functionality is simply about toggling flags and editing
numerical values in the game's memory. While, on principle, one could rely on Windows'
[`ReadProcessMemory`](https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-readprocessmemory)
and [`WriteProcessMemory`](https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-writeprocessmemory)
APIs in order to completely avoid altering the game's memory space more
than is strictly necessary, getting to play with pointer arithmetics
directly from inside of it feels much more fun.

In order to do that, most videogame mods out there rely on DLL hooking: that is,
the process of crafting a DLL with a name which is renownedly loaded by our target
application, and from its entry point (the `DllMain` exported symbol) load the original
library and hook a function in the same way we hooked the `Present()` function above.
Then, put the crafted library in a
[higher-priority position](https://docs.microsoft.com/en-us/windows/win32/dlls/dynamic-link-library-search-order)
(most likely the same directory as the target application's executable) so it is loaded
in its stead. For most DirectX games, you can override and hook the
`dinput8.dll` library. As an example, see [Jiiks' UniversalProxyChain project](https://github.com/Jiiks/UniversalProxyChain)
which is used by some Sekiro speedrunning mods.

This had a couple drawbacks for my project: first of all, the method was
already occupied by Jiiks' tool, and second, enabling and disabling the
practice tool would've required to manually copy and delete the DLL file
every time, and that makes for a rather uncomfy user experience.

To overcome this, I decided to adopt a different method and build an
external DLL injector, which is simply an `.exe` file: you double click
it, and the DLL is injected into the game's memory space.

How this can happen is rather easy: it is enough to call the
[`LoadLibrary`](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibrarya)
Windows API into the game's process via the
[`CreateRemoteThread`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createremotethread)
API. `LoadLibrary`'s argument is simply the loaded DLL full path; once
called, the DLL's entry point `DllMain()` function is called, and that
is everything we need to get the ball rolling.

```cpp
// Find process id and open it
DWORD pid = find_process("sekiro.exe");
HANDLE h_process = OpenProcess(PROCESS_ALL_ACCESS, FALSE, pid);

// Remotely allocate memory and write there the full_dll_path
void* proc_dll_path = VirtualAllocEx(h_process, NULL, _MAX_PATH, MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE);
WriteProcessMemory(h_process, proc_dll_path, full_dll_path, _MAX_PATH, NULL);

// Call the LoadLibraryA API with the freshly allocated string parameter
HANDLE h_thread = CreateRemoteThread(
  h_process, NULL, 0,    
  (LPTHREAD_START_ROUTINE) GetProcAddress(GetModuleHandle(__TEXT("Kernel32")), "LoadLibraryA"), 
  proc_dll_path, 0, NULL);
```

## Conclusions

This is it for the core architectural aspects of this small mod. I'm planning
to refactor however much reusable code I can from the principles outlined in
its own library, with an abstracted UI wrapper over the ImGui library,
possibly a scripting language to access the game's underlying memory and
factor out all the heavy lifting instead of scattering repeated code around.

In the meantime, if you're curious about the project, [check it out](https://github.com/veeenu/sekiro-practice-tool/tree/346a5cff805a1ba014fa46441b803cc3e400fb47)!