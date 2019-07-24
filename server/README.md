CodeChain Agent Hub
====================

[![Join the chat at https://gitter.im/CodeChain-io/codechain-agent-hub](https://badges.gitter.im/CodeChain-io/codechain-agent-hub.svg)](https://gitter.im/CodeChain-io/codechain-agent-hub?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

CodeChain Agent Hub is a server which collects many CodeChain node's information(best block, pending transactions, log, ...). Also, CodeChain Agent Hub serves collected data to CodeChain Dashboard.

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
To safely communicate with the Dashboard, please set the `PASSPHRASE` environment variable. The Dashboard program should use the same passphrase.
Also, you should set `NETWORK_ID` environment variable to print the network id in log messages.

When you are using the `PASSPHRASE` you should use SSL over the connection. If you don't use the SSL, the `PASSPHRASE` is open to the internet. 

CodeChain Agent Hub will listen 3012 port to communicate with the Dashboard using JSON-RPC.

CodeChain Agent Hub will listen 4012 port to communicate with the Agent using JSON-RPC.

Alerts
-------

The server sends an alert via Slack and Email in situations where there likely is a problem.

## Email alerts
To use email alerts, the server needs the [Sendgird](https://sendgrid.com/) api key.
```
SENDGRID_API_KEY={api key} SENDGRID_TO={email address} codechain-agent-hub
```

## Slack alerts
The server uses [webhooks](https://api.slack.com/incoming-webhooks)
```
SLACK_HOOK_URL={web hook url} codechain-agent-hub
```

Environmental Variables
------------------------

| NAME              | DESCRIPTION                                                                                          |
| ----------------- | ---------------------------------------------------------------------------------------------------- |
| START_AT_CONNECT  | If this variable is set, a CodeChain instance is started once an agent connects to the agent server. |
| NETWORK_ID        | Network ID information that is used in error messages or logs.                                       |
| SLACK_WEBHOOK_URL | Used to send alarms to Slack.                                                                        |
| SENDGRID_TO       | An email address to receive alarm emails.                                                            |
| SENDGRID_API_KEY  | An API Key that is used to send alarms.                                                              |
| PASSPHRASE        | A passphrase that is used to communicate with the Dashboard safely.                                  |
