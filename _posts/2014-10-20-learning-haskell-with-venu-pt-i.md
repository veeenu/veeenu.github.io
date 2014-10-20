---
layout: post
title: 'Learning Haskell with Venu, part I: Cellular automata'
date: 2014-10-20 17:30
---

Lately I've been experimenting a lot with a number of CG and procedural content
generation algorithms, and began feeling the need for mathematical rigour over
the practical, "real life situation" style I've been used to by writing
Javascript day in, day out. I got to a point where simple simulations required
way more boilerplate code than I was willing to write, and the usage of native
numeric constructs began getting in the way of performance. For example, I was
working on an $$ O(n) $$ image processing algorithm, representing the pixels as
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

Let's choose a neighborhood in the form of the [Moore neighborhood](http://en.wikipedia.org/wiki/Moore_neighborhood).
Let $$ A $$ denote the 2D matrix representing the automaton. The Moore
neighborhood for a generic cell $$ A_{x,y} $$ is defined as

$$
N_{x,y}(A) = \left\{ A_{x+i,y+j} \mid i, j \in [-1 .. 1] \right\}
$$

Let $$A^i$$ be the matrix at the $$i$$th generation. We could say that

$$
A^i_{x,y} = f(A^{i-1}_{x,y},N_{x,y}(A^{i-1}))
$$

Finally, we could decide our rule function $$f$$ to be a simplified version of the
one described in this [nice article](http://gamedevelopment.tutsplus.com/tutorials/generate-random-cave-levels-using-cellular-automata--gamedev-9664)
about cellular automata which I suggest you read, too.

$$
f(a_0,a_1...a_n) = \begin{cases} 1 &\mbox{} \sum\limits_{i=1}^n a_i > 4 \\
                   0 &\mbox{} otherwise \end{cases}
$$

I decided to represent the automaton's grid as a plain 1D array. The `generation`
function is defined inductively with great ease; being `vec` the array containing
the grid, the generation 0 is the array itself, and each subsequent generation
is defined as the `step` function mapped over the values of the previous
generation. `step` corresponds to the $$f$$ defined before, and the sum over the
neighborhood is represented as the fold of the sum operation over the cell values
extracted by the `getCell` function.

{%highlight haskell %}
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
        neighSum = foldr (+) 0 [getCell (x + x') (y + y') vec' | x' <- [-1..1], y' <- [-1..1]]
{%endhighlight%}

Coupled with Haskell's `System.Random`, we get a nice looking dungeon after a few generations:

![Cellular Automaton](/data/2014-10-20/ca_.png)

You can read the whole code here: [https://gist.github.com/veeenu/8dbba3e53f94d7142c63](https://gist.github.com/veeenu/8dbba3e53f94d7142c63).

Surely this could have been done a lot better, and it will be, as I keep studying the
language and getting better at it. Meanwhile, please feel free to criticize and comment
whatever you feel like to! :)
