FROM rust:1.70 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM ubuntu:20.04
WORKDIR /app
COPY --from=builder /app/target/release/basic_app .
EXPOSE 8080
CMD ["./basic_app"]
