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

Start with **T01** in the ticket tracker unless told otherwise.

## Prerequisites

- Rust toolchain (`cargo`, `rustc`)
- Docker (for `game_engine` matches)
- Official `docker_image` zip from [01-edu filler assets](https://assets.01-edu.org/filler/filler.zip)

## Quick start

```bash
# Build robot (once src/ exists)
cargo build --release
mkdir -p solution
cp target/release/filler solution/

# Unit tests (host)
cargo test
cargo clippy
cargo fmt --check
```

### Docker (engine)

```bash
cd docker_image   # extracted from filler.zip
docker build -t filler .
docker run -v "$(pwd)/../solution":/filler/solution -it filler
```

Inside the container:

```bash
./game_engine -f maps/map01 -p1 /filler/solution/filler -p2 robots/bender
```

## Source briefs

Read-only stakeholder text:

- [`docs/raw/REQUIREMENTS-SOURCE.md`](docs/raw/REQUIREMENTS-SOURCE.md)
- [`docs/raw/AUDIT-SOURCE.md`](docs/raw/AUDIT-SOURCE.md)

Derived agent docs (`docs/requirements.md`, `docs/audit.md`) trace REQ/AUD IDs to these sources.

## License

See [LICENSE](LICENSE).
