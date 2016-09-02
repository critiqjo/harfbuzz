extern crate harfbuzz_sys;

use harfbuzz_sys::*;

pub struct Buffer(*mut hb_buffer_t);

impl Buffer {
    pub fn new() -> Buffer {
        unsafe { Buffer(hb_buffer_create()) }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe { hb_buffer_destroy(self.0) };
    }
}

#[cfg(test)]
mod tests {
    use super::Buffer;

    #[test]
    fn it_works() {
        let _ = Buffer::new();
    }
}
