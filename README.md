<p align="center">
  <img src="assets/dockjail.png" alt="dockjail icon" width="320">
</p>

# dockjail

Keeps the macOS Dock locked to one display in multi-monitor setups.

macOS moves the Dock to whichever display the cursor touches at its bottom edge. dockjail installs
a global event tap and drops mouse movements in the bottom 10 points of every display except the
locked one, so the Dock never sees them and stays where it is.

## Quick start

1. **Install** with Homebrew. The binary is universal (Apple Silicon and Intel). Recent Homebrew
   versions only load formulae from third-party taps after you trust them.

   ```bash
   brew tap mehmetmuejde/dockjail https://github.com/mehmetmuejde/dockjail
   brew trust mehmetmuejde/dockjail
   brew install dockjail
   ```

2. **Allow your terminal** to use the Accessibility API. Run the check; if the permission is
   missing, macOS shows a prompt. Enable your terminal app in System Settings → Privacy & Security →
   Accessibility, then run the check again until it reports the permission as granted. `dockjail on`
   never asks for the permission itself and refuses to lock the Dock until the check passes.

   ```bash
   dockjail check
   ```

3. **Pick the display.** Drag the Dock to the display it should stay on, then lock it there. To pick
   a display by its index instead, list them first.

   ```bash
   dockjail on            # the display the Dock is on now
   dockjail displays      # list displays with index, UUID, frame and where the Dock is
   dockjail on 1          # the display with index 1
   ```

4. **Allow the login agent.** `dockjail on` starts the lock and registers a LaunchAgent that
   starts it at every login. The agent is a separate program without a terminal and needs its own
   entry. `dockjail on` checks after a few seconds whether the agent is running. If not, it opens
   the Accessibility page: enable `dockjail` there (if it is missing from the list, add the binary
   from `which dockjail`; Cmd+Shift+G lets you type the path). The agent retries every 30 seconds,
   so it starts shortly after.

5. **Check that it works.** Move the cursor to the bottom edge of another display: the Dock stays.
   `pgrep -x dockjail` prints a process id while the lock is running.

## Commands

| Command | What it does |
|---------|--------------|
| `dockjail check` | Checks the Accessibility permission and asks for it if missing |
| `dockjail displays` | Lists displays with index, UUID, frame, and which one shows the Dock |
| `dockjail on [INDEX]` | Locks the Dock to a display and starts at every login |
| `dockjail off` | Unlocks the Dock and stops starting at login |
| `dockjail log [-n LINES]` | Prints the last lines (default 50) of the login agent's log |

The agent stores the display's hardware UUID, so the lock survives reboots and cable or port
changes. The index is only used to pick the display.

## Update

```bash
brew upgrade dockjail
dockjail on
```

macOS ties the Accessibility permission to the exact binary, so it asks again after an update.
Remove the old `dockjail` entry from the list, add the binary again and enable it.

## Uninstall

```bash
dockjail off
brew uninstall dockjail
```

Run `dockjail off` first: it removes the LaunchAgent. Without it the agent keeps trying to start a
binary that is gone. Remove the `dockjail` entry from the Accessibility list yourself.

## Behavior

- The Dock stays on the locked display while that display is connected. While it is disconnected,
  nothing is blocked.
- dockjail never moves the Dock itself. If macOS moves it, for example after waking from sleep,
  drag it back by pushing the cursor against the bottom edge of the locked display.
- Only a Dock at the bottom of the screen is supported.
- Hot corners in the bottom corners of the other displays do not fire, because those mouse
  movements are dropped.
- The agent logs to `~/Library/Logs/dockjail.log` (read it with `dockjail log`) and restarts itself
  if it fails, for example while the permission is still missing.

## Build from source

```bash
cargo build --release
cargo test
```

Rust 1.85 or newer. `cargo install --path .` installs the binary to `~/.cargo/bin`; every rebuild
is a new binary for macOS and needs the permission again.

## Releases

Pushing a tag such as `v0.1.1` that matches the version in `Cargo.toml` builds the universal binary,
creates the GitHub release and updates `Formula/dockjail.rb` on `main`.

## License

[MIT](LICENSE)
