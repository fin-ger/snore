<h1 align="center">Welcome to snore 😴</h1>
<p align="center">
  <a href="https://github.com/fin-ger/snore/actions?query=workflow%3A%22rust+stable+build%22">
    <img src="https://github.com/fin-ger/snore/workflows/rust stable build/badge.svg" alt="rust stable build">
  </a>
  <a href="https://github.com/fin-ger/snore/actions?query=workflow%3A%22rust+nightly+build%22">
    <img src="https://github.com/fin-ger/snore/workflows/rust nightly build/badge.svg" alt="rust nightly build">
  </a>
  <a href="https://github.com/fin-ger/snore/blob/master/LICENSE">
    <img alt="GitHub" src="https://img.shields.io/github/license/fin-ger/snore">
  </a>
  <a href="http://spacemacs.org">
    <img src="https://cdn.rawgit.com/syl20bnr/spacemacs/442d025779da2f62fc86c2082703697714db6514/assets/spacemacs-badge.svg" />
  </a>
  <a href="http://makeapullrequest.com">
    <img alt="PRs Welcome" src="https://img.shields.io/badge/PRs-welcome-brightgreen.svg">
  </a>
  <br>
  <i>Runs for the given time and is similar to the Unix <code>sleep</code> command</i>
</p>

---

This program is meant to explore cross-platform distribution over many operating systems and package managers. The program provides a cross-platform interface for delaying the execution of a script for a given amount of time.

## Installation

### Statically Linked Binary for Linux

<a href="https://github.com/fin-ger/snore/releases/latest/download/snore-linux-amd64">
  <img src="https://img.shields.io/badge/download-linux%20amd64-blue?style=for-the-badge&logo=linux" alt="Download">
</a>
<a href="https://github.com/fin-ger/snore/releases/latest/download/snore-linux-aarch64">
  <img src="https://img.shields.io/badge/download-linux%20aarch64-blue?style=for-the-badge&logo=linux" alt="Download">
</a>
<a href="https://github.com/fin-ger/snore/releases/latest/download/snore-linux-arm">
  <img src="https://img.shields.io/badge/download-linux%20arm-blue?style=for-the-badge&logo=linux" alt="Download">
</a>
<a href="https://github.com/fin-ger/snore/releases/latest/download/snore-linux-armhf">
  <img src="https://img.shields.io/badge/download-linux%20armhf-blue?style=for-the-badge&logo=linux" alt="Download">
</a>

Download the `snore` binary suitable for your CPU architecture. If unsure, use **amd64**. After downloading, rename the executable to `snore`:

```shell
$ mv snore-linux-amd64 snore
```

Before you can run the executable, you have to set the executable flag:

```shell
$ chmod +x /path/to/snore
```

You can now run the program with:

```shell
$ /path/to/snore
```

> Move the executable into your [`PATH`](https://askubuntu.com/questions/109381/how-to-add-path-of-a-program-to-path-environment-variable) to use it like any other program installed on the system

### MacOS Binary

<a href="https://github.com/fin-ger/snore/releases/latest/download/snore-macos">
  <img src="https://img.shields.io/badge/download-macos-blue?style=for-the-badge&logo=apple" alt="Download">
</a>

Download the `snore` binary and rename it to `snore`:

```shell
$ mv snore-macos snore
```

Before you can run the executable, you have to set the executable flag:

```shell
$ chmod +x /path/to/snore
```

You can now run the program with:

```shell
$ /path/to/snore
```

> Move the executable into your [`PATH`](https://askubuntu.com/questions/109381/how-to-add-path-of-a-program-to-path-environment-variable) to use it like any other program installed on the system

### Windows Executable

<a href="https://github.com/fin-ger/snore/releases/latest/download/snore-windows">
  <img src="https://img.shields.io/badge/download-windows-blue?style=for-the-badge&logo=windows" alt="Download">
</a>

Download `snore-windows.exe` and rename it to `snore.exe`. You can now run the program in the Windows command-line:

```cmd
C:\> \path\to\snore.exe
```

> Move the executable into your [`PATH`](https://stackoverflow.com/a/41895179/7216382) to use it like any other program installed on the system

## Usage

When `snore` is available in your `PATH` you can start using it.

**Run `snore` for one second**

```shell
$ snore 1
```

**Run `snore` for 200 milliseconds**

```shell
$ snore 200ms
```

**Run `snore` for 1 hour and 20 minutes**

```shell
$ snore 1h 20m
```

**Run `snore` for 2 seconds and 500 milliseconds**

```shell
$ snore 500ms 2s
```

**Run `snore` for one and a half seconds**

```shell
$ snore 1.5s
```

**Run `snore` for 0.001 days**

```shell
$ snore 0.001d
```

## Building the Project

Instead of downloading a precompiled binary, you can build the project yourself from source. First you have to setup a Rust toolchain. I recommend using [`rustup`](https://rustup.rs/). When the latest Rust stable toolchain is successfully installed, you can compile the code.

```
$ cargo install --path .
```

The program will be installed to `~/.cargo/bin/snore`.
 
## Troubleshooting

If you find any bugs/unexpected behaviour or you have a proposition for future changes open an issue describing the current behaviour and what you expected.

## Deployment TODO

- [x] elf statically linked binary
- [x] mac executable
- [x] windows exe
- [ ] ebuild
- [ ] rpm
- [ ] deb
- [ ] pkgbuild
- [ ] apkbuild
- [ ] windows msi
- [ ] solaris
- [ ] homebrew
- [ ] macports
- [ ] chocolatey
- [ ] scoop
- [ ] nix
- [ ] openbsd
- [ ] netbsd
- [ ] freebsd
- [ ] npm
- [ ] pip
- [ ] cargo
- [ ] ruby bundler

## Authors

**Fin Christensen**

> [:octocat: `@fin-ger`](https://github.com/fin-ger)  
> [:elephant: `@fin_ger@weirder.earth`](https://weirder.earth/@fin_ger)  
> [:bird: `@fin_ger_github`](https://twitter.com/fin_ger_github)  

## Show your support

Give a :star: if this project helped you!