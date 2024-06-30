# lambdaman random walk solution

## usage

Searching seeds:

```
$ cargo run --release -p randomman -- search --stride=<N> [--rng=<name>] [--start-seed=<seed>] <problemID>
```

Compiling code with a seed:

```
$ cargo run --release -p randomman -- compile --stride=<N> [--rng=<name>] <problemID> <seed>
```

Compiling and submitting code with a seed:

```
$ cargo run --release -p randomman -- submit --stride=<N> [--rng=<name>] <problemID> <seed>
```

Submitting all known solutions:

```
$ cargo run --release -p randomman -- submit-all
```
