<p align="center">
  <a href="https://wasmer.io" target="_blank" rel="noopener noreferrer">
    <img width="300" src="https://raw.githubusercontent.com/wasmerio/wasmer/master/logo.png" alt="Wasmer logo">
  </a>
</p>

<p align="center">
  <a href="https://dev.azure.com/wasmerio/wasmer/_build/latest?definitionId=3&branchName=master">
    <img src="https://img.shields.io/azure-devops/build/wasmerio/wasmer/3.svg?style=flat-square" alt="Build Status">
  </a>
  <a href="https://github.com/wasmerio/wasmer/blob/master/LICENSE">
    <img src="https://img.shields.io/github/license/wasmerio/wasmer.svg?style=flat-square" alt="License">
  </a>
  <a href="https://spectrum.chat/wasmer">
    <img src="https://withspectrum.github.io/badge/badge.svg" alt="Join the Wasmer Community">
  </a>
  <a href="https://twitter.com/wasmerio">
    <img alt="Follow @wasmerio on Twitter" src="https://img.shields.io/twitter/follow/wasmerio?label=%40wasmerio&style=flat-square">
  </a>
</p>

## Introduction

[Wasmer](https://wasmer.io/) is a standalone WebAssembly runtime, for running WebAssembly [outside of the Browser](https://webassembly.org/docs/non-web/) supporting [WASI](https://github.com/WebAssembly/WASI) and [Emscripten](https://emscripten.org/).

Wasmer can be used as a Command Line Application or as a library embedded in different languages.

Install the Wasmer CLI with:

```sh
curl https://get.wasmer.io -sSfL | sh
```

> Note: *Wasmer is also available on Windows. Download the [`WasmerInstaller.exe` from the Github Releases](https://github.com/wasmerio/wasmer/releases) page.*

### Languages

Wasmer runtime can be used as a library embedded in different languages, so you can use WebAssembly anywhere ✨:

| &nbsp; | Language | Social | Author(s) | [Maintenance](https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata) |
|-|-|-|-|-|
| ![Rust logo](./docs/assets/languages/rust.svg) | [**Rust**](https://github.com/wasmerio/wasmer-rust-example) | ![last release](https://img.shields.io/crates/v//wasmer-runtime?style=flat-square) ![number of Github stars](https://img.shields.io/github/stars/wasmerio/wasmer?style=flat-square) | Wasmer | actively developed |
| ![C logo](./docs/assets/languages/c.svg) | [**C/C++**](https://github.com/wasmerio/wasmer-c-api) | ![last release](https://img.shields.io/github/v/release/wasmerio/wasmer?style=flat-square) ![number of Github stars](https://img.shields.io/github/stars/wasmerio/wasmer?style=flat-square) | Wasmer | actively developed |
| ![Python logo](./docs/assets/languages/python.svg) | [**Python**](https://github.com/wasmerio/python-ext-wasm) | ![last release](https://img.shields.io/pypi/v/wasmer?style=flat-square) ![number of Github stars](https://img.shields.io/github/stars/wasmerio/python-ext-wasm?style=flat-square) | Wasmer | actively developed |
| ![Go logo](./docs/assets/languages/go.svg) | [**Go**](https://github.com/wasmerio/go-ext-wasm) | ![last release](https://img.shields.io/github/v/release/wasmerio/go-ext-wasm?style=flat-square) ![number of Github stars](https://img.shields.io/github/stars/wasmerio/go-ext-wasm?style=flat-square) | Wasmer | actively developed |
| ![PHP logo](./docs/assets/languages/php.svg) | [**PHP**](https://github.com/wasmerio/php-ext-wasm) | ![last release](https://img.shields.io/github/v/release/wasmerio/php-ext-wasm?style=flat-square) ![number of Github stars](https://img.shields.io/github/stars/wasmerio/php-ext-wasm?style=flat-square) | Wasmer | actively developed |
| ![Ruby logo](./docs/assets/languages/ruby.svg) | [**Ruby**](https://github.com/wasmerio/ruby-ext-wasm) | ![last release](https://img.shields.io/github/v/release/wasmerio/ruby-ext-wasm?style=flat-square) ![number of Github stars](https://img.shields.io/github/stars/wasmerio/ruby-ext-wasm?style=flat-square) | Wasmer | actively developed |
| ![Postgres logo](./docs/assets/languages/postgres.svg) | [**Postgres**](https://github.com/wasmerio/postgres-ext-wasm) | ![last release](https://img.shields.io/github/v/release/wasmerio/postgres-ext-wasm?style=flat-square) ![number of Github stars](https://img.shields.io/github/stars/wasmerio/postgres-ext-wasm?style=flat-square) | Wasmer | actively developed |
| ![C# logo](./docs/assets/languages/csharp.svg) | [**C#/.Net**](https://github.com/migueldeicaza/WasmerSharp) | ![last release](https://img.shields.io/github/v/release/migueldeicaza/WasmerSharp?style=flat-square) ![number of Github stars](https://img.shields.io/github/stars/migueldeicaza/WasmerSharp?style=flat-square) | [Miguel de Icaza](https://github.com/migueldeicaza) | actively developed | ![last release](https://img.shields.io/nuget/v/WasmerSharp?style=flat-square) | ![number of Github stars](https://img.shields.io/github/stars/migueldeicaza/WasmerSharp?style=flat-square) |
| ![R logo](./docs/assets/languages/r.svg) | [**R**](https://github.com/dirkschumacher/wasmr) | ![number of Github stars](https://img.shields.io/github/stars/dirkschumacher/wasmr?style=flat-square) | [Dirk Schumacher](https://github.com/dirkschumacher) | actively developed |
| ![Swift logo](./docs/assets/languages/swift.svg) | [**Swift**](https://github.com/markmals/swift-ext-wasm) | ![number of Github stars](https://img.shields.io/github/stars/markmals/swift-ext-wasm?style=flat-square) | [Mark Malström](https://github.com/markmals/) | passively maintened |
| ❓ | | [your language is missing?](https://github.com/wasmerio/wasmer/issues/new?assignees=&labels=%F0%9F%8E%89+enhancement&template=---feature-request.md&title=) | |

### Usage

Wasmer can execute both the standard binary format (`.wasm`) and the text
format defined by the WebAssembly reference interpreter (`.wat`).

Once installed, you will be able to run any WebAssembly files (_including Lua, PHP, SQLite and nginx!_):

```sh
# Run Lua
wasmer run examples/lua.wasm
```

*You can find more `wasm/wat` examples in the [examples](./examples) directory.*

#### With WAPM

Installing Wasmer through `wasmer.io` includes
[`wapm`](https://github.com/wasmerio/wapm-cli), the [WebAssembly Package Manager](https://wapm.io/).

Wapm allows you to easily download, run, and distribute WebAssembly binaries.

```sh
# Install cowsay globally
wapm install -g cowsay

# Run cowsay
wapm run cowsay "Hello, world!"
```

For more information about wapm, check out the [website](https://www.wapm.io)
and this [example program](https://github.com/wapm-packages/rust-wasi-example).

## Code Structure

Wasmer is structured into different directories:

- [`src`](./src): code related to the Wasmer executable itself
- [`lib`](./lib): modularized libraries that Wasmer uses under the hood
- [`examples`](./examples): some useful examples to getting started with Wasmer

## Dependencies

Building Wasmer requires [rustup](https://rustup.rs/).

To build Wasmer on Windows, download and run [`rustup-init.exe`](https://win.rustup.rs/)
then follow the onscreen instructions.

To build on other systems, run:

```sh
curl https://sh.rustup.rs -sSf | sh
```

### Other dependencies

Please select your operating system:

<details>
  <summary><b>macOS</b></summary>
  <p>

#### macOS

If you have [Homebrew](https://brew.sh/) installed:

```sh
brew install cmake
```

Or, in case you have [MacPorts](https://www.macports.org/install.php):

```sh
sudo port install cmake
```

  </p>
</details>

<details>
  <summary><b>Debian-based Linuxes</b></summary>
  <p>

#### Debian-based Linuxes

```sh
sudo apt install cmake pkg-config libssl-dev
```
  </p>
</details>

<details>
  <summary><b>FreeBSD</b></summary>
  <p>

#### FreeBSD

```sh
pkg install cmake
```
  </p>
</details>

<details>
  <summary><b>Windows</b></summary>
  <p>

#### Windows (MSVC)

Windows support is _experimental_. WASI is fully supported, but Emscripten support is on the works (this means
nginx and Lua do not work on Windows - you can track the progress on [this issue](https://github.com/wasmerio/wasmer/issues/176)).

1. Install [Visual Studio](https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=Community&rel=15)

2. Install [Rust for Windows](https://win.rustup.rs)

3. Install [Python for Windows](https://www.python.org/downloads/release/python-2714/). The Windows x86-64 MSI installer is fine.
   Make sure to enable "Add python.exe to Path" during installation.

4. Install [Git for Windows](https://git-scm.com/download/win). Allow it to add `git.exe` to your PATH (default
   settings for the installer are fine).

5. Install [CMake](https://cmake.org/download/). Ensure CMake is in your PATH.

6. Install [LLVM 8.0](https://prereleases.llvm.org/win-snapshots/LLVM-8.0.0-r351033-win64.exe)
  </p>
</details>

## Building
[![Rustc Version 1.36+](https://img.shields.io/badge/rustc-1.36+-red.svg)](https://blog.rust-lang.org/2019/07/04/Rust-1.36.0.html)

Wasmer is built with [Cargo](https://crates.io/), the Rust package manager.

The Singlepass backend requires nightly, so if you want to use it,

Set Rust Nightly:
```
rustup default nightly
```

Otherwise an up to date (see badge above) verison of stable Rust will work.

And install Wasmer
```sh
# checkout code
git clone https://github.com/wasmerio/wasmer.git
cd wasmer

# install tools
make release-clif # To build with cranelift (default)

make release-llvm # To build with llvm support

make release-singlepass # To build with singlepass support

# or
make release # To build with singlepass, cranelift and llvm support
```

## Testing

Thanks to [spec tests](https://github.com/wasmerio/wasmer/tree/master/lib/spectests/spectests) we can ensure 100% compatibility with the WebAssembly spec test suite.

You can run all the tests with:

```sh
rustup default nightly
make test
```

### Testing backends

Each backend can be tested separately:

* Singlepass: `make singlepass`
* Cranelift: `make cranelift`
* LLVM: `make llvm`

### Testing integrations

Each integration can be tested separately:

* Spec tests: `make spectests`
* Emscripten: `make emtests`
* WASI: `make wasi`
* Middleware: `make middleware`
* C API: `make capi`


## Benchmarking

Benchmarks can be run with:

```sh
make bench-[backend]

# for example
make bench-singlepass
```

## Roadmap

Wasmer is an open project guided by strong principles, aiming to be modular, flexible and fast. It is open to the community to help set its direction.

Below are some of the goals of this project (in order of priority):

- [x] It should be 100% compatible with the [WebAssembly spec tests](https://github.com/wasmerio/wasmer/tree/master/lib/spectests/spectests)
- [x] It should be fast _(partially achieved)_
- [x] Support WASI - released in [0.3.0](https://github.com/wasmerio/wasmer/releases/tag/0.3.0)
- [x] Support Emscripten calls _(in the works)_
- [ ] Support Go js ABI calls

## Architecture

If you would like to know how Wasmer works under the hood, please see [docs/architecture.md](./docs/architecture.md).

## License

Wasmer is primarily distributed under the terms of the [MIT license](http://opensource.org/licenses/MIT) ([LICENSE](./LICENSE)).

[ATTRIBUTIONS](./ATTRIBUTIONS.md)
