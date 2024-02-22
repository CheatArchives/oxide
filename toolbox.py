#!/bin/python3.11

import argparse
from subprocess import run
from pathlib import Path
from datetime import datetime
from time import sleep
import os


LOCK_FILE = Path("/tmp") / "oxide.lock"


def inject(pid, lib):
    if LOCK_FILE.exists():
        print('lock file exists reloading')
        unload(pid, lib)
        sleep(2)

    command = ['sudo', 'gdb', '-n', '-q', '-batch',
               '-ex', 'attach ' + pid,
               '-ex', 'set $dlopen = (void* (*)(char*, int))dlopen',
               '-ex', 'set $dlerror = (char* (*)(void))dlerror',
               '-ex', 'call $dlopen("' + lib.__str__() + '", 2)',
               '-ex', 'call $dlerror()',
               '-ex', 'detach',
               '-ex', 'quit']

    result = run(command).returncode

    if result == 0:
        with open(LOCK_FILE, "w") as f:
            f.write(datetime.now().__str__())
    else:
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
    if result == 0 and LOCK_FILE.exists():
        os.remove(LOCK_FILE)
    else:
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

    lib = Path.cwd()/'target'/'i686-unknown-linux-gnu' / \
        ('debug' if debug else 'release') / 'liboxide.so'
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


parser = argparse.ArgumentParser(
    prog='oxide toolbox')

parser.add_argument('action', choices=[
                    'inject', 'unload', 'build'], default=inject)
parser.add_argument('-r', '--release', action='store_true')
args = parser.parse_args()

print(args)
match args.action:
    case 'inject':
        pid = get_pid()
        lib = get_lib(not args.release)

        inject(pid, lib, )
    case 'unload':

        pid = get_pid()
        lib = get_lib(not args.release)

        unload(pid, lib)
    case 'build':

        build(not args.release)
