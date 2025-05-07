# Dockerfile

# Build stage
FROM rust:latest AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Runtime stage
FROM ubuntu:23.04
WORKDIR /app 
COPY --from=builder /app/target/release/askus-v2 askus-v2
EXPOSE 8080
CMD ["./askus-v2"]
