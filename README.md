# filler

01-edu **Filler** robot — a Rust binary that plays the territory-capture game via stdin/stdout against the official `game_engine`.

## Agent workflow

This repo uses a requirements-first, audit-first agent setup:

| Doc | Purpose |
|-----|---------|
| [`AGENTS.md`](AGENTS.md) | Coding standards and commands |
| [`docs/requirements.md`](docs/requirements.md) | REQ IDs (stakeholder spec) |
| [`docs/audit.md`](docs/audit.md) | AUD IDs (acceptance gate) |
| [`docs/ticket-tracker.md`](docs/ticket-tracker.md) | Current work queue |

Start with **T10** in the ticket tracker (Sprint 0 complete).

## Prerequisites

- Rust toolchain (`cargo`, `rustc`)
- Docker (`docker.io` on WSL, or Docker Desktop with WSL integration)
- Official `docker_image` zip from [01-edu filler assets](https://assets.01-edu.org/filler/filler.zip)

## Build and test (host)

```bash
cargo build --release
cargo test
cargo clippy -- -D warnings
cargo fmt --check
```

Copy the release binary into `solution/` for engine matches:

```bash
cp target/release/filler solution/
chmod +x solution/filler
```

## Docker setup (`docker_image/`)

The engine zip is **not** in git. Extract it locally:

```bash
cd ~/filler
unzip /path/to/filler.zip   # creates docker_image/
```

Build and run the container from `docker_image/`:

```bash
cd docker_image
chmod +x linux_game_engine m1_game_engine
chmod +x linux_robots/* m1_robots/*
docker build -t filler .
docker run -v "$(pwd)/../solution":/filler/solution -it filler
```

Mount notes:

- Host `filler/solution/` → container `/filler/solution/`
- Run `docker run` from `docker_image/` so `../solution` resolves correctly
- `docker_image/` is listed in `.gitignore` (large binaries)

## Smoke test (inside container)

At the `/filler#` prompt:

```bash
# Reference robots only (AUD-1)
./linux_game_engine -f maps/map01 -p1 linux_robots/bender -p2 linux_robots/terminator

# Student player (after T12 + release build copied to solution/)
./linux_game_engine -f maps/map01 -p1 solution/filler -p2 linux_robots/bender
```

Use `linux_game_engine` and `linux_robots/` on Linux/WSL. On M1 Mac use `m1_game_engine` and `m1_robots/` (see `docker_image/README.md`).

## Source briefs

Read-only stakeholder text:

- [`docs/raw/REQUIREMENTS-SOURCE.md`](docs/raw/REQUIREMENTS-SOURCE.md)
- [`docs/raw/AUDIT-SOURCE.md`](docs/raw/AUDIT-SOURCE.md)

## License

See [LICENSE](LICENSE).
