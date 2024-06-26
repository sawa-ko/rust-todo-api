FROM alpine:latest as builder

# Set environment variables
ENV PATH="/root/.cargo/bin:${PATH}:/app/target/release/migration"
ENV RUSTFLAGS="-Ctarget-feature=-crt-static"

# Install build dependencies
RUN apk update && \
    apk add --no-cache \
    curl \
    gcc \
    g++ \
    libc-dev \
    openssl-dev

# Install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Check rust version
RUN rustc --version && cargo --version

# Set working directory
WORKDIR /app

# Copy source code
COPY . .

# Build migrations
RUN cargo build --release --package migration

# Run migrations
RUN ./target/release/migration

# Build application
RUN cargo build --release --package todo_api

FROM alpine:latest as runtime

# Set environment variables
ENV PATH="/app/target/release:${PATH}/app/todo_api"
ENV ROCKET_ADDRESS="0.0.0.0"

# Install runtime dependencies
RUN apk update && \
    apk add --no-cache \
    openssl \
    libgcc \
    libstdc++

# Set working directory \
WORKDIR /app

# Copy binary
COPY --from=builder /app/target/release/todo_api .

# Create .env file if it doesn't exist
RUN test -f ./.env || echo "" > ./.env

# Expose port
EXPOSE 8000

# Run application
ENTRYPOINT ["./todo_api"]