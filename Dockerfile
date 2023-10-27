FROM rust:1.67 as builder
WORKDIR /usr/src/ddns
COPY . .
RUN cargo install -p ddns --path .

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/ddns /usr/local/bin/ddns
CMD ["ddns"]