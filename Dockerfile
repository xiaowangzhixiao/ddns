FROM rust:1.67 as builder
WORKDIR /usr/src/ddns
COPY . .
RUN cargo install --path ./ddns

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y sqlite3 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/ddns /usr/local/bin/ddns
RUN mkdir -p /data
VOLUME [ "/data" ]
ENTRYPOINT ["ddns"]
CMD ["/data"]