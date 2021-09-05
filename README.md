# NOTICE

This repository has been archived.
It was intended as a proof-of-concept for hosting an API server (HTTP) to leverage `libkrun`.
It is not being pursued at this time, but may continue to do so in the future.
Thank you for checking out the project.

# libkrun-apiserver

[![GitHub](https://img.shields.io/github/license/nickgerace/libkrun-apiserver?style=flat-square)](./LICENSE)

This repository is extremely experimental and is a wrapper around [containers/krunvm](https://github.com/containers/krunvm).
It is recommended to interface with [containers/libkrun](https://github.com/containers/libkrun) directly rather than executing `krunvm` commands.
However, the functionality between calling `krunvm` directly and making `libkrun` library calls should be roughly equivalent.
Thus, this experiment exists to support the idea of an "API server for `libkrun`".
If the experiment continues, `krunvm` should be replaced with direct `libkrun` calls.

## Getting Started

You can get started on any machine with [vagrant](https://www.vagrantup.com/) installed.

> Before starting, you may want to check the `Vagrantfile` settings (particularly for the CPU and memory).

First, start the virtual machine and `ssh` into it.
Startup may take a long time due to setup.

```sh
vagrant up
vagrant ssh
```

Once inside the virtual machine, execute the following commands:

```sh
cd /vagrant
cargo build --release
buildah unshare
```

> You can also cross-compile, or natively build, the binary on your host.

Now that we are inside the `buildah unshare` environment, we can start the API server.

```sh
./target/release/krunvm-apiserver
```

Once the server is running, try sending an HTTP request to it.

```sh
curl -i http://localhost:3030/arguments \
    -X POST \
    -H "Content-Type: application/json" \
    -d '{"args": ["--help"]}'
```

## Developing

This repository requires a nightly toolchain for formatting, but uses a stable toolchain for everything else.

```sh
cargo +nightly fmt
```

## Code of Conduct

This repository follows and enforces the Rust programming language's [Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).

## Additional Information

- Author: [Nick Gerace](https://nickgerace.dev)
- License: [Apache 2.0](./LICENSE)
