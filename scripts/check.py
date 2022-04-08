#!/bin/env python3
from lib import *
import argparse
import os


tauri_cli_version = '1.0.0-rc.8'
windows = os.name == 'nt'


def main():
    opts = parse_args()
    check(opts.install)


def parse_args():
    '''Parses command line arguments'''
    args = argparse.ArgumentParser(description='Build mediarepo')
    args.add_argument('--install', action='store_true',
                      help='Install tools that can be installed automatically')
    return args.parse_args()


def check(install_tooling: bool = False):
    '''Checks dependencies'''
    check_daemon_tooling()
    check_ui_tooling(install_tooling)
    print('All checks passed')


def check_daemon_tooling():
    '''Checks dependencies for daemon'''
    check_exec('clang')
    check_exec('cargo')


def check_ui_tooling(install_tooling: bool = False):
    '''Checks dependencies for UI'''

    if not windows:
        check_exec('wget')
        check_exec('curl')
        check_exec('file')

    check_exec('clang')
    check_exec('cargo')
    check_exec('node')
    check_exec('npm')
    check_yarn(install_tooling)
    check_ng(install_tooling)

    if install_tooling:
        install_tauri_cli(tauri_cli_version)


if __name__ == '__main__':
    main()
