FROM rust:1.70.0-alpine3.18

RUN apk add --no-cache musl-dev

WORKDIR /app

ADD . /app/

RUN cargo build --release

RUN set -x && ls -lah /app/target/release

CMD ["/app/target/release/docker-rust-seed"]