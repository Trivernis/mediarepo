import shutil as shut
import os

build_output = 'out'


def check_exec(name: str):
    print('Checking {}...'.format(name))
    if shut.which(name) is None:
        raise Exception('{} not found'.format(name))
    exec(name + ' --version')


def check_yarn():
    print('Checking yarn...')
    if shut.which('yarn') is None:
        print('installing yarn...')
        exec('npm install -g yarn')
        check_exec('yarn')
    exec('yarn --version')


def check_ng():
    print('Checking ng...')
    if shut.which('ng') is None:
        print('installing ng...')
        exec('npm install -g @angular/cli')
        check_exec('ng')
    exec('ng --version')


def exec(cmd: str, dir: str = None) -> str:
    import subprocess
    child = subprocess.run(cmd, shell=True, cwd=dir)
    child.check_returncode()


def store_artifact(path: str):
    print('Storing {}'.format(path))
    if os.path.isdir(path):
        shut.copytree(path, os.path.join(build_output, os.path.basename(path)), dirs_exist_ok=True)
    else:
        shut.copy(path, build_output)


def cargo(cmd: str, dir: str = None) -> str:
    print('cargo {}'.format(cmd))
    exec('cargo {}'.format(cmd), dir)


def yarn(cmd: str, dir: str = None):
    print('yarn {}'.format(cmd))
    exec('yarn {}'.format(cmd), dir)


def build_daemon():
    cargo('fetch', 'mediarepo-daemon')
    cargo('build --release --frozen --verbose', 'mediarepo-daemon')

    if os.name == 'nt':
        store_artifact('mediarepo-daemon/target/release/mediarepo-daemon.exe')
    else:
        store_artifact('mediarepo-daemon/target/release/mediarepo-daemon')


def build_ui():
    cargo('install tauri-cli --version ^1.0.0-rc.4')
    yarn('install', 'mediarepo-ui')
    cargo('tauri build --verbose', 'mediarepo-ui')

    if os.name == 'nt':
        store_artifact('mediarepo-ui/src-tauri/target/release/mediarepo-ui.exe')
    else:
        store_artifact('mediarepo-ui/src-tauri/target/release/mediarepo-ui')

    store_artifact('mediarepo-ui/src-tauri/target/release/bundle/')


def check_daemon():
    check_exec('clang')
    check_exec('cargo')


def check_ui():
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
    check_daemon()
    check_ui()
    print('All checks passed')


def create_output_dir():
    if not os.path.exists(build_output):
        os.mkdir(build_output)


def clean():
    if os.path.exists(build_output):
        shut.rmtree(build_output)
    print('Cleaned')


def build(daemon=True, ui=True):
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
    build_parser.add_argument('--daemon', action='store_true', help='Build daemon')
    build_parser.add_argument('--ui', action='store_true', help='Build UI')
    
    subparsers.add_parser('clean')
    args = parser.parse_args()
    return args


def main():
    opts = parse_args()
    print(opts)

    if opts.command == 'build':
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