#[cfg(all(target_arch = "wasm32", target_os = "wasi"))]
mod wasi_module {
    use std::fs::File;
    use std::io::Read;

    use clickhouse_wasm_sdk::clickhouse_fatalf;
    #[no_mangle]
    pub extern "C" fn some_func() -> u64 {
        let mut file = match File::open("foo.txt") {
            Ok(f) => f,
            Err(e) => clickhouse_fatalf!("Error reading file: {:?}", e),
        };

        let mut contents = String::new();
        if let Err(e) = file.read_to_string(&mut contents) {
            clickhouse_fatalf!("Error reading file: {:?}", e);
        }
        match contents.trim().parse() {
            Ok(n) => return n,
            Err(e) => clickhouse_fatalf!("Error parsing number: {:?}", e),
        };
    }
}

#[cfg(all(target_arch = "wasm32", target_os = "wasi"))]
pub use wasi_module::*;
