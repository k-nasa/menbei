# menbei

[![CircleCI](https://circleci.com/gh/k-nasa/menbei.svg?style=svg)](https://circleci.com/gh/k-nasa/menbei)
[![crate-name at crates.io](https://img.shields.io/crates/v/menbei.svg)](https://crates.io/crates/menbei)

Tool to generate github issue link

## Installation
using cargo
```
cargo install menbei
```

using homebrew
```
brew tap k-nasa/homebrew-menbei
brew install menbei
```

## Usage
```
menbei 0.1.0
Tool to generate github issue link

USAGE:
    menbei [file] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <file>    issue link definition file

SUBCOMMANDS:
    dialogure    Create an issue link interactively
    help         Prints this message or the help of the given subcommand(s)
```

## Usage example
Generate links interactively

```
menbei dialogure

Input repository name: k-nasa/menbei
Input title: title
Input body: body
Input assignees: k-nasa
Input labels: question
Input projects:
```

output this
```
https://github.com/k-nasa/menbei/issues/new?title=title&body=body&assignees=k-nasa&labels=question&projects=
```

Generate link from file
```
menbei test.yaml
# or
menbei test.toml
```

## File example
YAML
``` test.yaml
repository: k-nasa/menbei
title: title
body: hogehoge
assignees:
- k-nasa
- hoge
labels:
- bug
- question
projects:
- k-nasa/menbei/1
```

TOML
``` test.toml
repository = "k-nasa/menbei"
title = "title"
body = "hogehoge"
assignees = ["k-nasa", "hoge"]
labels = ["bug", "question"]
projects = ["k-nasa/menbei/1"]
```
