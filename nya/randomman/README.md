# lambdaman random walk solution

## usage

Searching seeds:

```
$ cargo run --release -p randomman -- search [--rng=<name>] [--stride=<N>] <problemID>
```

Compiling code with a seed:

```
$ cargo run --release -p randomman -- compile [--rng=<name>] [--stride=<N>] <problemID> <seed>
```
