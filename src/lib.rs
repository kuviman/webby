#![deny(warnings)]

#[macro_use]
extern crate proc_macro_hack;
#[allow(unused_imports)]
#[macro_use]
extern crate webby_macro;
extern crate serde;
extern crate serde_json;
extern crate url;
pub extern crate emscripten;
extern crate prelude;

pub use webby_macro::*;
pub(crate) use prelude::*;

mod js;

pub use js::*;

pub fn get_query_parameters() -> HashMap<String, Vec<String>> {
    let url = emscripten::run_script_string("window.location.href");
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