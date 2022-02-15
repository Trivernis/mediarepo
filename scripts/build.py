#!/bin/env python3
import shutil as shut
import os
from lib import *
import json
from clean import clean
from check import check_daemon_depends, check_ui_depends


build_output = 'out'
verbose = False
ffmpeg = False
install_deps = False

windows = os.name == 'nt'


def main():
    opts = parse_args()

    global install_deps
    global build_output
    global verbose
    global ffmpeg
    global install_deps

    build_output = opts.output if opts.output else build_output
    verbose = opts.verbose
    ffmpeg = opts.ffmpeg
    install_deps = opts.install_deps

    build(opts.component)


def parse_args():
    import argparse
    parser = argparse.ArgumentParser(description='Build mediarepo')
    parser.add_argument(
        'component', type=str, nargs='?', default='all', choices=['daemon', 'ui', 'all'])
    parser.add_argument(
        '--verbose', action='store_true', help='Verbose build')
    parser.add_argument('--install-deps', action='store_true',
                        help='Install dependencies')
    parser.add_argument(
        '--output', action='store', help='Build output directory')
    parser.add_argument(
        '--ffmpeg', action='store_true', help='Build with ffmpeg')

    args = parser.parse_args()
    return args


def build(component: str):
    '''Builds the selected component'''
    clean()
    create_output_dir()

    if component == 'daemon' or component == 'all':
        check_daemon_depends()
        build_daemon()
    elif component == 'ui' or component == 'all':
        check_ui_depends(install_deps)
        build_ui()
    else:
        raise Exception('Unknown component: {}'.format(component))

    print('Build complete')


def build_daemon():
    '''Builds daemon'''
    cargo('fetch', 'mediarepo-daemon')

    if not ffmpeg:
        cargo('build --release --frozen --no-default-features', 'mediarepo-daemon')
    else:
        cargo('build --release --frozen', 'mediarepo-daemon')

    if windows:
        store_artifact('mediarepo-daemon/target/release/mediarepo-daemon.exe')
    else:
        store_artifact('mediarepo-daemon/target/release/mediarepo-daemon')


def build_ui():
    '''Builds UI'''
    yarn('install', 'mediarepo-ui')
    cargo('tauri build', 'mediarepo-ui')

    if windows:
        store_artifact(
            'mediarepo-ui/src-tauri/target/release/mediarepo-ui.exe')
    else:
        store_artifact('mediarepo-ui/src-tauri/target/release/mediarepo-ui')

    store_artifact('mediarepo-ui/src-tauri/target/release/bundle/')


def create_output_dir():
    '''Creates build output directory'''
    if not os.path.exists(build_output):
        os.mkdir(build_output)


def store_artifact(path: str):
    '''Stores a build artifact'''
    print('Storing {}'.format(path))
    if os.path.isdir(path):
        shut.copytree(path, os.path.join(
            build_output, os.path.basename(path)), dirs_exist_ok=True)
    else:
        shut.copy(path, build_output)


if __name__ == '__main__':
    main()
