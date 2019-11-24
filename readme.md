# Git-Maven (GM) Utils

[![Build Status](https://travis-ci.com/d-kos/gm-utils.svg?branch=master)](https://travis-ci.com/d-kos/gm-utils)

Cli tool for managing git branch version and Java maven project version at the same time.
Although there is a lot of options for this [use case](https://stackoverflow.com/questions/13583953/deriving-maven-artifact-version-from-git-branch) my main motivation was to build a tool for managing git/maven project versions without any pre-requirements such as maven plugins.

## Flow
1) Start feature branch from master branch which is always in x.y.z-SNAPSHOT
2) Open new git branch e.g. 'feature-a' and set mvn project version to x.y.z-feature-a-SNAPSHOT
3) Set project version back as master when opening a pull request

## Usage
```
~> gm --help
gm-utils 0.1.0

USAGE:
    gm [FLAGS] [OPTIONS]

FLAGS:
    -d, --debug    Activate debug mode
    -h, --help     Prints help information
    -r, --reset    Reset maven project version

OPTIONS:
    -v, --version <version>    Git branch / mvn project version
```

## Installation
If you have Rust installed clone project and run `cargo install`. Set flag `--force` for upgrades.

## TODO
Add precompiled exec binary for installation


## License

GM Utils is licensed under the [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
