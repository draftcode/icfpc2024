# Team Spica / ICFP Programming Contest 2024

## Members

- [@chiro](https://github.com/chiro/)
- [@draftcode](https://github.com/draftcode/)
- [@fuqinho](https://github.com/fuqinho/)
- [@nya3jp](https://github.com/nya3jp/)
- [@ogiekako](https://github.com/ogiekako/)
- [@phoenixstarhiro](https://github.com/phoenixstarhiro/)
- [@shunsakuraba](https://github.com/shunsakuraba/)
- [@tanakh](https://github.com/tanakh/)

## Programming languages

- Rust for most part of our solutions
- Python for small tools
- TypeScript and Python for infrastructure stuff
- Google Spreadsheet for 3d programming.

## Overview

TODO(all): Describe the overview of our work

## Notable work

### General

#### Common ICFP expression library

[./common](./common)

TODO(all): Describe this

Rust macro to embed ICFP expressions in Rust code:
[./tanakh/solver/src/bin/lambdaman.rs](./tanakh/solver/src/bin/lambdaman.rs)

#### Interactive communicator

[./communicate](./communicate)

TODO(nya): Describe this

#### ICFP-to-Haskell/Scheme converter

[./tanakh/evaluator](./tanakh/evaluator)

TODO(tanakh): Describe this

#### Scheme-to-ICFP compiler

[./scmcomp](./scmcomp)

TODO(ogiekako): Describe this

#### Infrastructure

[./frontend](./frontend)
[./backend_py](./backend_py)
[./backend_rs](./backend_rs)

TODO(draftcode): Describe this

### Lambdaman

#### Random walk solution generator

[./nya/randomman](./nya/randomman)

We used random walk for many problems (4, 5, 7, 10, 11, 12, 13, 14, 15, 17, 18,
21). For each program, we searched for a random seed that collects all pills in
1,000,000 moves. Essentially this is the ICFP program we used in all problems:

```
(fix (fn f s ->
  (if (== s END_SEED) {
    "solve lambdamanXX "
  } else {
    (concat
      (f (% (* s 48271) 18446744073709551557))
      (take 2 (drop (* (/ s 4611686018427387904) 2) "LLUUDDRR")))
  })
) START_SEED)
```

### Spaceship

#### chun's solvers

[./chun](./chun)

I created an input pre-ordering utility [spaceship_order](chun/spaceship_order), a solver [speceship_lasolver](chun/spaceship_lasolver), and one big solver-utility-set [speceship_analytical](chun/spaceship_analytical).

`spaceship_order` reorders input so that the points are visited by the spaceship in this order. It is mostly greedy order, but the simulated annealing was also used depending on the map.

`spaceship_lasolver` tries to visit the target in pre-sorted order with a A*-like heuristic search. It tries to "look-ahead" one target; suppose the ship is at `p0` with `v0` and  tries to visit `p1` then `p2`. The program searches the fastest route to `p2` via `p1`, then use  only the path between `p0` and `p1` (trash `p1` to `p2` path), then

`spaceship_analytical` implemnts several utility as well as optimizer for the already generated keypad seqeunce. First it converts the keypad sequence to the vector of `(point, velocity)`. Then the program tries to locally optimize the sequence. There are three optimizers inside. (1) 3-pt optimizer, take `p0-p1-p2` visited in this order, then change `v1` preserving `v0` and `v2`. (2) 4-pt optimizer. (3) swap optimizer, take `p0-p1-p2-p3` and try `p0-p1-p2-p3` visiting sequence.

#### tanakh's solver

[./tanakh/spaceship](./tanakh/spaceship)

TODO(tanakh): Describe this

#### fuqinho's solver

[./fuqinho/fuqinho-spaceship](./fuqinho/fuqinho-spaceship)

1. Determine visiting order using simulated annealing. (score is total traveling distance)
2. Precompute minimum steps to move dx with initial velocity v0 and terminal velocity v1. 
   - -100 <= v0, v1 <= 100, -10000 <= dx <= 10000
3. In 2-dimension movement, calculate the lower bound of steps using above data and search the minimum steps by incrementing target steps.
4. Using the data above, efficiently list up several possible paths from point 1 to point 2.
5. Find paths to visit all given points using beam search. (We keep best 1000 routes at each visited point)

### 3D

#### 3D interpreter

[./chir/interpreter](./chir/interpreter)

We implemented an interpreter for the 3D course in Rust. It implements all noted features and also has the labeling mechanism for easier development.
For example, the interpreter can run the following program:

```
. A     here . .
. v     .    . .
. .     .    . .
. @here .    . .
. .1    .    . .
```

The interpreter automatically fills the jump location for the time warp operator by looking up the label name, 'here'.

The interpreter also provides a mode to convert a program with labels into a program which can be submitted to the server.

### Efficiency

TODO(all): Add work about efficiency here
