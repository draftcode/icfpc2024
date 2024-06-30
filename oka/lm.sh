#!/bin/bash

# Usage: ./oka/lm.sh path/to/file.scm

set -ex

p=$(realpath $1)

cd "$(dirname "$0")/.."

which gosh

gosh $p | cargo run -r --bin lm run

cargo run -r --bin scmcomp submit < $p
