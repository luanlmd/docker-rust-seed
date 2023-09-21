FROM rust:1.70.0-alpine3.18

RUN apk add --no-cache musl-dev

WORKDIR /app

ADD . /app/

RUN cargo build

# RUN set -x && ls -lah /app/target/debug/

CMD ["/app/target/debug/docker-rust-seed"]