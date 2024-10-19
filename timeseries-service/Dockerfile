# Use the official Rust image as the base image
FROM rust:latest AS builder

# Set working directory
WORKDIR /usr/src/timeseries-service

# Copy the source code into the container
COPY . .

# Build the application
RUN cargo build --release --verbose

# Use a minimal base image to run the application
FROM debian:buster-slim

# Set working directory
WORKDIR /usr/local/bin

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/timeseries-service/target/release/timeseries-service .

# Run the application
CMD ["./timeseries-service"]
