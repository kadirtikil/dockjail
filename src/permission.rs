//! Accessibility permission, required for the event tap and for reading the Dock's position.

use core_foundation::base::TCFType;
use core_foundation::boolean::CFBoolean;
use core_foundation::dictionary::CFDictionary;
use core_foundation::string::CFString;

use crate::ffi;

/// Checks the permission and lets macOS show its "grant access" dialog if it is missing.
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
