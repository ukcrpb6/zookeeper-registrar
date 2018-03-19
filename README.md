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
