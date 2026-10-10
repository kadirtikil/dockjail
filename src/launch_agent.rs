//! Autostart at login through a per-user LaunchAgent.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub const LABEL: &str = "local.dockjail";

fn home() -> Result<PathBuf, String> {
    std::env::var_os("HOME").map(PathBuf::from).ok_or_else(|| "HOME is not set".to_string())
}

fn plist_path() -> Result<PathBuf, String> {
    Ok(home()?.join(format!("Library/LaunchAgents/{LABEL}.plist")))
}

/// The file the agent's output goes to.
pub fn log_path() -> Result<PathBuf, String> {
    Ok(home()?.join("Library/Logs/dockjail.log"))
}

/// launchd domain of the current user's GUI session.
fn domain() -> String {
    format!("gui/{}", unsafe { libc::getuid() })
}

/// Writes the plist for `dockjail run <run_args>` and loads it.
pub fn install(run_args: &[String]) -> Result<PathBuf, String> {
    // Deliberately not resolved: a symlink such as Homebrew's /opt/homebrew/bin/dockjail survives
    // upgrades, the versioned path it points to does not.
    let exe = std::env::current_exe().map_err(|e| format!("cannot determine own path: {e}"))?;
    let mut arguments = vec![exe.display().to_string(), "run".to_string()];
    arguments.extend(run_args.iter().cloned());

    let plist = render_plist(&arguments, &log_path()?.display().to_string());

    let path = plist_path()?;
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).map_err(|e| format!("cannot create {}: {e}", dir.display()))?;
    }

    // Unload an older version first; failing here only means it was not loaded.
    let _ = launchctl(&["bootout", &format!("{}/{LABEL}", domain())]);
    fs::write(&path, plist).map_err(|e| format!("cannot write {}: {e}", path.display()))?;
    launchctl(&["bootstrap", &domain(), &path.display().to_string()])?;

    Ok(path)
}

/// Stops the agent and removes its plist. Returns false if nothing was installed.
pub fn uninstall() -> Result<bool, String> {
    let path = plist_path()?;
    if !path.exists() {
        return Ok(false);
    }
    let _ = launchctl(&["bootout", &format!("{}/{LABEL}", domain())]);
    fs::remove_file(&path).map_err(|e| format!("cannot remove {}: {e}", path.display()))?;
    Ok(true)
}

fn launchctl(args: &[&str]) -> Result<(), String> {
    let output =
        Command::new("/bin/launchctl").args(args).output().map_err(|e| format!("cannot run launchctl: {e}"))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(format!("launchctl {} failed: {}", args.join(" "), String::from_utf8_lossy(&output.stderr).trim()))
    }
}

/// Restarts on failure (e.g. missing permission) but not after a clean exit.
fn render_plist(arguments: &[String], log: &str) -> String {
    let arguments: String = arguments.iter().map(|a| format!("\n        <string>{}</string>", escape(a))).collect();
    let log = escape(log);

    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>{LABEL}</string>
    <key>ProgramArguments</key>
    <array>{arguments}
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <dict>
        <key>SuccessfulExit</key>
        <false/>
    </dict>
    <key>ThrottleInterval</key>
    <integer>30</integer>
    <key>ProcessType</key>
    <string>Interactive</string>
    <key>StandardOutPath</key>
    <string>{log}</string>
    <key>StandardErrorPath</key>
    <string>{log}</string>
</dict>
</plist>
"#
    )
}

fn escape(value: &str) -> String {
    value.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plist_contains_escaped_arguments() {
        let args = vec!["/usr/local/bin/dockjail".to_string(), "run".into(), "--display".into(), "a&b".into()];
        let plist = render_plist(&args, "/tmp/dockjail.log");
        assert!(plist.contains("<string>local.dockjail</string>"));
        assert!(plist.contains("<string>run</string>"));
        assert!(plist.contains("<string>a&amp;b</string>"));
        assert!(plist.contains("<string>/tmp/dockjail.log</string>"));
    }
}
