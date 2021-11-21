use ::safer_ffi::prelude::*;



#[ffi_export]
pub fn test_with_no_args(){
    println!("hello from rust");
}

#[ffi_export]
pub fn test_with_one_int(arg: u8){
    println!("We got one int {}", arg);
}


#[ffi_export]
pub fn test_with_one_str(arg: safer_ffi::string::str_boxed){
    println!("We got one str {:?}", arg);
}

/// The following test function is necessary for the header generation.
#[::safer_ffi::cfg_headers]
#[test]
fn generate_headers() -> ::std::io::Result<()> {
    ::safer_ffi::headers::builder()
        .to_file("swig_example.h")?
        .generate()
}
