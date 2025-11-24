FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

# Build dependencies
RUN cargo chef cook --release --recipe-path recipe.json

# Build your app
COPY . .
RUN cargo build --release --bin youtube_summarizer_backend

# Runtime image
FROM debian:bookworm-slim AS runtime
WORKDIR /app

# For HTTPS support — required for calling YouTube/OpenAI APIs
RUN apt-get update && apt-get install -y ca-certificates && update-ca-certificates

# Copy the binary
COPY --from=builder /app/target/release/youtube_summarizer_backend /app/app

# Required by Fly.io
EXPOSE 8080

# Fly.io reads PORT environment variable
ENV PORT=8080

# Start the binary
CMD ["/app/app"]
