use ffi::*;
use libc::{c_uint};
use std::marker::PhantomData;
use std::ptr;
use std::slice;
use std::u32;

enum MemoryMode {
    Duplicate = 0,
    Readonly = 1,
    Writable = 2,
    //ReadonlyMayMakeWritable = 3,
}

/// A reference counted blob of data.
///
/// When the reference count reaches zero, it invokes a callback that was passed to it which is
/// responsible for freeing the underlying data (not exposed from this library).
pub struct Blob<'a> {
    inner: *mut hb_blob_t,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> Blob<'a> {
    unsafe fn create(data: &[u8], mode: MemoryMode) -> *mut hb_blob_t{
        assert!(data.len() < u32::MAX as usize);
        inner: hb_blob_create(data.as_ptr() as *mut i8,
                              data.len() as c_uint,
                              mode as c_uint,
                              ptr::null_mut(),
                              None)
    }

    /// Creates a read-only `Blob` from a byte slice.
    ///
    /// The destructor callback is empty.
    pub fn new<'s>(data: &'s [u8]) -> Blob<'s> {
        Blob {
            inner: unsafe { Blob::create(data, MemoryMode::Readonly) },
            _phantom: PhantomData,
        }
    }

    /// Creates a writable `Blob` from a mutable byte slice.
    ///
    /// The destructor callback is empty.
    pub fn writable<'s>(data: &'s mut [u8]) -> Blob<'s> {
        Blob {
            inner: unsafe { Blob::create(data, MemoryMode::Writable) },
            _phantom: PhantomData,
        }
    }

    /// Creates a writable `Blob` from a byte slice by duplicating it.
    ///
    /// If allocation fails, the empty blob is returned.
    ///
    /// An appropriate destructor callback is set internally.
    pub fn copied(data: &[u8]) -> Blob<'static> {
        Blob {
            inner: unsafe { Blob::create(data, MemoryMode::Duplicate) },
            _phantom: PhantomData,
        }
    }

    /// Returns the empty blob.
    pub fn empty() -> Blob<'static> {
        Blob {
            inner: unsafe { hb_blob_get_empty() },
            _phantom: PhantomData,
        }
    }

    /// Size of the blob in `u32`.
    pub fn len(&self) -> u32 {
        unsafe { hb_blob_get_length(self.inner) }
    }

    /// Returns a sub-range of bytes as a new blob without copying.
    ///
    /// Makes `self` immutable and increments the internal reference count.
    /// If something fails, the empty blob is returned.
    pub fn create_sub_blob(&mut self, offset: c_uint, length: c_uint) -> Blob<'a> {
        Blob {
            inner: unsafe { hb_blob_create_sub_blob(self.inner, offset, length) },
            _phantom: PhantomData,
        }
    }

    pub fn get_data(&self) -> &[u8] {
        let mut length = 0u32;
        unsafe {
            let data = hb_blob_get_data(self.inner, &mut length as *mut c_uint);
            slice::from_raw_parts(data as *const u8, length as usize)
        }
    }

// {{{
//    // immutable read-only blobs cannot be made writable by creating a copy
//    pub fn make_immutable(&mut self) -> () {
//        unsafe { hb_blob_make_immutable(self.inner) }
//    }
//
//    pub fn is_immutable(&self) -> bool {
//        unsafe { hb_blob_is_immutable(self.inner) != 0 }
//    }
//
//    pub fn reference(&mut self) -> *mut Blob {
//        unsafe { hb_blob_reference(blob) }
//    }
//
//    pub fn set_user_data(&mut self, key: *mut UserDataKey, data: *mut ::libc::c_void, destroy: hb_destroy_func_t, replace: hb_bool_t) -> hb_bool_t {
//        unsafe { hb_blob_set_user_data(blob, key, data, destroy, replace) }
//    }
//
//    pub fn get_user_data(&mut self, key: *mut UserDataKey) -> *mut ::libc::c_void {
//        unsafe { hb_blob_get_user_data(blob, key) }
//    }
//
//    pub fn get_data_writable(&mut self, length: *mut c_uint) -> *mut ::libc::c_char {
//        unsafe { hb_blob_get_data_writable(blob, length) }
//    }
// }}}

    pub fn raw(&self) -> *mut hb_blob_t {
        self.inner
    }
}

impl<'a> Drop for Blob<'a> {
    fn drop(&mut self) {
        unsafe { hb_blob_destroy(self.inner) }
    }
}
