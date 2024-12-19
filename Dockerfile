# build frontend into /app/dist
FROM alpine:latest AS build-frontend
RUN apk add --no-cache nodejs npm
COPY . /app
WORKDIR /app
RUN --mount=type=cache,target=/app/node_modules \
    npm install
RUN --mount=type=cache,target=/app/node_modules \
    npm run build

# output backend code in /app/web
FROM rust:1.81 AS build-backend
COPY . /app
WORKDIR /app
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release --bin web && \
    cp target/release/web /app/web

FROM debian:stable-slim
EXPOSE 8000
#HEALTHCHECK --interval=1m --timeout=3s CMD curl --fail http://127.0.0.1:8074/ || exit 1
#RUN apt update && apt install -y curl && rm -rf /var/lib/apt/lists/*
COPY --from=build-backend /app/web /app/
COPY --from=build-frontend /app/dist /app/dist

WORKDIR /app
ENV RUST_LOG=info
ENV ROCKET_LOG_LEVEL=normal
ENV ROCKET_ADDRESS=0.0.0.0
CMD ["/app/web"]
