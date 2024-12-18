install-deps:
    @echo "Installing dependencies"
    cargo install

install-cargo-watch:
    @echo "Installing cargo-watch..."
    cargo install cargo-watch

start-db:
    @echo "Starting database"
    docker compose -f dev/docker-compose.yaml up -d db

build:
    @echo "Building application"
    cargo build

run:
    @echo "Running application"
    cargo run

stop-db:
    @echo "Stopping database"
    docker compose -f dev/docker-compose.yaml down

start-prod:
    @echo "Starting production server"
    cargo run --release

start-everything:
    @echo "Starting everything"
    just start-db
    sleep 5
    just run

start-everything-prod:
    @echo "Starting everything in production"
    just start-db
    sleep 5
    just start-prod

generate-migration:
    @echo "Generating migration"
    just start-db
    sleep 5
    diesel migration run

only-generate-migration:
    @echo "Generating migration"
    diesel migration run

run-compose:
    @echo "Running docker compose"
    docker compose -f docker-compose.yaml up -d

watch:
    @echo "Starting live reload..."
    cargo watch -x 'run'