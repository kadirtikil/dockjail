# CLAUDE.md

## Project context

dockjail is a small macOS command line tool that keeps the Dock on one display. A global
event tap swallows the mouse movement that would make the Dock jump to another display.
`dockjail on` installs a per-user LaunchAgent so the lock is restored at every login; its
output goes to `~/Library/Logs/dockjail.log` (`dockjail log` prints the last lines).
Distributed through a Homebrew tap (`Formula/dockjail.rb`).

## Stack

- Rust, edition 2024, single binary crate
- `clap` (CLI), `core-graphics` / `core-foundation` / `libc` (macOS APIs)
- GitHub Actions for CI and releases

## Conventions

- Format with `cargo fmt` (`rustfmt.toml`: max width 120)
- Log through `logging::log!` so every line carries a UTC timestamp, never a bare `eprintln!`
- Commit messages follow Conventional Commits: `<type>(<scope>): <subject>`
