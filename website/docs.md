# Introduction

Cairo lint is a static code analysis tool for the Cairo language.

It can help you improve your code quality and consistency by checking the codebase against a set of predefined rules, called lints.
It can also automatically fix some of the issues found.

This tool is mostly depended on the separate lint rules. You can also read about every each of them here in the `Lints` section of the documentation.

## Installation

Cairo lint is provided within the [Scarb](https://docs.swmansion.com/scarb/) toolchain. You can install and download it [here](https://docs.swmansion.com/scarb/download.html)

## Getting started

To run Cairo lint in the current project, just type:

```sh
scarb lint
```

This will run the code analysis and suggest places to edit your code.
Running `lint` will yield issues like this:

```sh
$ scarb lint
  Linting hello_world v0.1.0 (/hello_world/Scarb.toml)
  warning: Plugin diagnostic: Unnecessary comparison with a boolean value. Use the variable directly.
   --> /hello_world/src/lib.cairo:2:8
    |
  2 |     if is_true() == true {
    |        -----------------
    |
```

To attempt to fix the issues automatically, you can run:

```sh
scarb lint --fix
```

You can also specify `--test` to perform analysis of your project's tests as well (i.e. all the Cairo code under `#[cfg(test)]` attributes).
To learn more about available arguments, just run `scarb lint --help`.
