use ::*;

pub fn run_script(script: &str) {
    emscripten::run_script(script);
}

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

pub unsafe trait CallbackArg {
    const SIGNATURE: char;
    type InteropType;
    fn js_conversion(name: &str) -> String;
    fn from(val: Self::InteropType) -> Self;
}

unsafe impl CallbackArg for i32 {
    const SIGNATURE: char = 'i';
    type InteropType = c_int;
    fn js_conversion(name: &str) -> String {
        name.to_owned()
    }
    fn from(val: c_int) -> i32 {
        val as i32
    }
}

unsafe impl CallbackArg for bool {
    const SIGNATURE: char = 'i';
    type InteropType = c_int;
    fn js_conversion(name: &str) -> String {
        format!("{} == 0 ? 0 : 1", name)
    }
    fn from(val: c_int) -> bool {
        val != 0
    }
}

macro_rules! impl_for_kind {
    ($($name:ident),*) => {
        impl<$($name:CallbackArg,)* F:StableFnMut<($($name,)*)>> IntoJson for Callback<($($name,)*),F> {
            #[allow(warnings)]
            fn into_json(self) -> String {
                let boxed = Box::new(self.f);
                extern "C" fn wrapper<$($name:CallbackArg,)* F:StableFnMut<($($name,)*)>>(f: c_int $(,$name:$name::InteropType)*) {
                    let mut boxed = unsafe { Box::from_raw(f as *mut F) };
                    boxed.call_mut(($($name::from($name),)*));
                    mem::forget(boxed);
                }
                let mut signature = String::from("vi");
                $(signature.push($name::SIGNATURE);)*
                let mut js_conversions = String::new();
                $(
                    js_conversions.push(',');
                    js_conversions += &$name::js_conversion(stringify!($name));
                )*
                format!(
                    concat!("function(", $(stringify!($name),",",)* ") {{",
                    "return Runtime.dynCall('{}', {}, [{}{}]);",
                    "}}"),
                    signature,
                    (wrapper::<$($name,)* F> as c_int).into_json(),
                    (Box::into_raw(boxed) as c_int).into_json(),
                    js_conversions)
            }
        }
    }
}

impl_for_kind!();
impl_for_kind!(A);
impl_for_kind!(A,B);
impl_for_kind!(A,B,C);
impl_for_kind!(A,B,C,D);