# Build stage - using Alpine for smaller base and musl for static linking
FROM rust:alpine AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev git

WORKDIR /app

# Copy everything needed for the build
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY .git ./.git

# Build the release binary with version info
RUN PHOS_GIT_HASH=$(git rev-parse --short HEAD 2>/dev/null || echo "docker") \
    PHOS_BUILD_DATE=$(date -u +%Y-%m-%d) \
    cargo build --release

# Strip the binary for smaller size
RUN strip target/release/phos

# Final stage - scratch image (empty, just the binary)
FROM scratch

# Copy the static binary
COPY --from=builder /app/target/release/phos /phos

# Set the entrypoint
ENTRYPOINT ["/phos"]
