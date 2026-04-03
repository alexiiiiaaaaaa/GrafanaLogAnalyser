# --- Stage 1: Build ---
FROM rust:1.80-slim AS builder

WORKDIR /app
# Install linker and build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev sqlite3 libsqlite3-dev

# Copy your source code
COPY . .

RUN chmod -R 755 /app

# Build the release binary
RUN cargo build --release

# --- Stage 2: Runtime ---
FROM debian:bookworm-slim

# Install runtime dependencies (SQLite lib)
RUN apt-get update && apt-get install -y libsqlite3-0 ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/atm-parser .

# Create a directory for the database
RUN mkdir -p /app/data /app/sqlite

# Set the environment variable for your app to find the DB
#ENV DATABASE_URL=sqlite:///app/data/atm_data.db

# Run the app
CMD ["./atm-parser"]
