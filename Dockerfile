# /Dockerfile
FROM rust:1.97-bookworm AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY apps/registry/back-end/Cargo.toml apps/registry/back-end/Cargo.toml

# Cache dependency compilation before copying application source.
RUN mkdir -p apps/registry/back-end/src \
    && printf 'fn main() {}\n' > apps/registry/back-end/src/main.rs \
    && cargo build --release --package fpm-registry

COPY apps/registry/back-end/src apps/registry/back-end/src
RUN touch apps/registry/back-end/src/main.rs \
    && cargo build --release --package fpm-registry

FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install --no-install-recommends -y ca-certificates libssl3 \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --uid 10001 --create-home --shell /usr/sbin/nologin fpm

COPY --from=builder /app/target/release/fpm-registry /usr/local/bin/fpm-registry

USER fpm
EXPOSE 6011
ENTRYPOINT ["/usr/local/bin/fpm-registry"]
