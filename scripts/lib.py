import subprocess
import shutil as shut


def install_tauri_cli(version: str):
    cargo('install tauri-cli --version ^{}'.format(version))


def check_ng(install: bool = False):
    '''Checks if ng is available and installs it
    if the install flag is set'''
    if not check_exec('ng'):
        if install:
            npm('install -g @angular/cli')
        else:
            raise Exception('ng not found')


def check_yarn(install: bool = False):
    '''Checks if yarn is available and installs it
    if the install flag is set'''
    if not check_exec('yarn'):
        if install:
            npm('install yarn')
        else:
            raise Exception('yarn not found')


def yarn(cmd: str, dir: str = None) -> str:
    '''Executes yarn in a given directory'''
    exec('yarn {}'.format(cmd), dir)


def cargo(cmd: str, dir: str = None):
    '''Executes cargo in a given directory'''
    exec('cargo {}'.format(cmd), dir)


def npm(cmd: str, dir: str = None) -> str:
    '''Executes npm in a given directory'''
    exec('npm {}'.format(cmd), dir)


def check_exec(name: str) -> bool:
    '''Checks if a command is available'''
    if shut.which(name) is None:
        print('{} not found'.format(name))
        return False
    exec('{} --version'.format(name))
    return True


def exec(cmd: str, dir: str = None) -> str:
    '''Executes a command in a given directory'''
    print('Running: {}'.format(cmd))
    child = subprocess.run(cmd, shell=True, cwd=dir)
    child.check_returncode()