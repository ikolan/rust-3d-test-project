use crate::colors;
use std::ffi::CString;

pub(crate) struct Window;
pub(crate) struct Frame<'a>(&'a Window);

impl Window {
    pub(crate) fn new(w: i32, h: i32) -> Self {
        unsafe {
            raylib_sys::InitWindow(w, h, CString::new("").unwrap().into_raw());
            raylib_sys::SetTargetFPS(60);
        }
        Self
    }

    pub(crate) fn should_close(&self) -> bool {
        unsafe { raylib_sys::WindowShouldClose() }
    }

    pub(crate) fn begin_drawing(&self) -> Frame {
        unsafe {
            raylib_sys::BeginDrawing();
            raylib_sys::ClearBackground(colors::BLACK);
            raylib_sys::SetWindowTitle(
                CString::new(raylib_sys::GetFPS().to_string())
                    .unwrap()
                    .into_raw(),
            );
        }
        Frame(self)
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            raylib_sys::CloseWindow();
        }
    }
}

impl Frame<'_> {
    pub(crate) fn draw_line(
        &self,
        start_pos_x: i32,
        start_pos_y: i32,
        end_pos_x: i32,
        end_pos_y: i32,
        color: raylib_sys::Color,
    ) {
        unsafe {
            raylib_sys::DrawLine(start_pos_x, start_pos_y, end_pos_x, end_pos_y, color);
        }
    }
}

impl Drop for Frame<'_> {
    fn drop(&mut self) {
        unsafe {
            raylib_sys::EndDrawing();
        }
    }
}
