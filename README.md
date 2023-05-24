# Nextenv

a CLI for loading .env files, inspired by how next.js handles .env files.

## Installation

install with cargo, after cloning the source

```
cargo install --path .
```

## Usage

```
nextenv -- <any command>
```

or

```
nextenv -e <enviroment name> -- <any command>
```

.env files will then be loaded from the current directory,

env vars will be set for the provided command with the following precedence:

1. existing env vars
2. .env.{environment}.local
3. .env.local
4. .env.{environment}
5. .env
