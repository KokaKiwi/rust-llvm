from bindgen.ast import *

root = Namespace()
llvm = root.Namespace('llvm')

CallingConv = llvm.Namespace('CallingConv')

# Config link names
LLVM_CONFIG = 'llvm-config'
STATIC_LINK = False

def llvm_config(*args):
    import subprocess

    cmd = [LLVM_CONFIG] + list(args)
    out = subprocess.check_output(cmd)
    output = str(out, 'utf-8')

    return output.strip()

def rust_linknames():
    version = llvm_config('--version')

    args = ['--libs']
    if version >= '3.5':
        args.append('--system-libs')

    linknames = []

    flags = llvm_config(*args).replace('\n', ' ').split(' ')
    for flag in flags:
        if flag[:2] != '-l':
            continue

        libname = flag[2:]
        linkname = libname

        if 'LLVM' in libname and STATIC_LINK:
            linkname += ':static'

        linknames.append(linkname)

    flags = llvm_config('--cxxflags')
    if STATIC_LINK:
        assert 'stdlib=libc++' not in flags
        linknames.append('stdc++:static')
    else:
        if 'stdlib=libc++' in flags:
            linknames.append('c++')
        else:
            linknames.append('stdc++')

    return linknames

root.attrs['rust-linknames'] = rust_linknames()
