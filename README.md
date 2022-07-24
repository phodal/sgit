# stupid-git

A simple cli to clone projects and update all projects.

1. create `sgit.yaml` file
2. run `sgit clone` or `sgit pull`

sample of `sgit.yaml`

```yaml
repos:
  - https://github.com/phodal/sgit.git
```

Todo in future:

- [x] get all repository from GitHub

generate repos by orgs:

```yaml
---
repos:
organization: xxx
token: xxx
```

License
---

@2022 This code is distributed under the MIT license. See `LICENSE` in this directory.
