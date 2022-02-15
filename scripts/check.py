#!/bin/env python3
from lib import *
import argparse
import os


tauri_cli_version = '1.0.0-rc.5'
windows = os.name == 'nt'


def main():
    opts = parse_args()
    check(opts.install_deps)


def parse_args():
    '''Parses command line arguments'''
    args = argparse.ArgumentParser(description='Build mediarepo')
    args.add_argument('--install-deps', action='store_true',
                      help='Install dependencies that can be installed automatically')
    return args.parse_args()


def check(install_deps: bool = False):
    '''Checks dependencies'''
    check_daemon_depends()
    check_ui_depends(install_deps)
    print('All checks passed')


def check_daemon_depends():
    '''Checks dependencies for daemon'''
    check_exec('clang')
    check_exec('cargo')


def check_ui_depends(install_deps: bool = False):
    '''Checks dependencies for UI'''

    if not windows:
        check_exec('wget')
        check_exec('curl')
        check_exec('file')

    check_exec('clang')
    check_exec('cargo')
    check_exec('node')
    check_exec('npm')
    check_yarn(install_deps)
    check_ng(install_deps)

    if install_deps:
        install_tauri_cli(tauri_cli_version)


if __name__ == '__main__':
    main()
