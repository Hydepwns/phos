# Build stage - using Alpine for smaller base and musl for static linking
FROM rust:alpine AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev git perl openssl-dev

WORKDIR /app

# Copy everything needed for the build
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY benches ./benches
COPY .git ./.git

# Build the release binaries
RUN PHOS_GIT_HASH=$(git rev-parse --short HEAD 2>/dev/null || echo "docker") \
    PHOS_BUILD_DATE=$(date -u +%Y-%m-%d) \
    cargo build --release --bin phos --bin phoscat

# Strip binaries for smaller size
RUN strip target/release/phos target/release/phoscat

# Final stage - scratch image (empty, just the binaries)
FROM scratch

# Copy the static binaries
COPY --from=builder /app/target/release/phos /phos
COPY --from=builder /app/target/release/phoscat /phoscat

# Set the entrypoint
ENTRYPOINT ["/phos"]
