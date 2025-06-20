# Build stage
FROM rust:bookworm AS builder
 
WORKDIR /app
COPY . .
RUN cargo build --release
 
# Final run stage
FROM debian:bookworm-slim AS runner
 
# Install required dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
 
WORKDIR /app
COPY --from=builder /app/target/release/karcis-com-backend /app/karcis-com-backend
# Copy the config directory to the runtime image
COPY --from=builder /app/config /app/config
CMD ["/app/karcis-com-backend"]