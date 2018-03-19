FROM scratch
COPY target/x86_64-unknown-linux-musl/release/zookeeper-registrar /zookeeper-registrar
COPY etc/app.json /app.json
ENTRYPOINT ["/zookeeper-registrar", "/app.json"]
CMD [ "-z", "zookeeper:2181" ]
