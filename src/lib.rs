#![deny(warnings)]

extern crate serde;
extern crate serde_json;
extern crate url;
extern crate emscripten_sys;
extern crate ugli;
extern crate prelude;

pub ( crate ) use prelude::*;

mod emscripten;

pub use emscripten::*;

mod html5;

pub use html5::*;

use emscripten_sys::*;

#[allow(non_camel_case_types)]
type EM_BOOL = c_int;

const EM_TRUE: EM_BOOL = 1;

const CANVAS_SELECTOR: &[u8] = b"#canvas\0";
const DOCUMENT_SELECTOR: &[u8] = b"#document\0";
const USE_CAPTURE: EM_BOOL = 1;

pub fn get_proc_address(name: &str) -> *const c_void {
    let name = CString::new(name).expect("Could not convert name to C string");
    unsafe { ::emscripten_sys::emscripten_GetProcAddress(name.as_ptr()) as *const _ }
}

pub fn get_query_parameters() -> HashMap<String, Vec<String>> {
    let url = run_script_string("window.location.href");
    let url = url::Url::parse(&url).expect("Failed to parse window.location.href");
    let mut result = HashMap::<String, Vec<String>>::new();
    for (key, value) in url.query_pairs() {
        let key: &str = &key;
        let value = value.into_owned();
        if result.contains_key(key) {
            result.get_mut(key).unwrap().push(value);
        } else {
            result.insert(key.to_owned(), vec![value]);
        }
    }
    result
}