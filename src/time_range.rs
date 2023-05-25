use crate::core_foundation_sys::base::{Boolean, CFAllocatorRef, CFTypeID, CFTypeRef, OSStatus};
use crate::core_foundation_sys::dictionary::CFDictionaryRef;
use crate::core_foundation_sys::string::CFStringRef;

use crate::sync::CMClockRef;
use crate::time::CMTime;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMTimeRange {
    pub start: CMTime,
    pub duration: CMTime,
}
