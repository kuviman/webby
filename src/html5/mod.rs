use ::*;

mod mouse;

pub use self::mouse::*;

mod wheel;

pub use self::wheel::*;

mod touch;

pub use self::touch::*;

mod webgl;

pub use self::webgl::*;

mod keys;

pub use self::keys::*;

pub ( crate ) fn into_canvas_pos(pos: Vec2<c_long>) -> Vec2<f64> {
    let pos = vec2(pos.x as f64, pos.y as f64);
    let css_size = unsafe {
        let mut css_width: c_double = mem::uninitialized();
        let mut css_height: c_double = mem::uninitialized();
        emscripten_get_element_css_size(std::ptr::null(), &mut css_width, &mut css_height);
        vec2(css_width as f64, css_height as f64)
    };
    let canvas_size = get_canvas_size();
    let canvas_size = vec2(canvas_size.x as f64, canvas_size.y as f64);
    vec2(
        pos.x * canvas_size.x / css_size.x,
        pos.y * canvas_size.y / css_size.y,
    )
}
