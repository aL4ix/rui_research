#!/usr/bin/env python3

"""
Make sure to change the configuration section according to your needs.

Dependencies: java, android sdk, cmake, rust

Many thanks to https://julhe.github.io/posts/building-an-android-app-with
-rust-and-sdl2/
"""
from contextlib import suppress
from os import makedirs, mkdir, environ, pathsep, chdir, getcwd
from os.path import exists, expanduser
from shutil import copytree, rmtree, copy
from subprocess import check_call

# Configuration section
ANDROID_PROJECT_NAME = 'android_project'
ANDROID_SDK_PATH = '~/Android/Sdk'
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
            f.write(f'{line}\n')
    if not found:
        raise LookupError(
            f'Could not find line "{line_to_comment}" in file {file}')


def process_section_in_gradle(file, section, comment_action):
    comment_stem = '//'

    with open(file, 'r') as f:
        contents = f.read()

    change = None
    with open(file, 'w') as f:
        for line in contents.splitlines():
            stripped = line.strip()
            if comment_action:
                if stripped.startswith(section):
                    change = True
            else:
                if stripped.startswith(comment_stem):
                    if stripped[2:].strip().startswith(section):
                        change = False
            if change:
                f.write(f'{comment_stem} {line}\n')
            elif change is None:  # This case should come first otherwise not
                # change will capture these both
                f.write(f'{line}\n')
            elif not change:
                line = line.replace(comment_stem, '', 1)
                f.write(f'{line}\n')

            if stripped.startswith('}'):  # End of section
                change = None


def check_path_exists(path, name):
    if not exists(path):
        raise FileNotFoundError(
            f'{name} not found at: {path}')


def generate_cargo_config(ndk_abs_path, config_path):
    template = f"""
[target.aarch64-linux-android]
ar = "{ndk_abs_path}/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android-ar"
linker ="{ndk_abs_path}/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android26-clang"

[target.armv7-linux-androideabi]
ar = "{ndk_abs_path}/toolchains/llvm/prebuilt/linux-x86_64/bin/arm-linux-androideabi-ar"
linker = "{ndk_abs_path}/toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi26-clang"

[target.i686-linux-android]
ar = "{ndk_abs_path}/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android-ar"
linker = "{ndk_abs_path}/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android26-clang"
    """
    with open(config_path, 'w') as f:
        f.write(template)


def build_android():
    original_cwd = getcwd()
    check_path_exists(f'../{RUST_SDL2_REPO}', 'Rust-SDL2 repo')
    android_sdk_abs_path = expanduser(ANDROID_SDK_PATH)
    check_path_exists(android_sdk_abs_path, 'Android SDK')

    if not exists(f'../{ANDROID_PROJECT_NAME}'):
        ndk_abs_path = expanduser(NDK_PATH)
        check_path_exists(ndk_abs_path, 'NDK path')
        generate_cargo_config(ndk_abs_path, expanduser('~/.cargo/config'))
        environ['PATH'] += pathsep + ndk_abs_path
        chdir(f'../{RUST_SDL2_REPO}/sdl2-sys')
        check_call('cargo build', shell=True)  # In fact, we only need this so
        # it auto downloads the SDL source code
        chdir(original_cwd)
        copytree(f'../{RUST_SDL2_REPO}/sdl2-sys/SDL', '../SDL')
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
        with open(f'{ANDROID_PROJECT_NAME}/local.properties', 'w') as f:
            f.write(f'sdk.dir={android_sdk_abs_path}')
        chdir(original_cwd)

    # Copy sdl2 libs into rust's build dir
    makedirs(f'target/aarch64-linux-android/{BUILD_MODE}/deps/', exist_ok=True)
    makedirs(f'target/armv7-linux-androideabi/{BUILD_MODE}/deps/',
             exist_ok=True)
    makedirs(f'target/i686-linux-android/{BUILD_MODE}/deps/', exist_ok=True)
    copytree(f'{SDL2_LIBS_PATH}/arm64-v8a',
             f'target/aarch64-linux-android/{BUILD_MODE}/deps/',
             dirs_exist_ok=True)
    copytree(f'{SDL2_LIBS_PATH}/armeabi-v7a',
             f'target/armv7-linux-androideabi/{BUILD_MODE}/deps/',
             dirs_exist_ok=True)
    copytree(f'{SDL2_LIBS_PATH}/x86',
             f'target/i686-linux-android/{BUILD_MODE}/deps/',
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
    with suppress(FileNotFoundError):
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
    copytree('assets', f'{assets_path}/assets')

    # Assemble debug
    chdir(fr'../{ANDROID_PROJECT_NAME}')
    check_call('./gradlew assembleDebug', shell=True)
    # Why debug, because release needs a certificate
    cwd = getcwd()
    print(f'CONGRATS! Debug APK generated at {cwd}/app/build/outputs/apk/debug')


if __name__ == '__main__':
    build_android()
