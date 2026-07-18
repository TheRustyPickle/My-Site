FROM rust:1.96-bookworm AS chef
# Use cargo-chef to cache dependencies
RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/LukeMathWalker/cargo-chef/releases/download/v0.1.77/cargo-chef-installer.sh | sh

# Prepare the build context
FROM chef AS planner
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Build dependencies using cargo-chef cache
FROM chef AS builder

# Install required tools
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends protobuf-compiler curl wget \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

RUN wget https://github.com/WebAssembly/binaryen/releases/download/version_131/binaryen-version_131-x86_64-linux.tar.gz \
    && tar -xvzf binaryen-version_131-x86_64-linux.tar.gz \
    && cp binaryen-version_131/bin/wasm-opt /usr/local/bin/ \
    && rm -rf binaryen-version_131*

RUN rustup target add wasm32-unknown-unknown
RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/download/v0.3.7/cargo-leptos-installer.sh | sh
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json


RUN curl -Lo /usr/local/bin/tailwindcss https://github.com/tailwindlabs/tailwindcss/releases/download/v4.3.3/tailwindcss-linux-x64 \
    && chmod +x /usr/local/bin/tailwindcss

# Add WASM target
RUN rustup target add wasm32-unknown-unknown

# Set up the application directory and copy source code
RUN mkdir -p /app
WORKDIR /app
COPY . .


ENV LEPTOS_TAILWIND_VERSION="4.3.3"

# Build the app with cargo-leptos
RUN cargo leptos build --split --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

# Copy the built application
COPY --from=builder /app/target/release/server /app/
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/Cargo.toml /app/
COPY --from=builder /app/server/src/ca.pem /app/ca/ca.pem

# Set environment variables
ENV CERT_LOCATION="/app/ca/ca.pem"
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
ENV IP=0.0.0.0
ENV PORT=10000
ENV ADDRESS=0.0.0.0

# Expose the port
EXPOSE 8080
EXPOSE 10000

# Run the server
CMD ["/app/server"]
