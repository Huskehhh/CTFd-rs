FROM rust:1.53-slim-buster as builder

RUN apt-get update && apt-get install -y build-essential default-libmysqlclient-dev libssl-dev openssl pkg-config

WORKDIR /home/rust/
COPY ctfdb ctfdb
COPY rest-api rest-api

WORKDIR /home/rust/rest-api/

RUN cargo install --path .

FROM debian:buster-slim

RUN apt-get update && apt-get upgrade -y && apt-get install -y mariadb-client openssl ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/rest-api /usr/local/bin/rest-api

CMD ["rest-api"]
