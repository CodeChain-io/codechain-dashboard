# CodeChain Dashboard UI

## Requirements

The software dependencies required to install and run CodeChain-Dashboard are:

- Latest version of the [CodeChain-Dashboard-Server](https://github.com/CodeChain-io/codechain-dashboard/tree/master/server)

## Run

Run codechain-dashboard-ui in the development mode.

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

|                                | Default               |
| ------------------------------ | --------------------- |
| REACT_APP_AGENT_HUB_HOST       | ws://localhost:3012   |
| REACT_APP_LOG_SERVER_HOST      | http://localhost:5012 |
| REACT_APP_TITLE                |                       |
| REACT_APP_AGENT_HUB_PASSPHRASE | passphrase            |
