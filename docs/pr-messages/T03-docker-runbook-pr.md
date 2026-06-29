# PR Implementation Report: T03

## Summary

Docker runbook added to README: zip extraction, build/run commands, volume mount path, and smoke-test commands using `linux_game_engine` / `linux_robots/`. AUD-1 verified manually (bender vs terminator match completed).

## Key Changes

- **README.md**: full Docker section with mount notes and smoke commands
- **AGENTS.md**: corrected engine binary paths

## Verification Results

### Manual Audit

- [x] **AUD-1**: Pass — `docker build -t filler .` and `./linux_game_engine -f maps/map01 -p1 linux_robots/bender -p2 linux_robots/terminator` run successfully

## Requirements Traceability

- [x] **REQ-8**: Docker setup documented

## Next Steps

T10 — input parsing
