use std::fs::remove_file;
use xshell::cmd;

pub(crate) fn clean() {
    let _ = remove_file("./python/swig_example/swig_example_wrap.o_wrap.o");
    let _ = remove_file("./python/swig_example/_swig_example.so");
    cmd!("cargo clean").read().expect("Failed to run cargo clean");
}
