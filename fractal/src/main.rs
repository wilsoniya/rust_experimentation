extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use std::cmp::min;

static PIXEL_SIZE: u32 = 1;
static WIN_WIDTH: u32 = 1024;
static WIN_HEIGHT: u32 = 768;

fn mandelbrot(renderer: &mut sdl2::render::Renderer) {
    let max_iteration = 255 * 3;
    let m_x0 = -2.5_f64;
    let m_x = 1_f64;
    let m_y0 = -1_f64;
    let m_y = 1_f64;

    for px in 0_u32..WIN_WIDTH {
        for py in 0_u32..WIN_HEIGHT {
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

fn set_pixel(renderer: &mut sdl2::render::Renderer, x: u32, y: u32, r: u8, g: u8, b: u8) {
    renderer.set_draw_color(Color::RGB(r, g, b));
    let pixel = Rect::new(x as i32, y as i32, PIXEL_SIZE, PIXEL_SIZE).unwrap().unwrap();
    renderer.fill_rect(pixel);
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo: Video",
                                        WIN_WIDTH, WIN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();

    renderer.set_draw_color(Color::RGB(0, 0, 0));
    renderer.clear();

    mandelbrot(&mut renderer);
    renderer.present();

    let mut running = true;
    let mut event_pump = sdl_context.event_pump().unwrap();

    while running {
        for event in event_pump.poll_iter() {

            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    running = false
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
    }
}
