# stupid-git

[![crates.io](https://img.shields.io/crates/v/sgit.svg)](https://crates.io/crates/sgit)
[![license](https://img.shields.io/crates/l/sgit)](https://github.com/inherd/sgit/blob/master/LICENSE)
[![Sgit Build](https://github.com/phodal/sgit/actions/workflows/build.yml/badge.svg)](https://github.com/phodal/sgit/actions/workflows/build.yml)

> A simple cli to clone projects and update all projects.

- [x] get all repository from GitHub
- [x] clone all
- [x] pull all
    - [x] with `git stash`

## Usage

1. create `sgit.yaml` file
2. run `sgit clone` or `sgit pull`

sample of `sgit.yaml`

```yaml
repos:
  - https://github.com/phodal/sgit.git
```

### auto gen repos for orgs

1. config for orgs with `sgit.yaml`:

```yaml
---
organization: xxx
token: xxx
repos: []
```

for private projects, you need to config `token` from: [https://github.com/settings/tokens](https://github.com/settings/tokens)

2. run `sgit gen` for fetch all repos info

```
sgit gen
```

3. run `sgit clone` or `sgit pull`

### others

```
sgit
A multiple repo's git cli

USAGE:
    sgit <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    add      add a repos !! not implement
    clone    Clones repos
    gen      generate sgit by org
    help     Print this message or the help of the given subcommand(s)
    init     init sgit config
    push     pushes things
```

License
---

@2022 This code is distributed under the MIT license. See `LICENSE` in this directory.
