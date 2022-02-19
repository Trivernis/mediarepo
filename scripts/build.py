#!/bin/env python3
import shutil as shut
import os
from lib import *
from clean import clean
from check import check, check_daemon_tooling, check_ui_tooling
from typing import List


build_output = 'out'
verbose = False
install_tooling = False

windows = os.name == 'nt'


def main():
    opts = parse_args()

    global install_tooling
    global build_output
    global verbose
    global install_tooling

    build_output = opts.output if opts.output else build_output
    verbose = opts.verbose
    install_tooling = opts.install_tooling

    build(opts.component, opts.bundles)


def parse_args():
    import argparse
    parser = argparse.ArgumentParser(description='Build mediarepo')
    parser.add_argument(
        'component', type=str, nargs='?', default='all', choices=['daemon', 'ui', 'all'])
    parser.add_argument(
        '--verbose', action='store_true', help='Verbose build')
    parser.add_argument(
        '--output', action='store', help='Build output directory')
    parser.add_argument('--install-tooling',
                        action='store_true', help='Install tooling')
    parser.add_argument('--bundles', nargs='+',
                        help='UI bundles to build')

    args = parser.parse_args()
    return args


def build(component: str, bundles: List[str] = None):
    '''Builds the selected component'''
    clean()
    create_output_dir()

    if component == 'all':
        check(install_tooling)
        build_daemon()
        build_ui(bundles)
    elif component == 'daemon':
        check_daemon_tooling()
        build_daemon()
    elif component == 'ui':
        check_ui_tooling(install_tooling)
        build_ui(bundles)

    print('Build complete')


def build_daemon():
    '''Builds daemon'''
    cargo('fetch', 'mediarepo-daemon')
    cargo('build --release --frozen', 'mediarepo-daemon')

    if windows:
        store_artifact('mediarepo-daemon/target/release/mediarepo-daemon.exe')
    else:
        store_artifact('mediarepo-daemon/target/release/mediarepo-daemon')


def build_ui(bundles: List[str] = None):
    '''Builds UI'''
    yarn('install', 'mediarepo-ui')

    if bundles is not None:
        cargo('tauri build --bundles ' + ' '.join(bundles), 'mediarepo-ui')
    else:
        cargo('tauri build ', 'mediarepo-ui')

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
    elif os.path.isfile(path):
        shut.copy(path, build_output)


if __name__ == '__main__':
    main()
