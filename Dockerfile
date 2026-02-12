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
    zlib-dev \
    && for p in build-base openssl-dev curl pkgconfig bash git libc-dev zlib-dev; do \
      echo "=== $p ==="; \
      apk info -v "$p" || apk info -W "$p" || true; \
    done

# print the version to later specify

RUN cargo install cargo-binstall
RUN cargo binstall trunk wasm-bindgen-cli

RUN rustup target add wasm32-unknown-unknown

WORKDIR /app
COPY . .
RUN trunk build --release

# Stage 2: Serve with nginx
FROM nginx:alpine

# Copy build output to nginx html directory
COPY --from=builder /app/dist /usr/share/nginx/html

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
