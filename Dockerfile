# --- Builder stage ---
FROM rust:1-bookworm AS builder

WORKDIR /app

# Copy dependency files first for better caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies separately
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy actual source code
COPY src ./src

# Build the application
# Touch main.rs to ensure it's rebuilt
RUN touch src/main.rs && cargo build --release

# --- Runtime stage ---
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/* && \
    useradd -m -u 10001 appuser

WORKDIR /app

# Copy the binary (using correct binary name from Cargo.toml)
COPY --from=builder /app/target/release/cdn /usr/local/bin/cdn

# Set ownership
RUN chown -R appuser:appuser /app

# Set environment variables
ENV RUST_LOG=info

# Expose port (adjust if needed)
EXPOSE 8000

# Switch to non-root user
USER appuser

# Run the application
CMD ["/usr/local/bin/cdn"]