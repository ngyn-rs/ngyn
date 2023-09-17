FROM rust:1.55 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:buster-slim
WORKDIR /app
COPY --from=builder /app/target/release/basic_app .
EXPOSE 8080
CMD ["./basic_app"]
