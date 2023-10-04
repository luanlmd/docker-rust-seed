FROM rust:1.72.1-alpine3.18

RUN apk add --no-cache musl-dev
RUN cargo install cargo-watch

WORKDIR /src

CMD ["cargo", "watch" ,"-w", "src", "-x", "run"]