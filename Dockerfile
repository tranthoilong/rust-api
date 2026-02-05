# Build stage
FROM rust:latest as builder

WORKDIR /usr/src/app
COPY . .

# We use sqlx-cli for compile-time verification if needed, or disable it. 
# Since sqlx checks are enabled in code, we need a DB connection or SQLX_OFFLINE=true.
# For simplicity in Docker build, we often use offline mode or require the lock file.
ENV SQLX_OFFLINE=true

RUN cargo build --release

# Production stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies (OpenSSL is common)
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/rust_api .
COPY --from=builder /usr/src/app/.env.example .env
COPY --from=builder /usr/src/app/migrations ./migrations 

# Expose port
EXPOSE 4000

# Run
CMD ["./rust_api"]
