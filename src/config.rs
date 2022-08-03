use opengl_graphics::OpenGL;

// Config Constants
pub const OPENGL_VERSION: OpenGL = OpenGL::V3_2;
pub const COLS: u32 = 30;
pub const ROWS: u32 = 20;
pub const SQUARE_WIDTH: u32 = 20;
pub const WIDTH: u32 = COLS * SQUARE_WIDTH;
pub const HEIGHT: u32 = ROWS * SQUARE_WIDTH;

pub const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub const BLACK: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
