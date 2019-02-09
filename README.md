CodeChain Agent Hub
====================

[![Join the chat at https://gitter.im/CodeChain-io/codechain-agent-hub](https://badges.gitter.im/CodeChain-io/codechain-agent-hub.svg)](https://gitter.im/CodeChain-io/codechain-agent-hub?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

CodeChain Agent Hub is a server which collects many CodeChain node's information(best block, pending parcels, log, ...). Also, CodeChain Agent Hub serves collected data to CodeChain Dashboard.

Install
--------

You should set up a rust toolchain.

You can install codechain-agent-hub by running `cargo install`

Install Postgres and create schema
-----------------

Ubuntu
```
sudo apt install postgresql postgresql-contrib
sudo -u postgres psql -f create_user_and_db.sql
generate-schema
```

Mac (brew)
```
brew install postgresql
brew services start postgresql
psql postgres -f create_user_and_db.sql
generate-schema
```

Run
----

Just run `codechain-agent-hub` in your shell.

CodeChain Agent Hub will listen 3012 port to communicate with the Dashboard using JSON-RPC.

CodeChain Agent Hub will listen 4012 port to communicate with the Agent using JSON-RPC.

CodeChain Agent Hub will listen 5012 port to serve CodeChain's log file using HTTP.
