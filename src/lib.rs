extern crate libc;

use std::os::raw::{c_char, c_int, c_uchar};

extern "C" {

    pub fn SOIL_load_image(
        filename: *const c_char,
        width: *mut c_int,
        height: *mut c_int,
        channels: *mut c_int,
        force_channels: c_int
    ) -> *mut c_uchar;


    pub fn SOIL_free_image_data(img_data: *mut c_uchar);
}