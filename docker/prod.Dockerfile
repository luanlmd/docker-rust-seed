FROM rust:1.70.0-alpine3.18

WORKDIR /app

ADD . /app/

RUN cargo build --release

# RUN ls -lah /app/target/release

CMD ["/app/target/release/docker-rust-seed"]