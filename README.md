ZooKeeper Registrar
===================

A service for registering a set of service fixtures into zookeeper.

Build
-----

To perform a development build

    cargo build

To perform an optimal release build

    cargo build --release
    strip -S target/release/zookeeper-registrar
    upx --ultra-brute target/release/zookeeper-registrar

Usage
-----

    Zookeeper Registrar

    Registers provided service fixtures in zookeeper.

    Usage:
      zookeeper-registrar [options] <path>

    Options:
      -z --zookeeper=<address>  Zookeeper connection string [default: localhost:2181].
      -h --help                 Show this screen.
      --version                 Show version.

Configuration
-------------

The service takes a path to a services JSON file which defines the static services to register, the configuration is as follows:

    {
      "services": [
        { "name": "name/of/service", "address": "localhost", "port": 8080 }
      ]
    }

Where the name is the path under `/services` in the zookeeper registry, note that a random service uuid will be appended as the ephemeral node. i.e. the above would register an entry for `/services/name/of/service/<uuid>`.

When the registrar is terminated `CTRL+C` the ephemeral node will be removed however the service path `/services/name/of/service` will remain.
