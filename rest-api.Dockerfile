FROM ghcr.io/huskehhh/rust-sccache:latest as builder

RUN apt-get update && apt-get install -y default-libmysqlclient-dev

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
