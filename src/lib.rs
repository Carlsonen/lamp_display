
use limage::*;

struct RedstoneDisplay {
    img: Limage,
    lamp_texture_on: Limage,
    lamp_texture_off: Limage
}
impl RedstoneDisplay {
    pub fn new(width: u32, height: u32) -> Result<Self, String> {
        let lamp_texture_on = Limage::open(&"texture/lamp_on.png".to_string())?;
        let lamp_texture_off = Limage::open(&"texture/lamp_off.png".to_string())?;

        let mut img = Limage::new(32 * width, 32 * height);
        for y in 0..height {
            for x in 0..width {
                img.paste((32 * x as i32, 32 * y as i32), &lamp_texture_off);
                img.paste((32 * x as i32 + 16, 32 * y as i32), &lamp_texture_off);
                img.paste((32 * x as i32, 32 * y as i32 + 16), &lamp_texture_off);
                img.paste((32 * x as i32 + 16, 32 * y as i32 + 16), &lamp_texture_off);
            }
        }

        Ok(RedstoneDisplay { 
            img: img, 
            lamp_texture_on: lamp_texture_on, 
            lamp_texture_off: lamp_texture_off 
        })
    }
    pub fn save(&self, path: &str) {
        self.img.save(path).unwrap()
    }
    pub fn turn_lamp_on(&mut self, x: u32, y: u32) {
        self.img.paste((32 * x as i32, 32 * y as i32), &self.lamp_texture_on);
        self.img.paste((32 * x as i32 + 16, 32 * y as i32), &self.lamp_texture_on);
        self.img.paste((32 * x as i32, 32 * y as i32 + 16), &self.lamp_texture_on);
        self.img.paste((32 * x as i32 + 16, 32 * y as i32 + 16), &self.lamp_texture_on);
    }
    pub fn turn_lamp_off(&mut self, x: u32, y: u32) {
        self.img.paste((32 * x as i32, 16 * y as i32), &self.lamp_texture_off);
        self.img.paste((32 * x as i32 + 16, 16 * y as i32), &self.lamp_texture_off);
        self.img.paste((32 * x as i32, 16 * y as i32 + 16), &self.lamp_texture_off);
        self.img.paste((32 * x as i32 + 16, 16 * y as i32 + 16), &self.lamp_texture_off);
    }
}
