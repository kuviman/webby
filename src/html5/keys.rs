use ::*;

pub struct Key {
    pub code: String,
}

impl Key {
    fn from(event: &EmscriptenKeyboardEvent) -> Self {
        let code = unsafe { CStr::from_ptr(event.code.as_ptr()) };
        let code = code.to_str().expect("Key code is not a valid UTF-8 string");
        Self {
            code: code.to_owned(),
        }
    }
}

pub struct KeyDownEvent {
    pub key: Key,
}

pub struct KeyUpEvent {
    pub key: Key,
}

pub fn set_keydown_callback<F: FnMut(KeyDownEvent)>(callback: F) {
    let callback = Box::new(Box::new(callback));
    unsafe {
        emscripten_set_keydown_callback(
            DOCUMENT_SELECTOR.as_ptr() as *const _,
            Box::into_raw(callback) as *mut _,
            USE_CAPTURE,
            Some(wrapper::<F>),
        );
    }
    unsafe extern "C" fn wrapper<F>(
        _: c_int,
        event: *const EmscriptenKeyboardEvent,
        callback: *mut c_void,
    ) -> EM_BOOL
        where
            F: FnMut(KeyDownEvent),
    {
        let event = &*event;
        let key = Key::from(&event);
        let mut callback = Box::<Box<F>>::from_raw(callback as *mut _);
        callback(KeyDownEvent { key });
        mem::forget(callback);
        EM_FALSE as EM_BOOL
    }
}

pub fn set_keyup_callback<F: FnMut(KeyUpEvent)>(callback: F) {
    let callback = Box::new(Box::new(callback));
    unsafe {
        emscripten_set_keyup_callback(
            DOCUMENT_SELECTOR.as_ptr() as *const _,
            Box::into_raw(callback) as *mut _,
            USE_CAPTURE,
            Some(wrapper::<F>),
        );
    }
    unsafe extern "C" fn wrapper<F>(
        _: c_int,
        event: *const EmscriptenKeyboardEvent,
        callback: *mut c_void,
    ) -> EM_BOOL
        where
            F: FnMut(KeyUpEvent),
    {
        let event = &*event;
        let key = Key::from(&event);
        let mut callback = Box::<Box<F>>::from_raw(callback as *mut _);
        callback(KeyUpEvent { key });
        mem::forget(callback);
        EM_FALSE as EM_BOOL
    }
}