

pub const TEST_TILE: Tile = Tile {data: [0x3C, 0x7E, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x7E, 0x5E, 0x7E, 0x0A, 0x7C, 0x56, 0x38, 0x7C]};

#[derive(Default, Copy, Clone)]
pub struct Tile {
    pub data: [u8; 16]
}

pub struct Screen {
    pub tiledata: [[Tile; 128]; 3], //$8000-$97FF
    pub tilemaps: [[[u8; 32]; 32]; 2], //$9800-$9FFF
    pub lcdc: u8, //$FF40
    pub scy: u8, //$FF42
    pub scx: u8, //$FF43
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            tiledata: [[Default::default(); 128]; 3],
            tilemaps: [[[0; 32]; 32]; 2],
            lcdc: 0,
            scy: 0,
            scx: 0,
        }
    }
    pub fn test_screen() -> Screen {
        let mut screen = Screen::new();
        screen.tiledata[0][0] = TEST_TILE;
        screen.lcdc = 0b10010011;
        screen.scx = 0b00000110;
        screen.scy = 0b00000110;
        return screen;
    }
}