extern crate soil2;

use soil2::SOIL_free_image_data;
use std::ptr::null_mut;

fn main() {
    unsafe {
        SOIL_free_image_data(null_mut());
    }
}
