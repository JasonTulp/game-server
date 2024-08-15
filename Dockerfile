FROM docker.io/library/rust:1.80-bookworm AS builder

WORKDIR "/server"

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Build the application
# During the build, DATABASE_URL will have a non-sensitive placeholder
ENV DATABASE_URL="placeholder"

# Build the application
# RUN #cargo build --release

# Run the application while exposing ports
EXPOSE 2048
CMD ["cargo", "run", "--release"]
