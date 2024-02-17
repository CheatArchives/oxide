#!/bin/bash

if [[ ! $(id -u) -eq 0 ]]; then
    echo "This script must not be ran as root!"
    exit 1
fi

cd "$(dirname "$0")"

lib=$(realpath 'target/i686-unknown-linux-gnu/debug/liboxide.so')
pid=$(pidof -s "hl2_linux")

if [[ ! -f $lib ]]; then
    echo "$lib not found"
    exit 1
fi

if [ -z "$pid" ]; then
   echo "tf2 not running"
   exit 1
fi

gdb -n -q -batch                                       \
    -ex "attach $pid"                                  \
    -ex "set \$dlopen = (void* (*)(char*, int))dlopen" \
    -ex "set \$dlerror =  (char* (*)(void))dlerror"    \
    -ex "call \$dlopen(\"$lib\", 2)"                   \
    -ex "call \$dlerror()"                             \
    -ex "continue"                                     


