FROM rust:1.53-slim-buster as builder

RUN apt-get update && apt-get install -y build-essential default-libmysqlclient-dev libssl-dev openssl pkg-config

WORKDIR /home/rust/
COPY ctfdb ctfdb
COPY bot bot

WORKDIR /home/rust/bot/

RUN cargo install --path .

FROM debian:buster-slim

RUN apt-get update && apt-get upgrade -y && apt-get install -y mariadb-client openssl ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/ctf_bot /usr/bin/ctf_bot

USER 1000

CMD ["ctf_bot"]
