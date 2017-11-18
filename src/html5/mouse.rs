use ::*;

pub enum MouseButton {
    Left,
    Middle,
    Right,
}

impl MouseButton {
    fn from(button: c_ushort) -> Option<MouseButton> {
        match button {
            0 => Some(MouseButton::Left),
            1 => Some(MouseButton::Middle),
            2 => Some(MouseButton::Right),
            _ => None,
        }
    }
}

pub struct MouseDownEvent {
    pub canvas_pos: Vec2<f64>,
    pub button: MouseButton,
}

pub fn set_mousedown_callback<F: FnMut(MouseDownEvent)>(callback: F) {
    let callback = Box::new(Box::new(callback));
    unsafe {
        emscripten_set_mousedown_callback(
            CANVAS_SELECTOR.as_ptr() as *const _,
            Box::into_raw(callback) as *mut _,
            USE_CAPTURE,
            Some(wrapper::<F>),
        );
    }
    unsafe extern "C" fn wrapper<F>(
        _: c_int,
        event: *const EmscriptenMouseEvent,
        callback: *mut c_void,
    ) -> EM_BOOL
        where
            F: FnMut(MouseDownEvent),
    {
        let event = *event;
        if let Some(button) = MouseButton::from(event.button) {
            let mut callback = Box::<Box<F>>::from_raw(callback as *mut _);
            let canvas_pos = into_canvas_pos(vec2(event.canvasX, event.canvasY));
            callback(MouseDownEvent { canvas_pos, button });
            mem::forget(callback);
        }
        EM_FALSE as EM_BOOL
    }
}

pub struct MouseUpEvent {
    pub canvas_pos: Vec2<f64>,
    pub button: MouseButton,
}

pub fn set_mouseup_callback<F: FnMut(MouseUpEvent)>(callback: F) {
    let callback = Box::new(Box::new(callback));
    unsafe {
        emscripten_set_mouseup_callback(
            CANVAS_SELECTOR.as_ptr() as *const _,
            Box::into_raw(callback) as *mut _,
            USE_CAPTURE,
            Some(wrapper::<F>),
        );
    }
    unsafe extern "C" fn wrapper<F>(
        _: c_int,
        event: *const EmscriptenMouseEvent,
        callback: *mut c_void,
    ) -> EM_BOOL
        where
            F: FnMut(MouseUpEvent),
    {
        let event = *event;
        if let Some(button) = MouseButton::from(event.button) {
            let mut callback = Box::<Box<F>>::from_raw(callback as *mut _);
            let canvas_pos = into_canvas_pos(vec2(event.canvasX, event.canvasY));
            callback(MouseUpEvent { canvas_pos, button });
            mem::forget(callback);
        }
        EM_FALSE as EM_BOOL
    }
}

pub struct MouseMoveEvent {
    pub canvas_pos: Vec2<f64>,
}

pub fn set_mousemove_callback<F: FnMut(MouseMoveEvent)>(callback: F) {
    let callback = Box::new(Box::new(callback));
    unsafe {
        emscripten_set_mousemove_callback(
            CANVAS_SELECTOR.as_ptr() as *const _,
            Box::into_raw(callback) as *mut _,
            USE_CAPTURE,
            Some(wrapper::<F>),
        );
    }
    unsafe extern "C" fn wrapper<F>(
        _: c_int,
        event: *const ::emscripten_sys::EmscriptenMouseEvent,
        callback: *mut c_void,
    ) -> EM_BOOL
        where
            F: FnMut(MouseMoveEvent),
    {
        let event = *event;
        let mut callback = Box::<Box<F>>::from_raw(callback as *mut _);
        let canvas_pos = into_canvas_pos(vec2(event.canvasX, event.canvasY));
        callback(MouseMoveEvent { canvas_pos });
        mem::forget(callback);
        EM_FALSE as EM_BOOL
    }
}
