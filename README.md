# CodeChain Agent [![Build Status](https://travis-ci.org/CodeChain-io/codechain-agent.svg?branch=master)](https://travis-ci.org/CodeChain-io/codechain-agent)

## Requirements

The following are the software dependencies required to install and run CodeChain-agent:

- [CodeChain](https://github.com/CodeChain-io/codechain)
- [CodeChain-agent-hub](https://github.com/CodeChain-io/codechain-agent-hub)

### Install dependencies (Ubuntu)

```
sudo apt install pkg-config libssl-dev
```

## Run

To run CodeChain-agent, just run

```
cargo run -- --agent-hub-url <agent-hub-url> --codechain-dir <codechain-dir> --codechain-p2p-address <codechain-p2p-address> --name <name>
```

## Formatting

Make sure you run `rustfmt` before creating a PR to the repo. You need to install the nightly-2018-07-17 version of `rustfmt`.

```
rustup toolchain install nightly-2018-07-17
rustup component add rustfmt-preview --toolchain nightly-2018-07-17
```

To run `rustfmt`,

```
cargo +nightly-2018-07-17 fmt
```

## User Manual

Under `docs` folder, run following command.

```
make html
```

User manual will be generated at `docs/_build/html`.
