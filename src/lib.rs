#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    improper_ctypes,
    dead_code,
    unused_imports
)]

#[macro_use]
extern crate cfg_if;
extern crate core_foundation_sys;
extern crate libc;

#[cfg(any(target_os = "macos", target_os = "ios"))]
#[link(name = "CoreMedia", kind = "framework")]
extern "C" {}

pub mod attachment;
pub mod audio_device_clock;
pub mod base;
pub mod block_buffer;
pub mod buffer_queue;
pub mod format_description;
pub mod format_description_bridge;
pub mod memory_pool;
pub mod metadata;
pub mod sample_buffer;
pub mod sample_queue;
pub mod sync;
pub mod time;
pub mod time_range;

pub use self::attachment::*;
pub use self::audio_device_clock::*;
pub use self::base::*;
pub use self::block_buffer::*;
pub use self::buffer_queue::*;
pub use self::format_description::*;
pub use self::format_description_bridge::*;
pub use self::memory_pool::*;
pub use self::metadata::*;
pub use self::sample_buffer::*;
pub use self::sample_queue::*;
pub use self::sync::*;
pub use self::time::*;
pub use self::time_range::*;
