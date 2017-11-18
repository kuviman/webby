use ::*;

pub struct TouchPoint {
    pub canvas_pos: Vec2<f64>,
}

pub struct TouchStartEvent {
    pub touches: Vec<TouchPoint>,
}

pub fn set_touchstart_callback<F: FnMut(TouchStartEvent)>(callback: F) {
    let callback = Box::new(Box::new(callback));
    unsafe {
        emscripten_set_touchstart_callback(
            CANVAS_SELECTOR.as_ptr() as *const _,
            Box::into_raw(callback) as *mut _,
            USE_CAPTURE,
            Some(wrapper::<F>),
        );
    }
    unsafe extern "C" fn wrapper<F>(
        _: c_int,
        event: *const EmscriptenTouchEvent,
        callback: *mut c_void,
    ) -> EM_BOOL
    where
        F: FnMut(TouchStartEvent),
    {
        let event = *event;
        let mut callback = Box::<Box<F>>::from_raw(callback as *mut _);
        let mut touches = Vec::with_capacity(event.numTouches as usize);
        for i in 0..event.numTouches as usize {
            let canvas_pos =
                into_canvas_pos(vec2(event.touches[i].canvasX, event.touches[i].canvasY));
            touches.push(TouchPoint { canvas_pos });
        }
        callback(TouchStartEvent { touches });
        mem::forget(callback);
        EM_TRUE
    }
}

pub struct TouchMoveEvent {
    pub touches: Vec<TouchPoint>,
}

pub fn set_touchmove_callback<F: FnMut(TouchMoveEvent)>(callback: F) {
    let callback = Box::new(Box::new(callback));
    unsafe {
        emscripten_set_touchmove_callback(
            CANVAS_SELECTOR.as_ptr() as *const _,
            Box::into_raw(callback) as *mut _,
            USE_CAPTURE,
            Some(wrapper::<F>),
        );
    }
    unsafe extern "C" fn wrapper<F>(
        _: c_int,
        event: *const EmscriptenTouchEvent,
        callback: *mut c_void,
    ) -> EM_BOOL
    where
        F: FnMut(TouchMoveEvent),
    {
        let event = *event;
        let mut callback = Box::<Box<F>>::from_raw(callback as *mut _);
        let mut touches = Vec::with_capacity(event.numTouches as usize);
        for i in 0..event.numTouches as usize {
            let canvas_pos =
                into_canvas_pos(vec2(event.touches[i].canvasX, event.touches[i].canvasY));
            touches.push(TouchPoint { canvas_pos });
        }
        callback(TouchMoveEvent { touches });
        mem::forget(callback);
        EM_TRUE
    }
}

pub struct TouchEndEvent;

pub fn set_touchend_callback<F: FnMut(TouchEndEvent)>(callback: F) {
    let callback = Box::new(Box::new(callback));
    unsafe {
        emscripten_set_touchend_callback(
            CANVAS_SELECTOR.as_ptr() as *const _,
            Box::into_raw(callback) as *mut _,
            USE_CAPTURE,
            Some(wrapper::<F>),
        );
    }
    unsafe extern "C" fn wrapper<F>(
        _: c_int,
        _: *const EmscriptenTouchEvent,
        callback: *mut c_void,
    ) -> EM_BOOL
    where
        F: FnMut(TouchEndEvent),
    {
        let mut callback = Box::<Box<F>>::from_raw(callback as *mut _);
        callback(TouchEndEvent);
        mem::forget(callback);
        EM_TRUE
    }
}
