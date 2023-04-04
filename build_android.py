#!/usr/bin/env python3

"""
Make sure to change the configuration section according to your needs.

Many thanks to https://julhe.github.io/posts/building-an-android-app-with
-rust-and-sdl2/
"""

from os import makedirs, mkdir, environ, pathsep, chdir, getcwd
from os.path import exists, expanduser
from shutil import copytree, rmtree, copy
from subprocess import check_call

# Configuration section
ANDROID_PROJECT_NAME = 'android_project'
NDK_PATH = '~/Android/Sdk/ndk/25.2.9519653'
RUST_SDL2_REPO = 'rust-sdl2-2.0.18'
LIB_NAME = 'libmain.so'
SDL2_LIBS_PATH = '../SDL/libs'  # could be removed or changed to SDL src folder
BUILD_MODE = 'release'


def comment_line(file, line_to_comment):
    found = False
    with open(file, 'r') as f:
        contents = f.read()

    with open(file, 'w') as f:
        for line in contents.splitlines():
            if line == line_to_comment:
                line = f'# {line}'
                found = True
            f.writelines(line)
    if not found:
        raise LookupError(
            f'Could not find line "{line_to_comment}" in file {file}')


def process_section_in_gradle(file, section, comment_action):
    # TODO
    pass


def check_path_exists(path, name):
    if not exists(path):
        raise FileNotFoundError(
            f'{name} not found at: {path}')


def build_for_android():
    original_cwd = getcwd()
    rust_sdl2_repo = f'../{RUST_SDL2_REPO}'
    check_path_exists(rust_sdl2_repo, 'Rust-SDL2 repo')

    if exists(f'../{ANDROID_PROJECT_NAME}'):
        ndk_path = expanduser(NDK_PATH)
        check_path_exists(ndk_path, 'NDK path')
        environ['PATH'] += pathsep + ndk_path
        copytree(f'../{RUST_SDL2_REPO}/sdl2-sys/SDL', '..')
        chdir('../SDL')
        check_call('ndk-build NDK_PROJECT_PATH=. APP_BUILD_SCRIPT=./Android.mk '
                   'APP_PLATFORM=android-19', shell=True)
        copytree('android-project', f'../{ANDROID_PROJECT_NAME}')
        chdir('..')
        check_call(f'ln -s `pwd`/SDL {ANDROID_PROJECT_NAME}/app/jni/',
                   shell=True)
        process_section_in_gradle(
            f'{ANDROID_PROJECT_NAME}/app/build.gradle', 'ndkBuild', True)
        process_section_in_gradle(
            f'{ANDROID_PROJECT_NAME}/app/build.gradle', 'cmake', False)
        comment_line(f'{ANDROID_PROJECT_NAME}/app/jni/CMakeLists.txt',
                     'add_subdirectory(src)')
        chdir(original_cwd)

    # Copy sdl2 libs into rust's build dir
    makedirs('target/aarch64-linux-android/$BUILD_MODE/deps/', exist_ok=True)
    makedirs('target/armv7-linux-androideabi/$BUILD_MODE/deps/', exist_ok=True)
    makedirs('target/i686-linux-android/$BUILD_MODE/deps/', exist_ok=True)
    copytree(f'{SDL2_LIBS_PATH}/arm64-v8a',
             'target/aarch64-linux-android/$BUILD_MODE/deps/',
             dirs_exist_ok=True)
    copytree(f'{SDL2_LIBS_PATH}/armeabi-v7a',
             'target/armv7-linux-androideabi/$BUILD_MODE/deps/',
             dirs_exist_ok=True)
    copytree(f'{SDL2_LIBS_PATH}/x86',
             'target/i686-linux-android/$BUILD_MODE/deps/',
             dirs_exist_ok=True)

    # Build the libraries
    build_mode = '' if BUILD_MODE == 'debug' else f'--{BUILD_MODE}'
    check_call(f'cargo build --target aarch64-linux-android {build_mode}',
               shell=True)
    check_call(f'cargo build --target armv7-linux-androideabi {build_mode}',
               shell=True)
    check_call(f'cargo build --target i686-linux-android {build_mode}',
               shell=True)

    # Prepare folders...
    jni_libs_path = f'../{ANDROID_PROJECT_NAME}/app/src/main/jniLibs'
    rmtree(f'{jni_libs_path}')
    mkdir(f'{jni_libs_path}')
    mkdir(f'{jni_libs_path}/arm64-v8a')
    mkdir(f'{jni_libs_path}/armeabi-v7a')
    mkdir(f'{jni_libs_path}/x86')

    # And copy the rust library into the android studio project, ready for
    # being included into the APK
    copy(f'target/aarch64-linux-android/{BUILD_MODE}/{LIB_NAME}',
         f'{jni_libs_path}/arm64-v8a/libmain.so')
    copy(f'target/armv7-linux-androideabi/{BUILD_MODE}/{LIB_NAME}',
         f'{jni_libs_path}/armeabi-v7a/libmain.so')
    copy(f'target/i686-linux-android/{BUILD_MODE}/{LIB_NAME}',
         f'{jni_libs_path}/x86/libmain.so')

    # Copy assets
    assets_path = f'../{ANDROID_PROJECT_NAME}/app/src/main/assets'
    if exists(assets_path):
        rmtree(assets_path)
    mkdir(assets_path)
    copytree('assets', assets_path)


if __name__ == '__main__':
    build_for_android()
