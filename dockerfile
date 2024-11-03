# Build stage
FROM docker.io/rust:1-slim-bookworm AS build

ARG pkg=rocket-auth

WORKDIR /build

COPY . .

RUN --mount=type=cache,target=/build/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    set -eux; \
    cargo build --release; \
    objcopy --compress-debug-sections target/release/$pkg ./main

# Run Stage
FROM docker.io/debian:bookworm-slim

WORKDIR /app

COPY --from=build /build/main ./

COPY --from=build /build/Rocket.toml ./Rocket.toml
COPY --from=build /build/static ./static
COPY --from=build /build/templates ./templates

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080

CMD ["./main"]