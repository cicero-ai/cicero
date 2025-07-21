extern crate libloading as lib;
//extern crate "C";
use std::path::Path;
use std::fs::File;
use std::io::Write;
use tempfile::NamedTempFile;
use std::env::consts::OS;
use std::error::Error;
use std::ffi::{c_char, CStr};
use std::ptr::{null, null_mut};
use libloading::{Library, Symbol};
use cicero_interfaces::sophia::{SophiaInterface, SophiaSharedLibrary};
use super::License;

#[no_mangle]
//pub extern "C" fn load_sophia(datadir: *const *const c_char, language: *const *const c_char) -> (*const dyn SophiaInterface, i32) {
pub extern "C" fn load_sophia(datadir: *const *const c_char, language: *const *const c_char) -> (*const (), i32) {

    // Load library
    let (lib, license): (Library, License) = match get_loaded_library() {
        Ok(r) => r,
        Err(code) => return (null_mut(), code)
    };

    // Format variables into &str
    let datadir_str = unsafe { CStr::from_ptr(*datadir).to_str().unwrap() };
    let language_str = unsafe { CStr::from_ptr(*language).to_str().unwrap() };

    unsafe {

        // Init plugin
        let init_func: Symbol<unsafe extern fn(&str, &str, Box<dyn ForgeAPI>) -> *const dyn SophiaInterface> = match lib.get(b"init") {
            Ok(r) => r,
            Err(_) => return (null_mut(), 8)
        };

        // Load plugin
        let sophia = init_func(&datadir_str, &language_str, license.into_api());

        let code: i32 = 0 as i32;
        return (sophia as *mut () as *const (), code);
        //return (sophia as *mut () as *const () sophia, 0 as i32);
        //return (Box::into_raw(Box::new(sophia)), 0);
    }

    (null_mut(), 10)
    //(Box::into_raw(Box::new(sophia)), 0)
}


#[no_mangle]
pub extern "C" fn free_sophia(plugin: *mut dyn SophiaInterface) {
    if !plugin.is_null() {
        unsafe {
            drop(Box::from_raw(plugin));
        }
    }
}

#[cfg(feature="local")]
/// Load Sophia
pub fn load_sophia_local(libdir: &str, datadir: &str, language: &str) -> Result<SophiaSharedLibrary, i32> {

    let libfile = format!("{}/libsophia.so", libdir);
    if !Path::new(&libfile).exists() {
        return Err(7);
    }

    // Load library
    unsafe {    
        let libpath = Path::new(&libfile);
        let lib = match Library::new(libpath) {
            Ok(r) => r,
            Err(e) => return Err(8)
        };

        return load_symbols(lib, &License::load_local(), &datadir, &language);
    }

    Err(10)
}

/// Load symbols
fn load_symbols(lib: Library, license: &License, datadir: &str, language: &str) -> Result<SophiaSharedLibrary, i32> {

    unsafe {

        // Init plugin
        let init_func: Symbol<unsafe extern fn(&str, &str, Box<dyn ForgeAPI>) -> *const dyn SophiaInterface> = match lib.get(b"init") {
            Ok(r) => r,
            Err(e) => return Err(8)
        };

        // Load plugin
        return Ok( SophiaSharedLibrary {
            ptr: init_func(&datadir, &language, license.into_api()),
            symbols: lib
        });
    }

    Err(9)
}

/// Get loaded library
fn get_loaded_library() -> Result<(Library, License), i32> {

    // Get library contents
    let lib_contents = match OS {
        "linux" => include_bytes!("/home/boxer/devel/cicero/cicero/share/verax/data/release/libsophia.so"),
        "macos" => include_bytes!("/home/boxer/devel/cicero/cicero/share/verax/data/release/libsophia.dylib"),
        "windows" => include_bytes!("/home/boxer/devel/cicero/cicero/share/verax/data/release/libsophia.dll"),
        _ => return Err(1)
    };

    // Load and validate license
    let mut license = License::load();
    if !license.validate() {
        return Err(2);
    }

    // Decrypt
    let lib_bytes = match license.decrypt(&lib_contents.as_slice()) {
        Some(r) => r,
        None => return Err(3)
    };

    // Create temp file
    let mut file = match NamedTempFile::new() {
        Ok(r) => r,
        Err(_) => return Err(4)
    };

    // Write library bytes to file
    if let Err(_) = file.write_all(&lib_bytes) {
        return Err(5);
    }

    // Load library
    unsafe {
        if let Ok(lib) = Library::new(file.path()) {
            file.close().unwrap();
            return Ok((lib, license));
        }
    }

    file.close().unwrap();
    Err(6)
}


