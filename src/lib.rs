#![deny(warnings)]

extern crate serde;
extern crate serde_json;
extern crate url;
pub extern crate emscripten;
extern crate prelude;

pub(crate) use prelude::*;

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

#[macro_export]
macro_rules! format_placeholders {
    () => ("");
    ($arg:expr) => ("{}");
    ($head:expr, $($tail:expr),+) => (
        concat!("{},", format_placeholders!($($tail),+))
    )
}

#[macro_export]
macro_rules! run_js {
    ($($($f:ident).+ ( $($args:expr),* );)*) => (
        $(
            $crate::emscripten::run_script(&format!(
                concat!(stringify!($($f).+), "(", format_placeholders!($($args),*), ")"),
                $($crate::IntoJson::into_json($args)),*));
        )*
    )
}

pub trait IntoJson {
    fn into_json(self) -> String;
}

impl<'a, T: ? Sized + serde::Serialize> IntoJson for &'a T {
    fn into_json(self) -> String {
        ::serde_json::to_string(self).expect("Could not convert to JSON")
    }
}

pub struct Callback<Arg, F: FnMut(Arg) + 'static> {
    f: F,
    phantom_data: PhantomData<Arg>,
}

impl<Arg, F: FnMut(Arg)> From<F> for Callback<Arg, F> {
    fn from(f: F) -> Self {
        Self {
            f,
            phantom_data: PhantomData,
        }
    }
}

impl<F: FnMut(bool)> IntoJson for Callback<bool, F> {
    fn into_json(self) -> String {
        let boxed = Box::new(self.f);
        extern "C" fn wrapper<F: FnMut(bool)>(f: c_int, b: c_int) {
            let mut boxed = unsafe { Box::from_raw(f as *mut F) };
            boxed(b != 0);
            mem::forget(boxed);
        }
        format!(
            "function(b) {{ Runtime.dynCall('vii', {}, [{}, b ? 1 : 0]); }}",
            (wrapper::<F> as c_int).into_json(),
            (Box::into_raw(boxed) as c_int).into_json()
        )
    }
}

impl<F: FnMut(i32)> IntoJson for Callback<i32, F> {
    fn into_json(self) -> String {
        let boxed = Box::new(self.f);
        extern "C" fn wrapper<F: FnMut(i32)>(f: c_int, x: c_int) {
            let mut boxed = unsafe { Box::from_raw(f as *mut F) };
            boxed(x as i32);
            mem::forget(boxed);
        }
        format!(
            "function(x) {{ Runtime.dynCall('vii', {}, [{}, x]); }}",
            (wrapper::<F> as c_int).into_json(),
            (Box::into_raw(boxed) as c_int).into_json()
        )
    }
}

impl<F: FnMut((i32, i32))> IntoJson for Callback<(i32, i32), F> {
    fn into_json(self) -> String {
        let boxed = Box::new(self.f);
        extern "C" fn wrapper<F: FnMut((i32, i32))>(f: c_int, a: c_int, b: c_int) {
            let mut boxed = unsafe { Box::from_raw(f as *mut F) };
            boxed((a as i32, b as i32));
            mem::forget(boxed);
        }
        format!(
            "function(a, b) {{ Runtime.dynCall('viii', {}, [{}, a, b]); }}",
            (wrapper::<F> as c_int).into_json(),
            (Box::into_raw(boxed) as c_int).into_json()
        )
    }
}

impl<F: FnMut(())> IntoJson for Callback<(), F> {
    fn into_json(self) -> String {
        let boxed = Box::new(self.f);
        extern "C" fn wrapper<F: FnMut(())>(f: c_int) {
            let mut boxed = unsafe { Box::from_raw(f as *mut F) };
            boxed(());
            mem::forget(boxed);
        }
        format!(
            "function(i) {{ Runtime.dynCall('vi', {}, [{}]); }}",
            (wrapper::<F> as c_int).into_json(),
            (Box::into_raw(boxed) as c_int).into_json()
        )
    }
}
