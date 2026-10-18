+++
template = "article.html"
title = "Learning Haskell with Venu: Cellular automata"
aliases = ["/2014/10/20/learning-haskell-with-venu-pt-i.html"]
+++

Lately I've been experimenting a lot with a number of CG and procedural content
generation algorithms, and began feeling the need for mathematical rigour over
the practical, "real life situation" style I've been used to by writing
Javascript day in, day out. I got to a point where simple simulations required
way more boilerplate code than I was willing to write, and the usage of native
numeric constructs began getting in the way of performance. 
<!-- more -->
For example, I was
working on an {% <tex> %} O(n) {% </tex> %} image processing algorithm, representing the pixels as
plain 1D arrays, and I discovered that wrapping the aforementioned array
inside a native `Uint32Array` produced an impressive performance boost, cutting
the execution time from ~46ms to ~1.5ms at the expense of adding a little
boilerplate code. Which is great in and of itself, but not so great in terms of
code cleanliness, and code cleanliness is fundamental when I'm trying to
represent somewhat complex mathematical ideas.

So I decided to look into functional programming. After reading about it for a
bit, I began working my way through the [Ninety-nine Haskell Problems](https://github.com/veeenu/ninetynine-haskell-problems)
to get comfortable with the new paradigm, and then decided to prototype
something a bit more challenging.

### Cellular automata

I chose to implement a simple cellular automaton. A *cellular automaton* is a
discrete computation model defined over a grid of cells, each one having a
state. Each step of the computation, called *generation*, involves computing
a new state for each cell that depends upon the previous states of the cell
and of a chosen subset of neighboring cells. The criteria with which the new
state is computed are called *rules* and are, basically, a function of those
previous states.

Let's choose a neighborhood in the form of the [Moore neighborhood](https://en.wikipedia.org/wiki/Moore_neighborhood).
Let {% <tex> %} A {% </tex> %} denote the 2D matrix representing the automaton. The Moore
neighborhood for a generic cell {% <tex> %} A_{x,y} {% </tex> %} is defined as

{% <tex> %}
N_{x,y}(A) = \left\{ A_{x+i,y+j} \mid (i, j) \in [-1 .. 1]^2 \setminus (0, 0) \right\}
{% </tex> %}

Let {% <tex> %} A^i {% </tex> %} be the matrix at the {% <tex> %}i{% </tex> %}th generation. We could say that

{% <tex> %}
A^i_{x,y} = f(A^{i-1}_{x,y},N_{x,y}(A^{i-1}))
{% </tex> %}

Finally, we could decide our rule function {% <tex> %}f{% </tex> %} to be a simplified version of the
one described in this [nice article](https://gamedevelopment.tutsplus.com/tutorials/generate-random-cave-levels-using-cellular-automata--gamedev-9664)
about cellular automata which I suggest you read, too.

{% <tex> %}
f(a_0,a_1...a_n) = \begin{cases} 1 &\mbox{} \sum\limits_{i=1}^n a_i > 4 \\
                   0 &\mbox{} otherwise \end{cases}
{% </tex> %}

I decided to represent the automaton's grid as a plain 1D array. The `generation`
function is defined inductively with great ease; being `vec` the array containing
the grid, the generation 0 is the array itself, and each subsequent generation
is defined as the `step` function mapped over the values of the previous
generation. `step` corresponds to the {% <tex> %}f{% </tex> %} defined before, and the sum over the
neighborhood is represented as the fold of the sum operation over the cell values
extracted by the `getCell` function.

```haskell
generation :: Int -> [Int] -> [Int]

generation 0 vec = vec
generation i vec = map step [0 .. length vec]
  where
    vec' = generation (i-1) vec
    step :: Int -> Int
    step i = if (neighSum > 4) then 1 else 0
      where
        x = i `mod` 32
        y = i `div` 32
        neighSum = foldr (+) 0 [getCell (x + x') (y + y') vec' | x' <- [-1..1], y' <- [-1..1], not (x' == 0 && y' == 0) ]
```

Coupled with Haskell's `System.Random`, we get a nice looking dungeon after a few generations:

![Cellular Automaton](/data/2014-10-20/ca_.png)

You can read the whole code here: [https://gist.github.com/veeenu/8dbba3e53f94d7142c63](https://gist.github.com/veeenu/8dbba3e53f94d7142c63).

Surely this could have been done a lot better, and it will be, as I keep studying the
language and getting better at it. Meanwhile, please feel free to criticize and comment
whatever you feel like to! :)

**ERRATA CORRIGE**. My friend Roberto pointed out that the 2-tuple {% <tex> %}(0, 0){% </tex> %} wasn't supposed
to be part of the Moore neighborhood, and, besides that, its status was already taken into
account for (but not used in this particular case) in the form of {% <tex> %}a_0{% </tex> %} in the rule
function anyway. This has now been corrected both in the code and in the mathematical
expression.
