# Stage 1: Build the Yew app
FROM rust:1.91.0-alpine3.22 AS builder

RUN apk add --no-cache \
    build-base \
#     openssl-dev \
    curl \
#     pkgconfig \
    bash 
#     git \ # not needed apparently 
#     libc-dev \
#     zlib-dev

RUN BINSTALL_VERSION=1.17.4 \
    curl -L https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh \
  | bash \
 && "$CARGO_HOME/bin/cargo-binstall" trunk@0.21.14 wasm-bindgen-cli@0.2.108 --no-confirm \
 && rustup target add wasm32-unknown-unknown

WORKDIR /app
COPY . .
RUN trunk build --release

# Stage 2: Serve with nginx
FROM nginx:alpine

# Copy build output to nginx html directory
COPY --from=builder /app/dist /usr/share/nginx/html

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
