# Builder stage
FROM rust:1.82 as builder

WORKDIR /app
COPY . .

# Build release binary
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app
# Install OpenSSL (needed for reqwest/https)
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/rust-llm-api .

CMD ["./rust-llm-api"]
