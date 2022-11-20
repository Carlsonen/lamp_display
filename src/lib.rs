use limage::prelude::*;

pub struct RedstoneDisplay {
    img: Limage,
    lamp_texture_on: Limage,
    lamp_texture_off: Limage
}
impl RedstoneDisplay {
    pub fn new(width: u32, height: u32) -> Result<Self, String> {
        let lamp_texture_on = Limage::open(&"texture/lamp_on.png".to_string())?;
        let lamp_texture_off = Limage::open(&"texture/lamp_off.png".to_string())?;

        let mut img = Limage::new(2 * 16 * width, 2 * 16 * height);
        for y in 0..height {
            for x in 0..width {
                paste4(x, y, &mut img, &lamp_texture_off);
            }
        }

        Ok(RedstoneDisplay { 
            img, 
            lamp_texture_on, 
            lamp_texture_off
        })
    }
    pub fn save(&self, path: &str) {
        self.img.save(path).unwrap()
    }
    pub fn turn_lamp_on(&mut self, x: u32, y: u32) {
        paste4(x, y, &mut self.img, &self.lamp_texture_on);
    }
    pub fn turn_lamp_off(&mut self, x: u32, y: u32) {
        paste4(x, y, &mut self.img, &self.lamp_texture_off);
    }
}

fn paste4(x: u32, y: u32, img: &mut Limage, texture: &Limage) {
    img.paste((32 * x as i32,      32 * y as i32),      texture);
    img.paste((32 * x as i32 + 16, 32 * y as i32),      texture);
    img.paste((32 * x as i32,      32 * y as i32 + 16), texture);
    img.paste((32 * x as i32 + 16, 32 * y as i32 + 16), texture);
}