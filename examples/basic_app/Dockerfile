FROM rust:1.81 AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM ubuntu:20.04
WORKDIR /app
COPY --from=builder /app/target/release/basic_app .
COPY --from=builder /app/static .
EXPOSE 8080
CMD ["./basic_app"]
