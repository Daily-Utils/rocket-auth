# Build Stage
FROM debian:latest AS build

ARG pkg=rocket-auth

RUN apt-get update && \
    apt-get install -y curl build-essential libssl-dev libmariadb-dev-compat libmariadb-dev pkg-config && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    . "$HOME/.cargo/env" && \
    rustup default stable

ENV PATH="/root/.cargo/bin:${PATH}"

RUN cargo install diesel_cli --no-default-features --features mysql

WORKDIR /build

COPY . .

RUN --mount=type=cache,target=/build/target \
    --mount=type=cache,target=/root/.cargo/registry \
    --mount=type=cache,target=/root/.cargo/git \
    set -eux; \
    cargo build --release; \
    objcopy --compress-debug-sections target/release/$pkg ./main

# Run Stage
FROM debian:latest

WORKDIR /app

COPY --from=build /root/.cargo/bin/diesel /usr/local/bin/diesel

COPY --from=build /build/main ./
COPY --from=build /build/Rocket.toml ./Rocket.toml
COPY --from=build /build/entrypoint.sh ./entrypoint.sh
COPY --from=build /build/diesel.toml ./diesel.toml
COPY --from=build /build/migrations ./migrations

RUN touch .env

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8888

# Install dependencies for the application
RUN apt-get update && apt-get install -y libmariadb-dev-compat libmariadb-dev

ENV MYSQLCLIENT_INCLUDE_DIR=/usr/include/mariadb
ENV MYSQLCLIENT_LIB_DIR=/usr/lib/x86_64-linux-gnu
ENV MYSQLCLIENT_VERSION=10.5.9

# Run Diesel migrations and then start the app
CMD ["./entrypoint.sh"]
