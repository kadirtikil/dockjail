//! The global event tap that keeps the cursor from triggering the Dock on foreign displays.

use std::ffi::c_void;
use std::sync::atomic::{AtomicPtr, Ordering};

use core_foundation::base::TCFType;
use core_foundation::runloop::{CFRunLoop, kCFRunLoopCommonModes};
use core_graphics::display::CGDisplay;
use core_graphics::event::{
    CGEvent, CGEventTap, CGEventTapLocation, CGEventTapOptions, CGEventTapPlacement, CGEventType, CallbackResult,
};
use core_graphics::geometry::CGRect;

use crate::{display, ffi};

/// Height of the strip at the bottom edge that makes the Dock switch displays.
const TRIGGER_ZONE_HEIGHT: f64 = 10.0;

/// Raw `CFMachPortRef` of the tap, needed to re-enable it from its own callback.
static TAP_PORT: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());

/// Drops mouse movements in the bottom strip of every display except the one with `anchor_uuid`.
/// Blocks until the process is stopped.
pub fn run(anchor_uuid: String) -> Result<(), String> {
    let tap = CGEventTap::new(
        CGEventTapLocation::Session,
        CGEventTapPlacement::HeadInsertEventTap,
        CGEventTapOptions::Default,
        vec![CGEventType::MouseMoved],
        move |_proxy, event_type, event| handle(&anchor_uuid, event_type, event),
    )
    .map_err(|_| "cannot create event tap (accessibility permission missing or stale?)".to_string())?;

    TAP_PORT.store(tap.mach_port().as_concrete_TypeRef().cast(), Ordering::SeqCst);

    let source = tap
        .mach_port()
        .create_runloop_source(0)
        .map_err(|_| "cannot create run loop source for event tap".to_string())?;
    CFRunLoop::get_current().add_source(&source, unsafe { kCFRunLoopCommonModes });
    tap.enable();

    CFRunLoop::run_current();
    Ok(())
}

/// Runs for every mouse movement, so it must stay fast: macOS disables taps that respond slowly.
/// The cheap checks come first; the UUID lookup only happens inside a bottom strip.
fn handle(anchor_uuid: &str, event_type: CGEventType, event: &CGEvent) -> CallbackResult {
    match event_type {
        CGEventType::TapDisabledByTimeout | CGEventType::TapDisabledByUserInput => {
            let port = TAP_PORT.load(Ordering::SeqCst);
            if !port.is_null() {
                unsafe { ffi::CGEventTapEnable(port.cast(), true) };
                eprintln!("event tap was disabled by the system, re-enabled");
            }
            return CallbackResult::Keep;
        }
        CGEventType::MouseMoved => {}
        _ => return CallbackResult::Keep,
    }

    let location = event.location();
    let Some(id) = display::at(location) else {
        return CallbackResult::Keep;
    };
    if !in_trigger_strip(location.y, CGDisplay::new(id).bounds()) {
        return CallbackResult::Keep;
    }

    // While the anchor display is not connected, nothing is blocked.
    match display::with_uuid(anchor_uuid) {
        Some(anchor) if anchor != id => CallbackResult::Drop,
        _ => CallbackResult::Keep,
    }
}

fn in_trigger_strip(y: f64, display: CGRect) -> bool {
    y >= display.origin.y + display.size.height - TRIGGER_ZONE_HEIGHT
}

#[cfg(test)]
mod tests {
    use super::*;
    use core_graphics::geometry::{CGPoint, CGSize};

    #[test]
    fn trigger_zone_is_ten_points_high() {
        let display = CGRect::new(&CGPoint::new(1512.0, 0.0), &CGSize::new(2560.0, 1440.0));
        assert!(in_trigger_strip(1439.0, display));
        assert!(in_trigger_strip(1430.0, display));
        assert!(!in_trigger_strip(1429.9, display));
        assert!(!in_trigger_strip(700.0, display));
    }
}
