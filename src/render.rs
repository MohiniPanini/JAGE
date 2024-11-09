use crate::render_scale;
use crate::screen::Tile;
use crate::screen::Screen;
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
    fn draw_dot(&mut self, x: i32, y: i32, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.fill_rect(Rect::new(
            x * (render_scale as i32),
            y * (render_scale as i32),
            render_scale,
            render_scale
        )).unwrap();
    }
    fn draw_test(&mut self) {
        let tile = Tile {data: [0x3C, 0x7E, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x7E, 0x5E, 0x7E, 0x0A, 0x7C, 0x56, 0x38, 0x7C]};
        self.draw_tile(tile, 0, 0);
    }
    fn draw_tile(&mut self, tile: Tile, x: i32, y: i32) {
        let gb_pocket_palette = Palette {colors: [
                Color {r: 0xe0, g: 0xdb, b: 0xcd, a: 0},
                Color {r: 0xa8, g: 0x9f, b: 0x94, a: 255},
                Color {r: 0x70, g: 0x6b, b: 0x66, a: 255},
                Color {r: 0x2b, g: 0x2b, b: 0x26, a: 255},
            ]
        };

        for i in 0..8 {
            let low_byte = tile.data[2 * i];
            let high_byte = tile.data[2 * i + 1];
            let mut filter: u8 = 0b1000000;
            for j in 0..8 {
                let high_bit = ((high_byte >> 7-j) & 1) << 1;
                let low_bit = (low_byte >> 7-j) & 1;
                let pixel = high_bit + low_bit;
                self.draw_dot(
                    x + (j as i32), 
                    y + (i as i32),
                    gb_pocket_palette.colors[pixel as usize]);
                filter = filter >> 1
            }
        }
    }

    //TODO: raster effects won't work yet
    pub fn render(&mut self, screen: Screen) {
        let mut wd_tilemap = [[0 as u8; 32]; 32];
        let mut bg_tilemap = wd_tilemap;
        let mut wd_tiledata: [Tile; 128] = [Default::default(); 128];
        let mut bg_tiledata = wd_tiledata.clone();
        let mut draw_wd = true;
        let mut draw_bg = true;
        let mut draw_objs = true;

        //process lcdc
        if screen.lcdc & 0b10000000 == 0 {
            self.canvas.set_draw_color(Color::WHITE);
            self.canvas.clear();
            self.canvas.present();
            return;
        }

        if screen.lcdc & 0b01000000 == 0 {
            wd_tilemap = screen.tilemaps[0];
        }
        else {
            wd_tilemap = screen.tilemaps[1];
        }

        draw_wd = screen.lcdc & 0b00100000 == 1;

        if screen.lcdc & 0b00010000 != 0 {
            bg_tiledata = screen.tiledata[0];
            wd_tiledata = screen.tiledata[1];
        } else {
            bg_tiledata = screen.tiledata[1];
            wd_tiledata = screen.tiledata[2];
        }

        if screen.lcdc & 0b00001000 == 0 {
            bg_tilemap = screen.tilemaps[0];
        }
        else {
            bg_tilemap = screen.tilemaps[1];
        }

        let two_tile_objs = screen.lcdc & 0b00000100 != 0;

        draw_objs = screen.lcdc & 0b00000010 != 0;

        if screen.lcdc & 0b00000001 == 0 {
            draw_wd = false;
            draw_bg = false;
        }

        //bg rendering!
        if draw_bg {
            let starting_row = screen.scy as usize / 8;
            let y_offset = screen.scy as i32 % 8; //number of pixels to offset row by; subtract this from y-coord of row
            let starting_column = screen.scx as usize / 8;
            let x_offset = screen.scx as i32 % 8;
    
            //we are simplifying rendering by treating it atomically for rn
            //TODO: implement scanline-by-scanline rendering for accuracy + rastering
            //row will then be an argument passed in from outside
            for row in 0..18 { //one extra because we might need to render part of a 19th row
                let tile_row = bg_tilemap[(starting_row + row) % 18];
                for column in 0..20 { //also one extra for the same reason
                    let tile_id = tile_row[(starting_column + column) % 20] as usize;
                    self.draw_tile(bg_tiledata[tile_id], (column as i32 * 8) - x_offset, (row as i32 * 8) - y_offset);
                }
            }
        }


        //self.draw_test();
        self.canvas.present();
    }
}