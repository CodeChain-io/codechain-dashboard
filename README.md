# CodeChain Dashboard

## Requirements

The software dependencies required to install and run CodeChain-Dashboard are:

- Latest version of the [CodeChain-Agent-Hub](https://github.com/CodeChain-io/codechain-agent-hub)

## Run

Run codechain-dashboard in the development mode.

```
yarn install
yarn run start
```

## Production build

```
yarn install
yarn run build
```

## Configuration

|                           | Default               |
| ------------------------- | --------------------- |
| REACT_APP_AGENT_HUB_HOST  | ws://localhost:3012   |
| REACT_APP_LOG_SERVER_HOST | http://localhost:5012 |
