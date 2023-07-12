# Hermes - High-speed Engine for Rust Microserver Experience in Static-serving

![Version - 0.1.0](https://img.shields.io/static/v1?label=Version&message=0.1.0&color=blue&style=for-the-badge)
![Build - Passing](https://img.shields.io/static/v1?label=Build&message=Passing&color=brightgreen&style=for-the-badge)
![Rust version - 1.70.0](https://img.shields.io/static/v1?label=Rust+version&message=1.70.0&color=orange&style=for-the-badge)

---

**⚠️ Warning: Hermes is still in development and is not ready for production use. Use at your own risk.**

## Table of contents
- [What is Hermes?](#what-is-hermes)
- [Why Hermes?](#why-hermes)
- [Key features](#key-features)
- [Installation](#installation)
  - [Docker](#docker)
  - [From binary](#from-binary)
  - [From source](#from-source)
- [Usage](#usage)
    - [Prerequisites](#prerequisites)
        - [Configuration file](#configuration-file)
        - [SurrealDB](#surrealdb)
    - [Launching the server](#launching-the-server)
- [Commands](#commands)
- [REST API](#rest-api)
- [License](#license)

## What is Hermes?
**Hermess** (or **High-speed Engine for Rust Microserver Experience in Static-serving**) is a simple, lightweight, and fast HTTP server written in Rust. It is designed to be used as a reverse proxy for static websites providing a simple and easy-to-use interface for authentication per subdomain.

## Why Hermes?
While you could use a classic reverse proxy for this like NginX to redirect a subdomain to a specific static site, it is not possible to authenticate users smoothly with a login page, access per subdomain/static site, and manage access by API. This is where Hermes comes in.

## Key features
- **⚡ Fast** - Hermes is written in Rust, which is a lightning fast language.
- **🚀 Lightweight** - Hermes is designed to be as lightweight as possible.
- **📘 Easy to use** - Hermes is designed to be easy to use and configure. The server can be launched with a single command, and the REST API is only a few endpoints with a simple API key authentication.
- **🔒 Secure** - Hermes is designed to be secure. The server is only accessible from localhost by default, and the REST API is only accessible with an API key.

## Installation

### Docker

> This is not the final version of Hermes, thus no docker image is available yet.

### From binary

> This is not the final version of Hermes, thus no binary is available yet.

### From source

You first need to install Rust and Cargo. You can find the installation instructions [here](https://www.rust-lang.org/tools/install).

Then, you can clone the repository and build the project:

```bash
git clone git@github.com:owlnext-fr/hermes.git
cd hermes
cargo build --release
```

The binary will be available in `target/release/hermes`.

## Usage

### Prerequisites

#### Configuration file
Hermes needs some environment variables to be set. You can find an example of a configuration file in the repository: [.env.dist](.env.dist).

Here is a list of the environment variables you need to set:

| Variable name | Description | Default value |
| --- | --- | --- |
| `SDB_PUBLIC_PORT` | The port on which SurrealDB is listening. | `8000` |
| `SDB_USER` | The username to use to connect to SurrealDB. | `surreal` (you should change it) |
| `SDB_PASSWORD` | The password to use to connect to SurrealDB. | `surreal` (you should change it) |
| `SDB_NAMESPACE` | The namespace to use to connect to SurrealDB. | `hermes` |
| `SDB_DB` | The database to use to connect to SurrealDB. | `hermes` |
| `SDB_HOST` | The host to use to connect to SurrealDB. | `localhost` |
| `SDB_LOG_LEVEL` | The log level to use for SurrealDB. | `trace` |
| `RUST_LOG` | The log level to use for Hermes. | `error` |
| `RUST_BACKTRACE` | Whether to display backtraces or not. | `0` |

> For production purposes, you may want to configure environment variables as system variables and use proper secrets.

> The docker image uses these environment variables as well.

#### SurrealDB
Before using Hermes, you need a proper SurrealDB instance running.

For development purposes, you can use the dockercompose file available in the repository:

```bash
docker-compose up -d
```

For production purposes, you may want to configure a properly secured SurrealDB instance. You can find the installation instructions [here](https://surrealdb.com/install).

### Launching the server

Once you have configured the environment variables, you can launch the server:

```bash
# with cargo
cargo run --release -- server

# or with the binary
hermes server

# or with docker
TBD
```

### Commands

TBD

### REST API

TBD

## License
This project is licensed under the [MIT license](LICENSE).