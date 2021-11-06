FROM rust:1.53-slim-buster as builder

RUN apt-get update && apt-get install -y build-essential default-libmysqlclient-dev libssl-dev openssl pkg-config
RUN cargo install sccache

ENV HOME=/home/rust
ENV SCCACHE_CACHE_SIZE="1G"
ENV SCCACHE_DIR=$HOME/.cache/sccache
ENV RUSTC_WRAPPER="/usr/local/cargo/bin/sccache"

WORKDIR $HOME
COPY ctfdb ctfdb
COPY rest-api rest-api

WORKDIR $HOME/rest-api/

RUN --mount=type=cache,target=$SCCACHE_DIR cargo install --path .

FROM debian:buster-slim

RUN apt-get update && apt-get upgrade -y && apt-get install -y mariadb-client openssl ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/rest-api /usr/bin/rest-api

USER 1000

CMD ["rest-api"]
