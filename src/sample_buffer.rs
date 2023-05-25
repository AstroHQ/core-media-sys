use crate::core_foundation_sys::array::CFArrayRef;
use crate::core_foundation_sys::base::{Boolean, CFAllocatorRef, CFTypeID, CFTypeRef, OSStatus};
use crate::core_foundation_sys::dictionary::CFDictionaryRef;
use crate::core_foundation_sys::string::CFStringRef;
use crate::libc::{c_void, size_t};
use core_video_sys::CVImageBufferRef;

use crate::base::{CMItemCount, CMItemIndex};
use crate::block_buffer::CMBlockBufferRef;
use crate::format_description::{CMFormatDescriptionRef, CMVideoFormatDescriptionRef};
use crate::sync::CMClockRef;
use crate::time::CMTime;

pub const kCMSampleBufferError_AllocationFailed: OSStatus				= -12730;
pub const kCMSampleBufferError_RequiredParameterMissing: OSStatus		= -12731;
pub const kCMSampleBufferError_AlreadyHasDataBuffer: OSStatus			= -12732;
pub const kCMSampleBufferError_BufferNotReady: OSStatus					= -12733;
pub const kCMSampleBufferError_SampleIndexOutOfRange: OSStatus			= -12734;
pub const kCMSampleBufferError_BufferHasNoSampleSizes: OSStatus			= -12735;
pub const kCMSampleBufferError_BufferHasNoSampleTimingInfo: OSStatus	= -12736;
pub const kCMSampleBufferError_ArrayTooSmall: OSStatus					= -12737;
pub const kCMSampleBufferError_InvalidEntryCount: OSStatus				= -12738;
pub const kCMSampleBufferError_CannotSubdivide: OSStatus				= -12739;
pub const kCMSampleBufferError_SampleTimingInfoInvalid: OSStatus		= -12740;
pub const kCMSampleBufferError_InvalidMediaTypeForOperation: OSStatus	= -12741;
pub const kCMSampleBufferError_InvalidSampleData: OSStatus				= -12742;
pub const kCMSampleBufferError_InvalidMediaFormat: OSStatus				= -12743;
pub const kCMSampleBufferError_Invalidated: OSStatus					= -12744;
pub const kCMSampleBufferError_DataFailed: OSStatus						= -16750;
pub const kCMSampleBufferError_DataCanceled: OSStatus					= -16751;

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
    pub static kCMSampleAttachmentKey_NotSync: CFStringRef;
    pub static kCMSampleAttachmentKey_PartialSync: CFStringRef;
    pub static kCMSampleAttachmentKey_HasRedundantCoding: CFStringRef;
    pub static kCMSampleAttachmentKey_IsDependedOnByOthers: CFStringRef;
    pub static kCMSampleAttachmentKey_DependsOnOthers: CFStringRef;
    pub static kCMSampleAttachmentKey_EarlierDisplayTimesAllowed: CFStringRef;
    pub static kCMSampleAttachmentKey_DisplayImmediately: CFStringRef;
    pub static kCMSampleAttachmentKey_DoNotDisplay: CFStringRef;
    pub static kCMSampleBufferAttachmentKey_ResetDecoderBeforeDecoding: CFStringRef;
    pub static kCMSampleBufferAttachmentKey_DrainAfterDecoding: CFStringRef;
    pub static kCMSampleBufferAttachmentKey_PostNotificationWhenConsumed: CFStringRef;
    pub static kCMSampleBufferAttachmentKey_ResumeOutput: CFStringRef;

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
    pub fn CMSampleBufferCreateForImageBuffer(
        allocator: CFAllocatorRef,
        imageBuffer: CVImageBufferRef,
        dataReady: bool,
        makeDataReadyCallback: CMSampleBufferMakeDataReadyCallback,
        makeDataReadyRefcon: *mut c_void,
        formatDescription: CMVideoFormatDescriptionRef,
        sampleTiming: *const CMSampleTimingInfo,
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
    pub fn CMSampleBufferGetFormatDescription(sbuf: CMSampleBufferRef) -> CMFormatDescriptionRef;
    pub fn CMSampleBufferGetDataBuffer(sbuf: CMSampleBufferRef) -> CMBlockBufferRef;
}
