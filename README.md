# CodeChain Dashboard [![Build Status](https://travis-ci.com/CodeChain-io/codechain-dashboard.svg?branch=master)](https://travis-ci.com/CodeChain-io/codechain-dashboard) [![Join the chat at https://gitter.im/CodeChain-io/codechain-agent-hub](https://badges.gitter.im/CodeChain-io/codechain-agent-hub.svg)](https://gitter.im/CodeChain-io/codechain-agent-hub?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

CodeChain Dashboard helps you maintain the CodeChain nodes.

This repository consists of three projects.
- ui
    * This project produces the static web site to render the information that the server has.
- server
    * This project collects the nodes' information from clients, and sends the information that the ui requested.
- client
    * This project executes a CodeChain node and sends the node information to the server.

```
-------Static Web Site------------
             +----+
             | ui |
             +----+
               |
 +------+   +------+   +------+
 |client|---|server|---|client|
 +------+   +------+   +------+
  |            |        |
 CodeChain  +------+   CodeChain
            |client|
            +------+
             |
            CodeChain
```
