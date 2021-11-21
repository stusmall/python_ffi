use std::env::current_dir;

use xshell::cmd;

pub(crate) fn build(release: bool) {
    build_rust(release);
    build_python(release);
}

fn build_rust(release: bool) {
    if release {
        cmd!("cargo build --release")
    } else {
        cmd!("cargo build")
    }
    .run()
    .expect("Failed to build rust workspace");
    cmd!(" cargo test --features c-headers -- generate_headers").run().expect("Failed to build headers");
}

fn build_python(release: bool) {
    let mut library_path = current_dir().unwrap();
    if release {
        library_path.push("target/release/libffi.a")
    } else {
        library_path.push("target/debug/libffi.a")
    };

    println!("Building python SWIG files");
    if let Err(e) = cmd!("swig -python -py3 -outdir python/swig_example -o python/swig_example/swig_example_wrap.c ./ffi/swig_example.i").run() {
        panic!("Python SWIG command failed: {}", e);
    }
    println!("Building python SWIG generated C file");
    if let Err(e) = cmd!("gcc -fpic -c -I /usr/include/python3.9 -I ./ffi/ python/swig_example/swig_example_wrap.c -o python/swig_example/swig_example_wrap.o").run() {
        panic!("Failed to build python SWIG generated C files: {}", e);
    }
    println!("Linking python shared object");
    if let Err(e) = cmd!("gcc -shared python/swig_example/swig_example_wrap.o {library_path} -o python/swig_example/_swig_example.so").run()
    {
        panic!("Failed to link the python shared object: {}", e);
    }
}
