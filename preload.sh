#!/bin/bash

cd "$(dirname "$0")"

lib=$(realpath 'target/i686-unknown-linux-gnu/debug/liboxide.so')

if [[ ! -f $lib ]]; then
    echo "$lib not found"
    exit 1
fi
LD_PRELOAD=$lib ./start_tf2.sh

