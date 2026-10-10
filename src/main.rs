mod display;
mod dock;
mod ffi;
mod launch_agent;
mod permission;
mod tap;

use std::process::ExitCode;

use clap::{Parser, Subcommand};

/// Keeps the macOS Dock locked to the display it is on.
#[derive(Parser)]
#[command(name = "dockjail", version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// List connected displays with the index to use for `on`
    Displays,
    /// Check the accessibility permission and ask for it if missing
    Check,
    /// Lock the Dock to a display, immediately and at every login
    On {
        /// Index from `dockjail displays`; default: the display the Dock is on now
        index: Option<usize>,
    },
    /// Unlock the Dock and stop locking at login
    Off,
    /// Lock the Dock to the given display in the foreground (used by the login item)
    #[command(hide = true)]
    Run {
        /// UUID of the display the Dock stays on
        display: String,
    },
}

fn main() -> ExitCode {
    let result = match Cli::parse().command {
        Command::Displays => displays(),
        Command::Check => check(),
        Command::On { index } => on(index),
        Command::Off => off(),
        Command::Run { display } => run(display),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::FAILURE
        }
    }
}

fn require_permission() -> Result<(), String> {
    if permission::request() {
        return Ok(());
    }
    Err(format!("accessibility permission missing.\n{}", permission::HINT))
}

fn check() -> Result<(), String> {
    require_permission()?;
    if let Ok(exe) = std::env::current_exe() {
        println!("accessibility permission granted for the program running {}", exe.display());
    }
    Ok(())
}

fn displays() -> Result<(), String> {
    let dock = dock::current_display();

    println!("{:<6} {:<38} {:<22} TYPE", "INDEX", "UUID", "FRAME");
    for (index, d) in display::list().iter().enumerate() {
        let mut kind = vec![if d.is_builtin { "built-in" } else { "external" }];
        if d.is_main {
            kind.push("main");
        }
        if dock == Some(d.id) {
            kind.push("dock");
        }
        let (origin, size) = (d.frame.origin, d.frame.size);
        let frame = format!("{},{} {}x{}", origin.x, origin.y, size.width, size.height);
        println!("{:<6} {:<38} {:<22} {}", index, d.uuid, frame, kind.join(", "));
    }
    Ok(())
}

fn on(index: Option<usize>) -> Result<(), String> {
    require_permission()?;
    let uuid = match index {
        Some(index) => display::list()
            .get(index)
            .map(|d| d.uuid.clone())
            .ok_or_else(|| format!("no display with index {index}, see `dockjail displays`"))?,
        None => display::uuid_of(dock::current_display().ok_or("cannot determine the display the Dock is on")?),
    };

    let path = launch_agent::install(std::slice::from_ref(&uuid))?;
    println!("Dock locked to display {uuid}");
    println!("installed {}", path.display());
    if let Ok(exe) = std::env::current_exe() {
        println!("grant the accessibility permission to {} if macOS asks for it", exe.display());
    }
    Ok(())
}

fn off() -> Result<(), String> {
    println!("{}", if launch_agent::uninstall()? { "Dock unlocked" } else { "Dock was not locked" });
    Ok(())
}

fn run(display: String) -> Result<(), String> {
    require_permission()?;
    eprintln!("locking the Dock to display {display}");
    tap::run(display)
}
