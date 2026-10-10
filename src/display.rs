//! Looking up displays by position and by hardware UUID.

use core_foundation::base::TCFType;
use core_foundation::string::CFString;
use core_foundation::uuid::CFUUID;
use core_foundation_sys::uuid::CFUUIDCreateString;
use core_graphics::display::CGDisplay;
use core_graphics::geometry::{CGPoint, CGRect};

use crate::ffi;

pub struct Display {
    pub uuid: String,
    pub frame: CGRect,
    pub is_main: bool,
    pub is_builtin: bool,
    pub id: u32,
}

/// All active displays, main display first, the rest ordered left to right.
pub fn list() -> Vec<Display> {
    let mut displays: Vec<Display> = CGDisplay::active_displays()
        .unwrap_or_default()
        .into_iter()
        .map(CGDisplay::new)
        .map(|d| Display {
            uuid: uuid_of(d.id),
            frame: d.bounds(),
            is_main: d.is_main(),
            is_builtin: d.is_builtin(),
            id: d.id,
        })
        .collect();
    displays.sort_by(|a, b| b.is_main.cmp(&a.is_main).then(a.frame.origin.x.total_cmp(&b.frame.origin.x)));
    displays
}

/// The display that contains `point` (global coordinates).
pub fn at(point: CGPoint) -> Option<u32> {
    let (ids, count) = CGDisplay::displays_with_point(point, 1).ok()?;
    ids.first().copied().filter(|_| count > 0)
}

/// The connected display with this UUID, if any.
pub fn with_uuid(uuid: &str) -> Option<u32> {
    CGDisplay::active_displays().ok()?.into_iter().find(|&id| uuid_of(id) == uuid)
}

/// Hardware UUID of a display; stays the same across reboots and port changes.
pub fn uuid_of(id: u32) -> String {
    unsafe {
        let uuid = ffi::CGDisplayCreateUUIDFromDisplayID(id);
        if uuid.is_null() {
            return format!("DISPLAY-{id}");
        }
        let uuid = CFUUID::wrap_under_create_rule(uuid);
        CFString::wrap_under_create_rule(CFUUIDCreateString(std::ptr::null(), uuid.as_concrete_TypeRef())).to_string()
    }
}
