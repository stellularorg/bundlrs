# Bundlrs Orbiter CLI

Experimental CLI for viewing pastes through a CLI.

> **During BETA, you'll have to build the CLI manually on Windows, MacOS, and Linux! See instructions below.**

## Build

```bash
cargo install just
git clone https://code.stellular.org/stellular/bundlrs
cd bundlrs/orbiter
just build
```

## Configuration

Configure server: (`https://stellular.net` is default)

- Create a file named `.orbiter.toml`
- Set the global `server` key

## Usage

Authenticate with secondary token:

```bash
# command
orbiter token {token}

# example
orbiter token 0000000000
```

View a paste:

```bash
# command
orbiter view {paste}

# example
orbiter view pub/info
```
