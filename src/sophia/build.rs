fn main() {
    println!("cargo:rustc-link-lib=bz2");
    println!("cargo:rustc-link-lib=zstd");
}
