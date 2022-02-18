#!/bin/env python3
import os
import shutil as shut
import argparse


def main():
    opts = parse_args()
    clean(opts.output if opts.output else 'out')


def parse_args():
    '''Parses command line arguments'''
    args = argparse.ArgumentParser(description='Build mediarepo')
    args.add_argument('--output', action='store', help='Build output directory')
    return args.parse_args()


def clean(build_output: str = 'out'):
    '''Removes build output'''
    if os.path.exists(build_output):
        shut.rmtree(build_output)
    print('Cleaned')


if __name__ == '__main__':
    main()