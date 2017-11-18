use ::*;

pub struct WheelEvent {
    pub canvas_pos: Vec2<f64>,
    pub delta: f64,
}

const DOM_DELTA_PIXEL: c_ulong = 0x00;
const DOM_DELTA_LINE: c_ulong = 0x01;
const DOM_DELTA_PAGE: c_ulong = 0x02;

pub fn set_wheel_callback<F: FnMut(WheelEvent)>(callback: F) {
    let callback = Box::new(Box::new(callback));
    unsafe {
        emscripten_set_wheel_callback(
            CANVAS_SELECTOR.as_ptr() as *const _,
            Box::into_raw(callback) as *mut _,
            USE_CAPTURE,
            Some(wrapper::<F>),
        );
    }
    unsafe extern "C" fn wrapper<F>(
        _: c_int,
        event: *const EmscriptenWheelEvent,
        callback: *mut c_void,
    ) -> EM_BOOL
    where
        F: FnMut(WheelEvent),
    {
        let event = *event;
        let mut callback = Box::<Box<F>>::from_raw(callback as *mut _);
        let canvas_pos = into_canvas_pos(vec2(event.mouse.canvasX, event.mouse.canvasY));
        callback(WheelEvent {
            canvas_pos,
            delta: event.deltaY as f64 *
                match event.deltaMode {
                    DOM_DELTA_PIXEL => 1.0,
                    DOM_DELTA_LINE => 17.0,
                    DOM_DELTA_PAGE => 800.0,
                    _ => panic!("Unexpected event.deltaMode"),
                },
        });
        mem::forget(callback);
        EM_TRUE
    }
}
