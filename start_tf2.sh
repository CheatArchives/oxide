#!/bin/bash

if [[ $(id -u) -eq 0 ]]; then
    echo "This script must not be ran as root!"
    exit 1
fi

cd "$(dirname "$0")"

pid=$(pidof -s "hl2_linux")

if [ -n "$pid" ]; then
   echo "tf2 running"
   exit 1
fi

tf2_dir=$(realpath ~/.local/share/Steam/steamapps/common/Team\ Fortress\ 2)

cd "$tf2_dir"

command=" LD_LIBRARY_PATH=bin hl2_linux -game tf"
if [[ ! $1 == "-f" ]]; then
  command="$command|& grep oxide"
fi
eval $command

