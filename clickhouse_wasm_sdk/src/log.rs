#[macro_export]
macro_rules! clickhouse_logf {
    ($($arg:tt)*) => {
        unsafe {
            let s = format!($($arg)*);
            $crate::host_imports::clickhouse_log(s.as_ptr() as *const i8, s.len());
        }
    }
}

#[macro_export]
macro_rules! clickhouse_fatalf {
    ($($arg:tt)*) => {
        unsafe {
            let s = format!($($arg)*);
            $crate::host_imports::clickhouse_terminate(s.as_ptr() as *const i8, s.len());
        }
    }
}
