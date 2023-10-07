FROM rust:1.73.0-slim-bullseye as builder
WORKDIR /usr/src/volunte-api
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/volunte-api /usr/local/bin/volunte-api
CMD ["volunte-api", :$URI, :$PORT]