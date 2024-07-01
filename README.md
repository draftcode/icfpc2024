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
- TypeScript and Python for infrastructure stuff

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

TODO(nya): Describe this

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

TODO(fuqinho): Describe this

### 3D

#### 3D interpreter

[./chir/interpreter](./chir/interpreter)

TODO(chir): Describe this

### Efficiency

TODO(all): Add work about efficiency here
