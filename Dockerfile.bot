FROM ekidd/rust-musl-builder:latest as builder

WORKDIR /home/rust/
ADD --chown=rust:rust ctfdb ctfdb
ADD --chown=rust:rust bot bot

WORKDIR /home/rust/bot/

RUN cargo build --release

FROM alpine:latest

RUN apk --no-cache add ca-certificates

COPY --from=builder /home/rust/bot/target/x86_64-unknown-linux-musl/release/ctf_bot /

CMD ["./ctf_bot"]
