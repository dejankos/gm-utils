# Git-Maven (GM) Utils

[![Build Status](https://travis-ci.com/dejankos/gm-utils.svg?branch=master)](https://travis-ci.com/dejankos/gm-utils)

Cli tool for managing git branch version and Java maven project version at the same time.
Although there is a lot of options for this [use case](https://stackoverflow.com/questions/13583953/deriving-maven-artifact-version-from-git-branch) my main motivation was to build a tool for managing git/maven project versions without any pre-requirements such as maven plugins.

## Flow
1) Start feature branch from master branch which is always in x.y.z-SNAPSHOT
2) Open new git branch e.g. 'feature-a' and set mvn project version to x.y.z-feature-a-SNAPSHOT
3) Set project version back as master when opening a pull request

## Usage
```
~> gm --help
   gm-utils 0.1.2
   
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
>gm test123 -d
18:17:09 [DEBUG] (1) gm::app: Received cli args = CliArgs { new_version: Some("test123"), debug: true, reset: false }
18:17:09 [DEBUG] (1) gm::app: Checking git is installed...
18:17:09 [DEBUG] (1) gm::app: Git installed.
18:17:09 [DEBUG] (1) gm::app: Checking maven is installed...
18:17:09 [DEBUG] (1) gm::app: Maven installed.
18:17:09 [DEBUG] (1) gm::app: Project working dir = "/home/user/current_project"
18:17:09 [ INFO] Creating new git branch test123 from master
18:17:09 [ INFO] Branch test successfully created from master
18:17:09 [ INFO] Current mvn project version 0.0.1-SNAPSHOT
18:17:09 [ INFO] Changing mvn project version to 0.0.1-test123-SNAPSHOT
18:17:11 [ INFO] Done
```

Reset version:
```
> gm -r
18:23:27 [ INFO] Current mvn project version 0.0.1-test123-SNAPSHOT
18:23:27 [ INFO] Changing mvn project version to 0.0.1-SNAPSHOT
18:23:28 [ INFO] Done
```

For debug output set flag `-d`.

## Installation
Download latest precompiled executable (Linux/OSX) from [release page](https://github.com/d-kos/gm-utils/releases) and add to system path.
Add executable to `PATH` in `~/.bashrc` or to `/usr/local/bin`.

If you have Rust installed clone project and run `cargo install`, set flag `--force` for upgrades.
Or run command `curl -LSfs https://japaric.github.io/trust/install.sh | sh -s -- --git d-kos/gm-utils`. 

## TODO
- reset version to current remote project (master branch) version instead of local

## License

GM Utils is licensed under the [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
