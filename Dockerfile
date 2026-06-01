# build frontend into /app/frontend/dist
FROM node:26 AS build-frontend
COPY frontend /app/frontend
WORKDIR /app/frontend
RUN --mount=type=cache,target=/app/frontend/node_modules \
    npm install
RUN --mount=type=cache,target=/app/frontend/node_modules \
    npm run build

# output backend binary in /app/web
FROM rust:1.96 AS build-backend
COPY backend /app/backend
WORKDIR /app/backend
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/backend/target \
    cargo build --release --bin web && \
    cp target/release/web /app/web

FROM debian:stable-slim
EXPOSE 8000
HEALTHCHECK --interval=1m --timeout=3s --start-interval=1s --start-period=30s \
    CMD curl --fail http://127.0.0.1:8000/ || exit 1
RUN apt update && apt install -y curl && rm -rf /var/lib/apt/lists/*
COPY --from=build-backend /app/web /app/
COPY --from=build-frontend /app/frontend/dist /app/dist

WORKDIR /app
ENV RUST_LOG=info
ENV ROCKET_LOG_LEVEL=normal
ENV ROCKET_ADDRESS=0.0.0.0
CMD ["/app/web"]
