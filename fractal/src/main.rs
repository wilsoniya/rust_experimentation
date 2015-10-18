extern crate sdl2;
extern crate native;

use std::cmp::min;
use std::num::abs;

use sdl2::video::{Window, PosCentered, OPENGL};
use sdl2::event::{QuitEvent, poll_event};
use sdl2::rect::{Rect};
use sdl2::timer::delay;

static PIXEL_SIZE: i32 = 1;
static WIN_WIDTH: i32 = 1024;
static WIN_HEIGHT: i32 = 768;

fn mandelbrot(renderer: &mut sdl2::render::Renderer) {
    let max_iteration = 255 * 3;
    let m_x0 = -2.5_f64;
    let m_x = 1_f64;
    let m_y0 = -1_f64;
    let m_y = 1_f64;

    for px in range(0_i32, WIN_WIDTH) {
        for py in range(0_i32, WIN_HEIGHT) {
            let x0 = (m_x - m_x0) / (WIN_WIDTH as f64) * (px as f64) + m_x0;
            let y0 = (m_y - m_y0) / (WIN_HEIGHT as f64) * (py as f64) + m_y0; 
            let mut x = 0_f64;
            let mut y = 0_f64;

            let mut i = 0i32;
            while (x*x + y*y < (2.0 * 2.0)) && (i < max_iteration) {
                let xtemp = x*x - y*y + x0;
                y = 2.0 * x * y + y0;
                x = xtemp; 
                i += 10;
            }

            // coloring of sampled pixel
            if i > 0 {
                let t_i: u8 = (i % 255) as u8;
                let r = min(i, 255) as u8;
                let g = if i - 255 < 0 {
                    0 
                } else if i - 255 > 255 {
                    255
                } else {
                    i - 255
                } as u8;
                let b = if i - 510 < 0 {
                    0 
                } else if i - 510 > 255 {
                    255
                } else {
                    i - 510
                } as u8;
                set_pixel(renderer, px, py, r, g, b); 
            }
        }
    } 
    println!("Mandelbrot complete");
}

fn set_pixel(renderer: &mut sdl2::render::Renderer, x: i32, y: i32, r: u8, g: u8, b: u8) {
    renderer.set_draw_color(sdl2::pixels::RGB(r, g, b));
    let pixel = Rect::new(x, y, PIXEL_SIZE, PIXEL_SIZE);
    match renderer.fill_rect(&pixel) {
        Ok(_) => {},
        Err(err) => fail!("failed to draw rect: {}", err)
    } 
}

fn main() {
    // start sdl2 with everything
    sdl2::init(sdl2::INIT_EVERYTHING);

    // Create a window
    let window  = match Window::new(
            "eg03", PosCentered, PosCentered, WIN_WIDTH as int, WIN_HEIGHT as int, OPENGL) {
        Ok(window) => window,
        Err(err)   => fail!("failed to create window: {}", err)
    };

    // Create a rendering context
    let mut renderer = match sdl2::render::Renderer::from_window( 
            window, sdl2::render::DriverAuto, sdl2::render::ACCELERATED) {
        Ok(renderer) => renderer,
        Err(err) => fail!("failed to create renderer: {}", err)
    };

    renderer.set_draw_color(sdl2::pixels::RGB(0, 0, 0));
    renderer.clear();

    mandelbrot(&mut renderer); 
    renderer.present();

    'event : loop {
        match poll_event() {
            QuitEvent(_) => break 'event,
            _            => delay(100),
        }
    }

    sdl2::quit(); 
} 
