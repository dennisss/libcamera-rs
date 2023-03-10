mod camera;
mod camera_configuration;
mod camera_manager;
mod control;
mod control_info;
mod control_info_map;
mod control_list;
mod control_value;
pub mod controls;
mod errors;
mod ffi;
mod frame_buffer;
mod frame_buffer_allocator;
mod pixel_format;
mod request;
mod stream;
mod stream_configuration;
mod stream_formats;

mod bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(unused)]

    mod raw {
        include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
    }

    pub use raw::root::libcamera::*;
}

pub use bindings::formats;
pub use camera::*;
pub use camera_configuration::*;
pub use camera_manager::*;
pub use control::Control;
pub use control::*;
pub use control_info::*;
pub use control_info_map::*;
pub use control_list::*;
pub use control_value::*;
pub use errors::*;
pub use frame_buffer::*;
pub use frame_buffer_allocator::*;
pub use pixel_format::*;
pub use request::*;
pub use stream::*;
pub use stream_configuration::*;
pub use stream_formats::*;

pub use crate::ffi::{CameraConfigurationStatus, FrameBufferPlane, StreamRole};
