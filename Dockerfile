FROM rust:1.85-bullseye as chef
# Use cargo-chef to cache dependencies
RUN cargo install cargo-chef

# Prepare the build context
FROM chef AS planner
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Build dependencies using cargo-chef cache
FROM chef AS builder

# Install required tools
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends protobuf-compiler ffmpeg curl wget \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json


RUN curl -Lo /usr/local/bin/tailwindcss https://github.com/tailwindlabs/tailwindcss/releases/download/v4.0.11/tailwindcss-linux-x64 \
    && chmod +x /usr/local/bin/tailwindcss

RUN curl -L -o cargo-leptos.tar.gz https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-x86_64-unknown-linux-gnu.tar.gz \
    && tar -xzf cargo-leptos.tar.gz \
    && cd cargo-leptos-x86_64-unknown-linux-gnu \
    && mv cargo-leptos /usr/local/bin/ \
    && chmod +x /usr/local/bin/cargo-leptos \
    && cd ~ \
    && rm -rf cargo-leptos.tar.gz cargo-leptos-x86_64-unknown-linux-gnu

# Add WASM target
RUN rustup target add wasm32-unknown-unknown

# Set up the application directory and copy source code
RUN mkdir -p /app
WORKDIR /app
COPY . .


ENV LEPTOS_TAILWIND_VERSION="4.0.11"

# Build the app with cargo-leptos
RUN cargo leptos build --release -vv

FROM rust:1-bullseye AS runtime
WORKDIR /app
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates ffmpeg \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

# Copy the built application
COPY --from=builder /app/target/release/server /app/
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/Cargo.toml /app/

# Set environment variables
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
ENV PORT=8080
ENV IP=0.0.0.0

# Expose the port
EXPOSE 8080

# Run the server
CMD ["/app/server"]
