# Use the pre-built binary from Jenkins
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -r -s /bin/false appuser

# Copy the pre-built binary from Jenkins workspace
COPY target/release/actix-example /usr/local/bin/app

# Change ownership
RUN chown appuser:appuser /usr/local/bin/app

# Switch to non-root user
USER appuser

# Expose port (adjust if your app uses different port)
EXPOSE 8080

# Run the binary
CMD ["app"]