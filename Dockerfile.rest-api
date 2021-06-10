FROM ekidd/rust-musl-builder:latest as builder

WORKDIR /home/rust/
ADD --chown=rust:rust ctfdb ctfdb
ADD --chown=rust:rust rest-api rest-api

WORKDIR /home/rust/rest-api/

RUN cargo build --release

FROM alpine:latest

RUN apk --no-cache add ca-certificates

COPY --from=builder /home/rust/rest-api/target/x86_64-unknown-linux-musl/release/rest-api /

CMD ["./rest-api"]
