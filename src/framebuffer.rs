// src/framebuffer.rs

use raylib::prelude::*;

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub color_buffer: Image,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let color_buffer = Image::gen_image_color(width as i32, height as i32, Color::BLACK);
        Framebuffer {
            width,
            height,
            color_buffer,
            background_color: Color::BLACK,
            current_color: Color::WHITE,
        }
    }

    pub fn clear(&mut self) {
        self.color_buffer = Image::gen_image_color(
            self.width as i32,
            self.height as i32,
            self.background_color,
        );
    }

    pub fn set_pixel(&mut self, x: u32, y: u32) {
        if x < self.width && y < self.height {
            self.color_buffer
                .draw_pixel(x as i32, y as i32, self.current_color);
        }
    }

    pub fn point(&mut self, x: u32, y: u32) {
        self.set_pixel(x, y);
    }

    /// CORRECCIÓN: ahora hacemos un cast seguro del puntero `data` a `*const u8`
    /// y luego calculamos el offset en bytes (4 por píxel: R,G,B,A).
    pub fn get_color(&self, x: u32, y: u32) -> Color {
        unsafe {
            let pixel_data = self.color_buffer.data as *const u8;
            let row_stride = self.width as isize * 4;           // bytes por fila
            let offset = y as isize * row_stride + (x as isize * 4);
            let base = pixel_data.offset(offset);
            let r = *base;
            let g = *base.offset(1);
            let b = *base.offset(2);
            let a = *base.offset(3);
            Color::new(r, g, b, a)
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn _render_to_file(&self, file_path: &str) {
        self.color_buffer.export_image(file_path);
    }

    pub fn swap_buffers(
        &self,
        window: &mut RaylibHandle,
        raylib_thread: &RaylibThread,
        target_width: i32,
        target_height: i32,
    ) {
        if let Ok(texture) = window.load_texture_from_image(raylib_thread, &self.color_buffer) {
            let mut d = window.begin_drawing(raylib_thread);
            d.clear_background(self.background_color);

            let scale_x = target_width as f32 / self.width as f32;
            let scale_y = target_height as f32 / self.height as f32;
            let scale = scale_x.min(scale_y);

            d.draw_texture_ex(
                &texture,
                Vector2::new(0.0, 0.0),
                0.0,
                scale,
                Color::WHITE,
            );
        }
    }
}
