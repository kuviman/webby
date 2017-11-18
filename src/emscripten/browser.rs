use ::*;

pub fn get_canvas_size() -> Vec2<usize> {
    unsafe {
        let mut width: c_int = mem::uninitialized();
        let mut height: c_int = mem::uninitialized();
        let mut is_fullscreen: c_int = mem::uninitialized();
        emscripten_get_canvas_size(&mut width, &mut height, &mut is_fullscreen);
        vec2(width as usize, height as usize)
    }
}

pub fn set_main_loop<F: FnMut()>(callback: F) {
    static mut SET: bool = false;
    let callback = Box::new(Box::new(callback));
    unsafe {
        if SET {
            emscripten_cancel_main_loop();
        } else {
            SET = true;
        }
        emscripten_set_main_loop_arg(Some(wrapper::<F>), Box::into_raw(callback) as *mut _, 0, 1);
        // FIXME: this is a hack. Emscripten (or rust?) optimizes emscripten_GetProcAddress out without this.
        emscripten_GetProcAddress(std::ptr::null());
    }
    unsafe extern "C" fn wrapper<F>(arg: *mut c_void)
        where
            F: FnMut(),
    {
        let mut callback = Box::<Box<F>>::from_raw(arg as *mut _);
        callback();
        mem::forget(callback);
    }
}

pub fn get_now() -> f64 {
    unsafe { emscripten_get_now() as f64 / 1000.0 }
}
