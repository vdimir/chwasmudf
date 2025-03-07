extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn clickhouse_udf(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let fn_name = &input.sig.ident;
    let fn_inputs = &input.sig.inputs;
    let fn_body = input.block;
    let fn_output = input.sig.output;

    let internal_fn_ident = syn::Ident::new(&format!("_chudf_impl_{}", fn_name), fn_name.span());

    let arg_names: Vec<_> = fn_inputs
        .iter()
        .enumerate()
        .map(|(i, _)| syn::Ident::new(&format!("arg{}", i), proc_macro2::Span::call_site()))
        .collect();

    let expanded = quote! {

        // Re-define original function with a new name
        fn #internal_fn_ident(#fn_inputs) #fn_output {
            #fn_body
        }

        #[no_mangle]
        pub extern "C" fn #fn_name(data: &::clickhouse_wasm_sdk::mem::CHBytesBuffer, num_rows: usize) -> *const ::clickhouse_wasm_sdk::mem::CHBytesBuffer {
            use ::clickhouse_wasm_sdk::mem::CHBytesBuffer;

            if data.ptr.is_null() || data.len == 0 {
                ::clickhouse_wasm_sdk::clickhouse_logf!("Input data is empty");
                return std::ptr::null();
            }

            let input_data = unsafe { std::slice::from_raw_parts(data.ptr, data.len) };
            let mut cin = std::io::Cursor::new(input_data);

            let mut serialized = Vec::<u8>::new();
            let mut cout = std::io::Cursor::new(&mut serialized);
            let mut serializer = ::rmp_serde::encode::Serializer::new(&mut cout);
            let mut result_num_rows = 0;
            while cin.position() != data.len as u64 {
                let mut deser = ::rmp_serde::decode::Deserializer::new(&mut cin);
                // Reading arguments
                #(

                    let #arg_names = match serde::de::Deserialize::deserialize(&mut deser) {
                        Ok(val) => val,
                        Err(err) => {
                            ::clickhouse_wasm_sdk::clickhouse_fatalf!("Error deserializing argument: {:?}", err);
                        }
                    };
                )*

                // Calling the inner function
                let result = #internal_fn_ident(#(#arg_names),*);

                if let Err(write_err) = serde::ser::Serialize::serialize(&result, &mut serializer) {
                    ::clickhouse_wasm_sdk::clickhouse_fatalf!("Error serializing result: {:?}", write_err);
                }
                result_num_rows += 1;
            }

            if result_num_rows != num_rows {
                ::clickhouse_wasm_sdk::clickhouse_fatalf!("Expected {} rows, got {}", num_rows, result_num_rows);
            }

            let out = CHBytesBuffer::from_vec(serialized);
            return Box::into_raw(Box::new(out));
        }
    };

    TokenStream::from(expanded)
}
