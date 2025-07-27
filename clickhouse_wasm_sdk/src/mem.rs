#[repr(C)]
#[derive(Debug)]
pub struct CHBytesBuffer {
    pub ptr: *const u8,
    pub len: usize,
    pub capacity: usize,
}

impl CHBytesBuffer {
    pub fn new() -> Self {
        CHBytesBuffer {
            ptr: std::ptr::null(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn from_vec(v: Vec<u8>) -> Self {
        let ptr = v.as_ptr();
        let len = v.len();
        let capacity = v.capacity();
        std::mem::forget(v);
        CHBytesBuffer { ptr, len, capacity }
    }
}

#[no_mangle]
pub extern "C" fn clickhouse_create_buffer(size: usize) -> *mut CHBytesBuffer {
    if size == 0 {
        return std::ptr::null_mut();
    }
    let buf = CHBytesBuffer::from_vec(vec![0; size]);
    let ptr = Box::into_raw(Box::new(buf));
    return ptr;
}

#[no_mangle]
pub extern "C" fn clickhouse_destroy_buffer(ptr: *mut CHBytesBuffer) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let ptr = Box::from_raw(ptr);
        let _ = Vec::<u8>::from_raw_parts(ptr.ptr as *mut u8, ptr.len, ptr.capacity);
    }
}
