use crate::render_scale;
use crate::tilemap::Tile;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::rect::Rect;

pub struct Palette {
    colors: [Color; 4]
}

pub struct Renderer {
    canvas: Canvas<Window>,
}

impl Renderer {
    pub fn new(window: Window) -> Renderer {
        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();
        return Renderer {canvas}
    }
    fn draw_dot(&mut self, x: u32, y: u32, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.fill_rect(Rect::new(
            (x * render_scale) as i32,
            (y * render_scale) as i32,
            render_scale,
            render_scale
        )).unwrap();
    }
    fn draw_test(&mut self) {
        let data = [0x3C, 0x7E, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x7E, 0x5E, 0x7E, 0x0A, 0x7C, 0x56, 0x38, 0x7C];
        let tile = Tile {data};
        self.draw_tile(tile, 0, 0);
    }
    fn draw_tile(&mut self, tile: Tile, x: u32, y: u32) {
        let gb_pocket_palette = Palette {colors: [
                Color {r: 0xe0, g: 0xdb, b: 0xcd, a: 255},
                Color {r: 0xa8, g: 0x9f, b: 0x94, a: 255},
                Color {r: 0x70, g: 0x6b, b: 0x66, a: 255},
                Color {r: 0x2b, g: 0x2b, b: 0x26, a: 255},
            ]
        };
        let test_palette = Palette {colors: [
            Color::RED,
            Color::BLUE,
            Color::YELLOW,
            Color::GREEN
        ]};

        for i in 0..8 {
            let low_byte = tile.data[2 * i];
            let high_byte = tile.data[2 * i + 1];
            let mut filter: u8 = 0b1000000;
            for j in 0..8 {
                let high_bit = ((high_byte >> 7-j) & 1) << 1;
                let low_bit = (low_byte >> 7-j) & 1;
                let pixel = (high_bit + low_bit) as usize;
                println!("{:}", pixel);
                self.draw_dot(
                    x + (j as u32), 
                    y + (i as u32),
                    gb_pocket_palette.colors[pixel as usize]);
                filter = filter >> 1
            }
        }
    }
    pub fn render(&mut self) {
        self.draw_test();
        self.canvas.present();
    }
}