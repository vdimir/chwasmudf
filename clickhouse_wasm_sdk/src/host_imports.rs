use std::ffi::c_char;

extern "C" {
    pub fn clickhouse_log(s: *const c_char, len: usize);
    pub fn clickhouse_terminate(s: *const c_char, len: usize) -> !;
    pub fn clickhouse_server_version() -> u64;
}

pub fn log(s: &str) {
    unsafe {
        clickhouse_log(s.as_ptr() as *const i8, s.len());
    }
}

pub fn terminate(s: &str) -> ! {
    unsafe {
        clickhouse_terminate(s.as_ptr() as *const i8, s.len());
    }
}

pub fn server_version() -> u64 {
    unsafe { clickhouse_server_version() }
}
