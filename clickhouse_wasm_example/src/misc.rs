use clickhouse_wasm_udf_bindgen::clickhouse_udf;

use clickhouse_wasm_sdk::host_imports::server_version;
use clickhouse_wasm_sdk::mem::CHBytesBuffer;
use clickhouse_wasm_sdk::{clickhouse_fatalf, clickhouse_logf};
use std::collections::HashMap;
use std::vec::Vec;

use std::cell::RefCell;
use std::io::{BufRead, BufReader, Cursor, Write};

#[no_mangle]
pub extern "C" fn add3_i64(a: u64, b: u64, c: u64) -> u64 {
    a + b + c
}

#[no_mangle]
pub extern "C" fn add3_i32(a: u32, b: u32, c: u32) -> u32 {
    a + b + c
}

#[no_mangle]
pub extern "C" fn add3_f32(a: f32, b: f32, c: f32) -> f32 {
    a + b + c
}

#[no_mangle]
pub extern "C" fn add3_f64(a: f64, b: f64, c: f64) -> f64 {
    a + b + c
}

#[no_mangle]
pub extern "C" fn golden_ratio() -> f64 {
    let five: f64 = 5.0;
    (1.0 + five.sqrt()) / 2.0
}

thread_local! {
    static COUNTER: RefCell<u64> = RefCell::new(0);
}

#[no_mangle]
pub extern "C" fn simple_counter() -> u64 {
    COUNTER.with(|counter| {
        let mut counter = counter.borrow_mut();
        *counter += 1;
        *counter
    })
}

#[no_mangle]
pub extern "C" fn hello_csv(data: *const u8, num_rows: usize) -> *const CHBytesBuffer {
    if !data.is_null() {
        clickhouse_fatalf!("Input data is not empty");
    }
    let mut serialized = Vec::<u8>::new();
    let mut cout = Cursor::new(&mut serialized);
    for _ in 0..num_rows {
        let write_res = writeln!(&mut cout, "\"Hello, ClickHouse {}!\"", server_version());
        if write_res.is_err() {
            clickhouse_fatalf!("Error serializing data: {:?}", write_res.err());
        }
    }
    Box::into_raw(Box::new(CHBytesBuffer::from_vec(serialized)))
}

#[no_mangle]
pub extern "C" fn add3_csv(input_data: &CHBytesBuffer, _num_rows: usize) -> *const CHBytesBuffer {
    if input_data.ptr.is_null() || input_data.len == 0 {
        clickhouse_logf!("Input data is empty");
        return std::ptr::null();
    }

    let mut cin =
        Cursor::new(unsafe { std::slice::from_raw_parts(input_data.ptr, input_data.len) });
    let mut serialized = Vec::<u8>::new();
    let mut cout = Cursor::new(&mut serialized);
    let reader = BufReader::new(&mut cin);
    let mut num_rows = 0;

    for input_line in reader.lines() {
        match input_line {
            Ok(line) => {
                let mut row_sum = 0;
                for num in line.split(',') {
                    match num.trim().parse::<u64>() {
                        Ok(val) => row_sum += val,
                        Err(err) => clickhouse_fatalf!("Error parsing data: {:?}", err),
                    }
                }
                let write_res = writeln!(&mut cout, "{}", row_sum);
                if write_res.is_err() {
                    clickhouse_fatalf!("Error serializing data: {:?}", write_res.err());
                }
                num_rows += 1;
            }
            Err(err) => clickhouse_fatalf!("Error reading input data: {:?}", err),
        }
    }
    clickhouse_logf!("ok, result rows {}", num_rows);
    let out = CHBytesBuffer::from_vec(serialized);
    Box::into_raw(Box::new(out))
}

#[no_mangle]
pub extern "C" fn always_returns_ten_rows(
    _data: &CHBytesBuffer,
    num_rows: usize,
) -> *const CHBytesBuffer {
    let mut serialized = Vec::<u8>::new();
    let mut cout = Cursor::new(&mut serialized);
    let result_rows = if num_rows != 10 { 10 } else { 11 };
    for _ in 0..result_rows {
        let write_res = writeln!(&mut cout, "{}", 1);
        if write_res.is_err() {
            clickhouse_fatalf!("Error serializing data: {:?}", write_res.err());
        }
    }
    let res = CHBytesBuffer::from_vec(serialized);
    Box::into_raw(Box::new(res))
}

#[no_mangle]
pub extern "C" fn get_block_size(_data: &CHBytesBuffer, num_rows: usize) -> *const CHBytesBuffer {
    let mut serialized = Vec::<u8>::new();
    let mut cout = Cursor::new(&mut serialized);
    for _ in 0..num_rows {
        let write_res = writeln!(&mut cout, "{}", num_rows);
        if write_res.is_err() {
            clickhouse_fatalf!("Error serializing data: {:?}", write_res.err());
        }
    }
    let res = CHBytesBuffer::from_vec(serialized);
    Box::into_raw(Box::new(res))
}

#[clickhouse_udf]
fn vadd3_u64(a: u64, b: u64, c: u64) -> u64 {
    return a + b + c;
}

#[clickhouse_udf]
fn complex_data_type(a: HashMap<String, u64>, b: Vec<String>) -> u64 {
    let mut result = 0;
    for name in b {
        if let Some(value) = a.get(&name) {
            result += value;
        }
    }
    return result;
}
