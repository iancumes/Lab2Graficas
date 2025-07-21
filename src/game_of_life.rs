pub struct GameOfLife {
    width: usize,
    height: usize,
    cells: Vec<bool>,
    buffer: Vec<bool>,
}

impl GameOfLife {
    /// Crea un autómata de tamaño width×height (todo muerto)
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Self {
            width,
            height,
            cells: vec![false; size],
            buffer: vec![false; size],
        }
    }

    /// Marca la celda (x,y) como viva (`alive = true`) o muerta
    pub fn set(&mut self, x: usize, y: usize, alive: bool) {
        self.cells[y * self.width + x] = alive;
    }

    /// Aplica las reglas de Conway para generar la siguiente generación
    pub fn step(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let mut n = 0;
                for dy in [-1isize, 0, 1].iter() {
                    for dx in [-1isize, 0, 1].iter() {
                        if *dx == 0 && *dy == 0 { continue; }
                        let nx = (x as isize + dx + self.width as isize) % self.width as isize;
                        let ny = (y as isize + dy + self.height as isize) % self.height as isize;
                        if self.cells[ny as usize * self.width + nx as usize] {
                            n += 1;
                        }
                    }
                }
                let idx = y * self.width + x;
                self.buffer[idx] = match (self.cells[idx], n) {
                    (true, 2) | (true, 3) => true,   // sobrevive
                    (true, _)    => false,           // muere
                    (false, 3)   => true,            // reproducción
                    (state, _)   => state,           // resto sin cambio
                };
            }
        }
        // intercambiamos buffers
        std::mem::swap(&mut self.cells, &mut self.buffer);
    }

    /// Dibuja todas las células vivas en el framebuffer usando `point`
    pub fn draw(&self, fb: &mut crate::framebuffer::Framebuffer) {
        fb.clear();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.cells[y * self.width + x] {
                    fb.point(x as u32, y as u32);
                }
            }
        }
    }
}
