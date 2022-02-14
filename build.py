#!/bin/env python3
import shutil as shut
import os
import subprocess

tauri_cli_version = '1.0.0-rc.5'
build_output = 'out'
verbose = False

windows = os.name == 'nt'


def exec(cmd: str, dir: str = None) -> str:
    print('Running: {}'.format(cmd))
    child = subprocess.run(cmd, shell=True, cwd=dir)
    child.check_returncode()


def check_exec(name: str):
    print('Checking {}...'.format(name))

    if shut.which(name) is None:
        raise Exception('{} not found'.format(name))
    exec(name + ' --version')


def check_yarn():
    print('Checking yarn...')

    if shut.which('yarn') is None:
        print('installing yarn...')
        npm('install -g yarn')
        check_exec('yarn')
    exec('yarn --version')


def check_ng():
    print('Checking ng...')

    if shut.which('ng') is None:
        print('installing ng...')
        npm('install -g @angular/cli')
        check_exec('ng')
    exec('ng --version')


def store_artifact(path: str):
    print('Storing {}'.format(path))
    if os.path.isdir(path):
        shut.copytree(path, os.path.join(
            build_output, os.path.basename(path)), dirs_exist_ok=True)
    else:
        shut.copy(path, build_output)


def cargo(cmd: str, dir: str = None):
    if verbose:
        exec('cargo {} --verbose'.format(cmd), dir)
    else:
        exec('cargo {}'.format(cmd), dir)


def npm(cmd: str, dir: str = None):
    exec('npm {}'.format(cmd), dir)


def yarn(cmd: str, dir: str = None):
    exec('yarn {}'.format(cmd), dir)


def build_daemon():
    '''Builds daemon'''
    cargo('fetch', 'mediarepo-daemon')
    cargo('build --release --frozen', 'mediarepo-daemon')

    if windows:
        store_artifact('mediarepo-daemon/target/release/mediarepo-daemon.exe')
    else:
        store_artifact('mediarepo-daemon/target/release/mediarepo-daemon')


def build_ui():
    '''Builds UI'''
    cargo('install tauri-cli --version ^{}'.format(tauri_cli_version))
    yarn('install', 'mediarepo-ui')
    cargo('tauri build', 'mediarepo-ui')

    if windows:
        store_artifact(
            'mediarepo-ui/src-tauri/target/release/mediarepo-ui.exe')
    else:
        store_artifact('mediarepo-ui/src-tauri/target/release/mediarepo-ui')

    store_artifact('mediarepo-ui/src-tauri/target/release/bundle/')


def check_daemon():
    '''Checks dependencies for daemon'''
    check_exec('clang')
    check_exec('cargo')


def check_ui():
    '''Checks dependencies for UI'''
    check_exec('clang')
    check_exec('cargo')
    check_exec('wget')
    check_exec('curl')
    check_exec('file')
    check_exec('node')
    check_exec('npm')
    check_yarn()
    check_ng()


def check():
    '''Checks dependencies'''
    check_daemon()
    check_ui()
    print('All checks passed')


def create_output_dir():
    '''Creates build output directory'''
    if not os.path.exists(build_output):
        os.mkdir(build_output)


def clean():
    '''Removes build output'''
    if os.path.exists(build_output):
        shut.rmtree(build_output)
    print('Cleaned')


def build(daemon=True, ui=True):
    '''Builds both daemon and UI'''
    clean()
    create_output_dir()

    if daemon:
        check_daemon()
        build_daemon()

    if ui:
        check_ui()
        build_ui()

    print('Build complete')


def parse_args():
    import argparse
    parser = argparse.ArgumentParser(description='Build mediarepo')
    subparsers = parser.add_subparsers(dest='command')
    subparsers.required = True

    subparsers.add_parser('check')

    build_parser = subparsers.add_parser('build')
    build_parser.add_argument(
        '--daemon', action='store_true', help='Build daemon')
    build_parser.add_argument('--ui', action='store_true', help='Build UI')
    build_parser.add_argument(
        '--verbose', action='store_true', help='Verbose build')
    build_parser.add_argument(
        '--output', action='store', help='Build output directory')

    subparsers.add_parser('clean')
    args = parser.parse_args()
    return args


def main():
    opts = parse_args()

    if opts.command == 'build':
        global build_output
        build_output = opts.output if opts.output else build_output

        global verbose
        verbose = opts.verbose

        if opts.daemon:
            build(True, False)
        elif opts.ui:
            build(False, True)
        else:
            build()
    elif opts.command == 'check':
        check()
    elif opts.command == 'clean':
        clean()


if __name__ == '__main__':
    main()
