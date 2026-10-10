//! Locating the Dock through the Accessibility API.

use std::process::Command;

use core_foundation::base::{CFType, TCFType};
use core_foundation::string::CFString;
use core_foundation_sys::array::{CFArrayGetCount, CFArrayGetTypeID, CFArrayGetValueAtIndex};
use core_foundation_sys::base::{CFGetTypeID, CFTypeRef};
use core_graphics::geometry::CGPoint;

use crate::{display, ffi};

/// The display the Dock is currently shown on, if it can be determined.
pub fn current_display() -> Option<u32> {
    display::at(probe_point()?)
}

/// A point just above the top-left corner of the Dock's bar, in global coordinates. Unlike the bar
/// itself, it lies inside the Dock's display even when the bar is hidden below the bottom edge.
fn probe_point() -> Option<CGPoint> {
    let pid = pid()?;

    unsafe {
        let app = ffi::AXUIElementCreateApplication(pid);
        if app.is_null() {
            return None;
        }
        let app = CFType::wrap_under_create_rule(app);

        let children = copy_attribute(app.as_CFTypeRef(), "AXChildren")?;
        let children = children.as_CFTypeRef();
        if CFGetTypeID(children) != CFArrayGetTypeID() {
            return None;
        }

        for i in 0..CFArrayGetCount(children.cast()) {
            let child = CFArrayGetValueAtIndex(children.cast(), i);
            if role(child).as_deref() != Some("AXList") {
                continue;
            }

            let mut position = CGPoint::default();
            let position_value = copy_attribute(child, "AXPosition")?;
            let ok = ffi::AXValueGetValue(
                position_value.as_CFTypeRef(),
                ffi::kAXValueTypeCGPoint,
                (&raw mut position).cast(),
            ) != 0;

            if ok {
                return Some(CGPoint::new(position.x, position.y - 1.0));
            }
        }
    }

    None
}

unsafe fn copy_attribute(element: CFTypeRef, name: &str) -> Option<CFType> {
    let name = CFString::new(name);
    let mut value: CFTypeRef = std::ptr::null();
    let err = unsafe { ffi::AXUIElementCopyAttributeValue(element, name.as_concrete_TypeRef(), &mut value) };
    if err != ffi::kAXErrorSuccess || value.is_null() {
        return None;
    }
    Some(unsafe { CFType::wrap_under_create_rule(value) })
}

unsafe fn role(element: CFTypeRef) -> Option<String> {
    let value = unsafe { copy_attribute(element, "AXRole")? };
    value.downcast::<CFString>().map(|s| s.to_string())
}

/// PID of the current user's Dock process.
fn pid() -> Option<libc::pid_t> {
    let uid = unsafe { libc::getuid() };
    let output = Command::new("/usr/bin/pgrep").args(["-x", "-U", &uid.to_string(), "Dock"]).output().ok()?;
    String::from_utf8_lossy(&output.stdout).lines().next()?.trim().parse().ok()
}
