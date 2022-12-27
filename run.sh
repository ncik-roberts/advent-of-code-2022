#!/bin/bash

n=$1
shift

cd "day$n"
cargo run -- "$@"
