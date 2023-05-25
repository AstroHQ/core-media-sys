use crate::core_foundation_sys::base::{Boolean, CFAllocatorRef, CFTypeID, CFTypeRef, OSStatus};
use crate::core_foundation_sys::dictionary::CFDictionaryRef;
use crate::core_foundation_sys::string::{CFStringEncoding, CFStringRef};
use crate::libc::size_t;

use crate::block_buffer::CMBlockBufferRef;
use crate::format_description::CMVideoFormatDescriptionRef;
use crate::sync::CMClockRef;

pub const kCMFormatDescriptionBridgeError_InvalidParameter: OSStatus = -12712;
pub const kCMFormatDescriptionBridgeError_AllocationFailed: OSStatus = -12713;
pub const kCMFormatDescriptionBridgeError_InvalidSerializedSampleDescription: OSStatus = -12714;
pub const kCMFormatDescriptionBridgeError_InvalidFormatDescription: OSStatus = -12715;
pub const kCMFormatDescriptionBridgeError_IncompatibleFormatDescription: OSStatus = -12716;
pub const kCMFormatDescriptionBridgeError_UnsupportedSampleDescriptionFlavor: OSStatus = -12717;
pub const kCMFormatDescriptionBridgeError_InvalidSlice: OSStatus = -12719;

extern "C" {
    pub static kCMImageDescriptionFlavor_QuickTimeMovie: CFStringRef;
    pub static kCMImageDescriptionFlavor_ISOFamily: CFStringRef;
    pub static kCMImageDescriptionFlavor_3GPFamily: CFStringRef;

    pub fn CMVideoFormatDescriptionCreateFromH264ParameterSets(
        allocator: CFAllocatorRef,
        parameter_set_count: size_t,
        parameter_set_pointers: *const *const u8,
        parameter_set_sizes: *const size_t,
        nal_unit_header_length: i32,
        format_description_out: *mut CMVideoFormatDescriptionRef,
    ) -> OSStatus;
    pub fn CMVideoFormatDescriptionGetH264ParameterSetAtIndex(
        videoDesc: CMVideoFormatDescriptionRef,
        parameterSetIndex: size_t,
        parameterSetPointerOut: *mut *const u8,
        parameterSetSizeOut: *mut size_t,
        parameterSetCountOut: *mut size_t,
        NALUnitHeaderLengthOut: *mut i32,
    ) -> OSStatus;
    pub fn CMVideoFormatDescriptionCreateFromBigEndianImageDescriptionData(
        allocator: CFAllocatorRef,
        imageDescriptionData: *const u8,
        imageDescriptionSize: size_t,
        imageDescriptionStringEncoding: CFStringEncoding,
        imageDescriptionFlavor: CFStringRef,
        videoFormatDescriptionOut: *mut CMVideoFormatDescriptionRef,
    ) -> OSStatus;
    pub fn CMVideoFormatDescriptionCreateFromBigEndianImageDescriptionBlockBuffer(
        allocator: CFAllocatorRef,
        imageDescriptionBlockBuffer: CMBlockBufferRef,
        imageDescriptionStringEncoding: CFStringEncoding,
        imageDescriptionFlavor: CFStringRef,
        videoFormatDescriptionOut: *mut CMVideoFormatDescriptionRef,
    ) -> OSStatus;

}
