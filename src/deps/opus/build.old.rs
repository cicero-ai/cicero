
fn main() {

        println!("cargo:rustc-l ink-lib=static=rocksdb");
        println!("cargo:rustc-link-search=native={}/../../build/librocksdb-sys-*/out", std::env::var("OUT_DIR").unwrap());

        // Ensure static libs are bundled into the dylib
        println!("cargo:rustc-link-arg=-Wl,--whole-archive");
        println!("cargo:rustc-link-arg=-lrocksdb");
        println!("cargo:rustc-link-arg=-Wl,--no-whole-archive");

}





