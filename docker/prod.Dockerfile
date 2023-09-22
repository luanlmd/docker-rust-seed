FROM rust:1.70.0-alpine3.18
RUN apk add --no-cache musl-dev
WORKDIR /app
ADD . /app/
RUN cargo build --release

FROM alpine:3.18
COPY --from=0 /app/target/release/docker-rust-seed /opt
CMD ["/opt/docker-rust-seed"]