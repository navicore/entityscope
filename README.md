<!-- ghmig:moved -->
> **This repository has moved to [https://git.navicore.tech/navicore/entityscope](https://git.navicore.tech/navicore/entityscope).**
>
> The GitHub copy is archived and no longer maintained.

# EntityScope

# UNDER CONSTRUCTION

The `entityscope` project is an experiment in combining generating code from
openapi specs with cli interfaces, 2d/3d game interfaces, and always-on digital
twin runtime concepts.

## Installation

Basic usage (includes generator only):
```cargo add entityscope```

Full functionality:
```cargo add entityscope --features full```

Custom setup:
```cargo add entityscope --features "generator explorer"```

## Features

- `generator`: Entity generation functionality
- `explorer`: Data exploration tools
- `runtime`: Entity twin runtime
- `full`: All features