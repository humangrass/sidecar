# Sidecar

Easily extensible Rust services for tracing.

## Features

### Proxycar

**Proxycar** is a lightweight, extendable Rust-based HTTP proxy service designed for use as a sidecar. It proxies client
requests to a target service, logs incoming requests and outgoing responses, and supports basic tracing functionality
for distributed systems.

`// TODO`

## Installation

Clone the repository and navigate to the proxycar directory:

```bash
git clone git@github.com:humangrass/sidecar.git
```

## Usage

Create configuration file (e.g. `proxycar.config.example.yaml`)

```bash
cp proxycar.config.example.yaml proxycar.config.yaml
```

Run the service:

```bash
cargo run -- --config config.yaml
```

Proxy service will be available at `http://localhost:3000`

Requests are forwarded to the target service defined in the configuration (`http://localhost:8000` in this example).

### Cli Options

```bash
cargo run -- -h
```

### Build

```bash
cargo build --release
```
