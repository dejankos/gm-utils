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
   gm-utils 0.1.1
   
   USAGE:
       gm [FLAGS] [new-version]
   
   FLAGS:
       -d, --debug      Activate debug mode
       -h, --help       Prints help information
       -r, --reset      Reset maven project version
       -V, --version    Prints version information
   
   ARGS:
       <new-version>    Git branch / mvn project version

```

## Examples
Change git branch and maven project version from project root:
```
>gm test123

Checking if branch test123 already exists...
Creating new git branch test123 from master
 
Branch test123 successfully created from master

Current mvn project version 0.0.1-SNAPSHOT
Changing mvn project version to 0.0.1-test123-SNAPSHOT
Done
```

Reset version:
```
> gm -r
Current mvn project version 0.0.1-test123-SNAPSHOT
Changing mvn project version to 0.0.1-SNAPSHOT
Done
```

For debug output set flag `-d`.

## Installation
Download latest precompiled executable (only for Linux) from [release page](https://github.com/d-kos/gm-utils/releases) and add to system path.
Add executable to `PATH` in `~/.bashrc` or to `/usr/local/bin`.

If you have Rust installed clone project and run `cargo install`. Set flag `--force` for upgrades.
Or run command `curl -LSfs https://japaric.github.io/trust/install.sh | sh -s -- --git d-kos/gm-utils`. 

Available only for Linux.

## TODO
- reset version to current remote project (master branch) version instead of local

## License

GM Utils is licensed under the [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
