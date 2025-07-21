// src/main.rs

mod framebuffer;
mod game_of_life;
mod line;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use game_of_life::GameOfLife;
use std::thread;
use std::time::Duration;

/// Semilla de Gosper Glider Gun en (ox,oy)
fn seed_gosper_gun(game: &mut GameOfLife, ox: usize, oy: usize) {
    let coords = [
        (24,0),(22,1),(24,1),
        (12,2),(13,2),(20,2),(21,2),(34,2),(35,2),
        (11,3),(15,3),(20,3),(21,3),(34,3),(35,3),
        (0,4),(1,4),(10,4),(16,4),(20,4),(21,4),
        (0,5),(1,5),(10,5),(14,5),(16,5),(17,5),(22,5),(24,5),
        (12,6),(13,6),(20,6),(21,6),(34,6),(35,6),
        (22,7),(24,7),
        (23,8),
    ];
    for &(dx,dy) in &coords {
        game.set(ox + dx, oy + dy, true);
    }
}

/// Semilla de un glider en (ox,oy)
fn seed_glider(game: &mut GameOfLife, ox: usize, oy: usize) {
    let coords = [ (1,0), (2,1), (0,2), (1,2), (2,2) ];
    for &(dx,dy) in &coords {
        game.set(ox + dx, oy + dy, true);
    }
}

fn main() {
    // Tamaño real de la ventana
    let window_w = 800;
    let window_h = 600;

    // Resolución lógica del autómata
    let grid_w = 100;
    let grid_h = 100;

    let (mut window, thread) = raylib::init()
        .size(window_w, window_h)
        .title("Conway's Game of Life")
        .build();

    // Framebuffer lógico
    let mut fb = Framebuffer::new(grid_w, grid_h);
    fb.set_background_color(Color::BLACK);
    fb.set_current_color(Color::WHITE);

    // Instancia del juego
    let mut game = GameOfLife::new(grid_w as usize, grid_h as usize);

    // 1) STILL LIVES
    // Block
    game.set(2, 2, true);  game.set(3, 2, true);
    game.set(2, 3, true);  game.set(3, 3, true);

    // Beehive
    let ox = 10; let oy = 2;
    game.set(ox+1, oy+0, true); game.set(ox+2, oy+0, true);
    game.set(ox+0, oy+1, true);                     game.set(ox+3, oy+1, true);
    game.set(ox+1, oy+2, true); game.set(ox+2, oy+2, true);

    // Loaf
    let ox = 18; let oy = 2;
    game.set(ox+1, oy+0, true); game.set(ox+2, oy+0, true);
    game.set(ox+0, oy+1, true); game.set(ox+3, oy+1, true);
    game.set(ox+1, oy+2, true);                     game.set(ox+3, oy+2, true);
                              game.set(ox+2, oy+3, true);

    // Boat
    let ox = 26; let oy = 2;
    game.set(ox+0, oy+0, true); game.set(ox+1, oy+0, true);
    game.set(ox+0, oy+1, true);                     game.set(ox+2, oy+1, true);
                              game.set(ox+1, oy+2, true);

    // Tub
    let ox = 34; let oy = 2;
    game.set(ox+1, oy+0, true);
    game.set(ox+0, oy+1, true);                     game.set(ox+2, oy+1, true);
    game.set(ox+1, oy+2, true);

    // 2) OSCILLATORS
    // Blinker
    let ox = 2; let oy = 10;
    game.set(ox+0, oy+0, true);
    game.set(ox+1, oy+0, true);
    game.set(ox+2, oy+0, true);

    // Toad
    let ox = 10; let oy = 10;
    game.set(ox+1, oy+0, true); game.set(ox+2, oy+0, true); game.set(ox+3, oy+0, true);
    game.set(ox+0, oy+1, true); game.set(ox+1, oy+1, true); game.set(ox+2, oy+1, true);

    // Beacon
    let ox = 18; let oy = 10;
    game.set(ox+0, oy+0, true); game.set(ox+1, oy+0, true);
    game.set(ox+0, oy+1, true); game.set(ox+1, oy+1, true);
    game.set(ox+2, oy+2, true); game.set(ox+3, oy+2, true);
    game.set(ox+2, oy+3, true); game.set(ox+3, oy+3, true);

    // Pulsar
    let ox = 30; let oy = 10;
    let pulsar = [
        (2,0),(3,0),(4,0),(8,0),(9,0),(10,0),
        (0,2),(5,2),(7,2),(12,2),
        (0,3),(5,3),(7,3),(12,3),
        (0,4),(5,4),(7,4),(12,4),
        (2,5),(3,5),(4,5),(8,5),(9,5),(10,5),
        (2,7),(3,7),(4,7),(8,7),(9,7),(10,7),
        (0,8),(5,8),(7,8),(12,8),
        (0,9),(5,9),(7,9),(12,9),
        (0,10),(5,10),(7,10),(12,10),
        (2,12),(3,12),(4,12),(8,12),(9,12),(10,12),
    ];
    for &(dx,dy) in &pulsar {
        game.set(ox + dx, oy + dy, true);
    }

    // Pentadecathlon
    let ox = 50; let oy = 10;
    let penta = [
        (2,0),(3,0),(2,1),(3,1),(2,3),(3,3),(2,4),(3,4),(2,6),(3,6),(2,7),(3,7),(2,9),(3,9),(2,10),(3,10),
    ];
    for &(dx,dy) in &penta {
        game.set(ox + dx, oy + dy, true);
    }

    // 3) SPACESHIPS
    // Glider
    seed_glider(&mut game, 2, 30);

    // LWSS
    let ox = 10; let oy = 30;
    let lwss = [(1,0),(2,0),(3,0),(4,0),(0,1),(4,1),(4,2),(0,3),(3,3)];
    for &(dx,dy) in &lwss {
        game.set(ox + dx, oy + dy, true);
    }

    // MWSS
    let ox = 20; let oy = 30;
    let mwss = [(1,0),(2,0),(3,0),(4,0),(5,0),(0,1),(5,1),(5,2),(0,3),(4,3)];
    for &(dx,dy) in &mwss {
        game.set(ox + dx, oy + dy, true);
    }

    // HWSS
    let ox = 32; let oy = 30;
    let hwss = [(1,0),(2,0),(3,0),(4,0),(5,0),(6,0),(0,1),(6,1),(6,2),(0,3),(5,3)];
    for &(dx,dy) in &hwss {
        game.set(ox + dx, oy + dy, true);
    }

    // 4) GOSPER GLIDER GUN
    seed_gosper_gun(&mut game, 2, 40);

    // 5) Tiling de muchos gliders por toda la grilla
    for ox in (0..grid_w as usize).step_by(10) {
        for oy in (0..grid_h as usize).step_by(10) {
            seed_glider(&mut game, ox, oy);
        }
    }

    while !window.window_should_close() {
        game.step();                                // siguiente generación
        game.draw(&mut fb);                         // dibuja en el framebuffer lógico
        fb.swap_buffers(&mut window, &thread, window_w, window_h);
        thread::sleep(Duration::from_millis(100));  // control de velocidad
    }
}
