#!/bin/python3.11

import argparse
from subprocess import run
from pathlib import Path
from time import sleep
import os


TF2_DIR = Path.home() / '.local'/'share'/'Steam' / \
    'steamapps'/'common'/'Team Fortress 2'


def inject(pid, lib):

    command = ['sudo', 'gdb', '-n', '-q', '-batch',
               '-ex', 'attach ' + pid,
               '-ex', 'set $dlopen = (void* (*)(char*, int))dlopen',
               '-ex', 'set $dlerror = (char* (*)(void))dlerror',
               '-ex', 'call $dlopen("' + lib.__str__() + '", 2)',
               '-ex', 'call $dlerror()',
               '-ex', 'detach',
               '-ex', 'quit']

    result = run(command).returncode

    if not result == 0:
        print("failed to inject")
        exit(result)


def unload(pid, lib):

    command = ['sudo', 'gdb', '-n', '-q', '-batch',
               '-ex', 'attach ' + pid,
               '-ex', 'set $dlopen = (void* (*)(char*, int))dlopen',
               '-ex', 'set $dlclose = (int (*)(void*))dlclose',
               '-ex', 'set $dlerror = (char* (*)(void))dlerror',
               '-ex', 'set $self = $dlopen("'+lib.__str__()+'", 6)',
               '-ex', 'call $dlerror()',
               '-ex', 'call $dlclose($self)',
               '-ex', 'call $dlerror()',
               '-ex', 'call $dlclose($self)',
               '-ex', 'call $dlerror()',
               '-ex', 'detach',
               '-ex', 'quit']

    result = run(command).returncode

    print(result)
    if not result == 0:
        print("failed to unload")
        exit(result)


def get_pid():
    pid = run(
        ['pidof', 'hl2_linux'], capture_output=True).stdout.decode('utf-8')\
        .strip()

    if pid == '':
        print('tf2 not runnig')
        exit(0)
    return pid


def get_lib(debug=False):

    lib = Path(os.path.dirname(os.path.realpath(__file__))) / 'target' \
        / 'i686-unknown-linux-gnu' / ('debug' if debug else 'release') \
        / 'liboxide.so'
    if not lib.exists():
        build(debug)
    return lib


def build(dev=False):
    command = ["cargo", "build"]
    if not dev:
        command.append("-r")

    result = run(command).returncode
    if result != 0:
        print("failed to build oxide")
        exit(0)


def start_tf2():
    run(["bash", "./hl2.sh", "-game", "tf"], cwd=TF2_DIR,
        env={**os.environ, "RUST_BACKTRACE": "FULL", "LD_LIBRARY_PATH": "bin"})


parser = argparse.ArgumentParser(
    prog='oxide toolbox')

parser.add_argument('action', choices=[
                    'inject', 'unload', 'build', 'start_tf2', 'reload'], default=inject)
parser.add_argument(
    '-d', '--debug', help='build for debug ', action='store_true')
args = parser.parse_args()

print(args)
match args.action:
    case 'inject':
        pid = get_pid()
        lib = get_lib(args.debug)

        inject(pid, lib, )
    case 'unload':
        pid = get_pid()
        lib = get_lib(args.debug)

        unload(pid, lib)
    case 'reload':
        pid = get_pid()
        lib = get_lib(args.debug)

        unload(pid, lib)
        sleep(2)
        inject(pid, lib)
    case 'build':
        build(args.debug)
    case 'start_tf2':
        start_tf2()
