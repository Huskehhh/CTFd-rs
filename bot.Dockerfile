FROM ghcr.io/huskehhh/rust-sccache:latest as builder

WORKDIR $HOME
COPY ctfdb ctfdb
COPY bot bot

WORKDIR $HOME/bot/

RUN --mount=type=secret,id=AWS_ACCESS_KEY_ID \
    --mount=type=secret,id=AWS_SECRET_ACCESS_KEY \
    --mount=type=secret,id=SCCACHE_ENDPOINT \
    --mount=type=secret,id=SCCACHE_BUCKET \
    --mount=type=secret,id=SCCACHE_S3_USE_SSL \
    export AWS_ACCESS_KEY_ID=$(cat /run/secrets/AWS_ACCESS_KEY_ID) && \
    export AWS_SECRET_ACCESS_KEY=$(cat /run/secrets/AWS_SECRET_ACCESS_KEY) && \
    export SCCACHE_ENDPOINT=$(cat /run/secrets/SCCACHE_ENDPOINT) && \
    export SCCACHE_BUCKET=$(cat /run/secrets/SCCACHE_BUCKET) && \
    export SCCACHE_S3_USE_SSL=$(cat /run/secrets/SCCACHE_S3_USE_SSL) && \
    cargo install --path .

FROM debian:buster-slim

RUN apt-get update && apt-get upgrade -y && apt-get install -y mariadb-client openssl ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/ctf_bot /usr/bin/ctf_bot

USER 1000

CMD ["ctf_bot"]
