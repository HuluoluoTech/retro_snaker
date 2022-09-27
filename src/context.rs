
use glutin_window::GlutinWindow;
use piston::window::WindowSettings;
use crate::config::*;

pub fn setup() -> GlutinWindow {
    match WindowSettings::new("RetroSnaker", [WIDTH, HEIGHT])
        .opengl(OPENGL_VERSION)
        .exit_on_esc(true)
        .build()
    {
        Ok(w) => w,
        Err(s) => panic!("Init Window Error: {}", s),
    }
}
