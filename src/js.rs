use ::*;

proc_macro_expr_decl! {
    js! => js_impl
}

pub trait IntoJson {
    fn into_json(self) -> String;
}

impl<T: serde::Serialize> IntoJson for T {
    fn into_json(self) -> String {
        serde_json::to_string(&self).expect("Failed to convert value to json")
    }
}

pub struct Callback<Args, F: StableFnMut<Args> + 'static> {
    f: F,
    phantom_data: PhantomData<Args>,
}

impl<Args, F: StableFnMut<Args>> From<F> for Callback<Args, F> {
    fn from(f: F) -> Self {
        Self {
            f,
            phantom_data: PhantomData,
        }
    }
}

impl<F: StableFnMut<(bool, )>> IntoJson for Callback<(bool, ), F> {
    fn into_json(self) -> String {
        let boxed = Box::new(self.f);
        extern "C" fn wrapper<F: StableFnMut<(bool, )>>(f: c_int, b: c_int) {
            let mut boxed = unsafe { Box::from_raw(f as *mut F) };
            boxed.call_mut((b != 0, ));
            mem::forget(boxed);
        }
        format!(
            "function(b) {{ Runtime.dynCall('vii', {}, [{}, b ? 1 : 0]); }}",
            (wrapper::<F> as c_int).into_json(),
            (Box::into_raw(boxed) as c_int).into_json()
        )
    }
}

impl<F: StableFnMut<(i32, )>> IntoJson for Callback<(i32, ), F> {
    fn into_json(self) -> String {
        let boxed = Box::new(self.f);
        extern "C" fn wrapper<F: StableFnMut<(i32, )>>(f: c_int, x: c_int) {
            let mut boxed = unsafe { Box::from_raw(f as *mut F) };
            boxed.call_mut((x as i32, ));
            mem::forget(boxed);
        }
        format!(
            "function(x) {{ Runtime.dynCall('vii', {}, [{}, x]); }}",
            (wrapper::<F> as c_int).into_json(),
            (Box::into_raw(boxed) as c_int).into_json()
        )
    }
}

impl<F: StableFnMut<(i32, i32)>> IntoJson for Callback<(i32, i32), F> {
    fn into_json(self) -> String {
        let boxed = Box::new(self.f);
        extern "C" fn wrapper<F: StableFnMut<(i32, i32)>>(f: c_int, a: c_int, b: c_int) {
            let mut boxed = unsafe { Box::from_raw(f as *mut F) };
            boxed.call_mut((a as i32, b as i32));
            mem::forget(boxed);
        }
        format!(
            "function(a, b) {{ Runtime.dynCall('viii', {}, [{}, a, b]); }}",
            (wrapper::<F> as c_int).into_json(),
            (Box::into_raw(boxed) as c_int).into_json()
        )
    }
}

impl<F: StableFnMut<()>> IntoJson for Callback<(), F> {
    fn into_json(self) -> String {
        let boxed = Box::new(self.f);
        extern "C" fn wrapper<F: StableFnMut<()>>(f: c_int) {
            let mut boxed = unsafe { Box::from_raw(f as *mut F) };
            boxed.call_mut(());
            mem::forget(boxed);
        }
        format!(
            "function(i) {{ Runtime.dynCall('vi', {}, [{}]); }}",
            (wrapper::<F> as c_int).into_json(),
            (Box::into_raw(boxed) as c_int).into_json()
        )
    }
}

pub fn run_script(script: &str) {
    emscripten::run_script(script);
}