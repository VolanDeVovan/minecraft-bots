FROM rust:1.65.0-alpine as builder

WORKDIR /usr/src/app

RUN apk add --no-cache musl-dev openssl-dev && \
    rustup default nightly 

COPY . .

RUN cargo build --release

FROM alpine:3.16

WORKDIR /app

COPY --from=builder /usr/src/app/target/release/minecraft-bots /app/minecraft-bots

CMD /app/minecraft-bots