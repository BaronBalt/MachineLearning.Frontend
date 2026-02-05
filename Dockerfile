# Stage 1: Build the Yew app
FROM rust:1.91.0-alpine3.22 AS builder

RUN apk add --no-cache \
    build-base \
    openssl-dev \
    curl \
    pkgconfig \
    bash \
    git \
    libc-dev \
    zlib-dev
# Install trunk and wasm-bindgen-cli
RUN cargo install cargo-binstall
RUN cargo binstall trunk wasm-bindgen-cli

# Add the wasm target BEFORE building
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app

# Copy dependency files first for caching
COPY Cargo.toml Cargo.lock ./

RUN mkdir -p src
RUN echo "fn main() {println!(\"Hello, world!\");}" > src/main.rs

# Pre-download dependencies
RUN cargo fetch

# Copy the rest of the source code
COPY . .

# Build the Yew app into /app/dist
RUN trunk build --release

# Stage 2: Serve the static web app
FROM nginx:stable-alpine

# Copy the built files from the builder
COPY --from=builder /app/dist /usr/share/nginx/html

# Expose port 80 for web traffic
EXPOSE 80

# Start nginx
CMD ["nginx", "-g", "daemon off;"]

