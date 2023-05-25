use crate::core_foundation_sys::array::CFArrayRef;
use crate::core_foundation_sys::base::{Boolean, CFAllocatorRef, CFTypeID, CFTypeRef, OSStatus};
use crate::core_foundation_sys::dictionary::CFDictionaryRef;
use crate::core_foundation_sys::string::CFStringRef;
use crate::libc::{c_void, size_t};

use crate::base::{CMItemCount, CMItemIndex};
use crate::block_buffer::CMBlockBufferRef;
use crate::format_description::CMFormatDescriptionRef;
use crate::sync::CMClockRef;
use crate::time::CMTime;

pub type CMSampleBufferRef = *mut c_void;
pub type CMSampleBufferMakeDataReadyCallback =
    extern "C" fn(sbuf: CMSampleBufferRef, makeDataReadyRefcon: *mut c_void) -> OSStatus;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMSampleTimingInfo {
    pub duration: CMTime,
    pub presentation_time_stamp: CMTime,
    pub decode_time_stamp: CMTime,
}

extern "C" {
    pub fn CMSampleBufferCreate(
        allocator: CFAllocatorRef,
        dataBuffer: CMBlockBufferRef,
        dataReady: bool,
        makeDataReadyCallback: CMSampleBufferMakeDataReadyCallback,
        makeDataReadyRefcon: *mut c_void,
        formatDescription: CMFormatDescriptionRef,
        numSamples: CMItemCount,
        numSampleTimingEntries: CMItemCount,
        sampleTimingArray: *const CMSampleTimingInfo,
        numSampleSizeEntries: CMItemCount,
        sampleSizeArray: *const size_t,
        sampleBufferOut: *mut CMSampleBufferRef,
    ) -> OSStatus;
    pub fn CMSampleBufferGetSampleAttachmentsArray(
        sbuf: CMSampleBufferRef,
        createIfNecessary: bool,
    ) -> CFArrayRef;
    pub fn CMSampleBufferGetSampleTimingInfo(
        sbuf: CMSampleBufferRef,
        sampleIndex: CMItemIndex,
        timingInfoOut: *mut CMSampleTimingInfo,
    ) -> OSStatus;
}
