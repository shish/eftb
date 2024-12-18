# output backend code in /app/target
FROM rust:1.81 AS build-backend
COPY Cargo.toml Cargo.lock /app/
WORKDIR /app
RUN mkdir backend && \
    echo "fn main() {println!(\"stub\")}" > /app/backend/web.rs && \
    cargo build --release --bin web && \
    rm -rf backend target/release/deps/web-*
COPY backend /app/backend
RUN cargo build --release --bin web

FROM debian:stable-slim
EXPOSE 8000
#HEALTHCHECK --interval=1m --timeout=3s CMD curl --fail http://127.0.0.1:8074/ || exit 1
#RUN apt update && apt install -y curl && rm -rf /var/lib/apt/lists/*
COPY --from=build-backend /app/target/release/web /app/
COPY assets /app/assets
COPY data /app/data

WORKDIR /app
ENV RUST_LOG=info
ENV ROCKET_LOG_LEVEL=normal
ENV ROCKET_ADDRESS=0.0.0.0
CMD ["/app/web"]
