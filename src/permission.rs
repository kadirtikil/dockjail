//! Accessibility permission, required for the event tap and for reading the Dock's position.

use core_foundation::base::TCFType;
use core_foundation::boolean::CFBoolean;
use core_foundation::dictionary::CFDictionary;
use core_foundation::string::CFString;

use crate::ffi;

/// Opens the Accessibility page of System Settings.
pub fn open_settings() {
    let _ = std::process::Command::new("/usr/bin/open")
        .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility")
        .status();
}

/// Checks the permission without showing a dialog.
pub fn is_trusted() -> bool {
    unsafe { ffi::AXIsProcessTrusted() != 0 }
}

/// Checks the permission and lets macOS show its "grant access" dialog if it is missing.
/// The dialog does not block: the answer is the state at the time of the call.
pub fn request() -> bool {
    unsafe {
        let key = CFString::wrap_under_get_rule(ffi::kAXTrustedCheckOptionPrompt);
        let options = CFDictionary::from_CFType_pairs(&[(key, CFBoolean::true_value())]);
        ffi::AXIsProcessTrustedWithOptions(options.as_concrete_TypeRef()) != 0
    }
}

pub const HINT: &str = "Grant access in System Settings → Privacy & Security → Accessibility.\n\
    When started from a terminal, the terminal app needs the permission; \
    as a LaunchAgent, the dockjail binary itself needs it.";
