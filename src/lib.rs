use limage::*;
pub trait Display {
    type Disp;

    fn new(width: u32, height: u32) -> Result<Self::Disp, String>;
    fn save(&self, path: &str);
    fn turn_lamp_on(&mut self, x: u32, y: u32);
    fn turn_lamp_off(&mut self, x: u32, y: u32);
}
pub struct RedstoneDisplay {
    img: LimageRgb,
    lamp_texture_on: LimageRgb,
    lamp_texture_off: LimageRgb,
}
impl Display for RedstoneDisplay {
    type Disp = RedstoneDisplay;

    fn new(width: u32, height: u32) -> Result<Self, String> {
        let lamp_texture_on = LimageRgb::open(&"texture/lamp_on.png".to_string())?;
        let lamp_texture_off = LimageRgb::open(&"texture/lamp_off.png".to_string())?;

        let mut img = LimageRgb::new(2 * 16 * width, 2 * 16 * height);
        for y in 0..height {
            for x in 0..width {
                paste4(x, y, &mut img, &lamp_texture_off);
            }
        }

        Ok(RedstoneDisplay {
            img,
            lamp_texture_on,
            lamp_texture_off,
        })
    }
    fn save(&self, path: &str) {
        self.img.save(path).unwrap()
    }
    fn turn_lamp_on(&mut self, x: u32, y: u32) {
        paste4(x, y, &mut self.img, &self.lamp_texture_on);
    }
    fn turn_lamp_off(&mut self, x: u32, y: u32) {
        paste4(x, y, &mut self.img, &self.lamp_texture_off);
    }
}

pub struct PixelDisplay {
    img: LimageRgb,
}
impl Display for PixelDisplay {
    type Disp = PixelDisplay;

    fn new(width: u32, height: u32) -> Result<Self, String> {
        let img = LimageRgb::new(width, height);
        Ok(PixelDisplay { img })
    }
    fn save(&self, path: &str) {
        self.img.save(path).unwrap()
    }
    fn turn_lamp_on(&mut self, x: u32, y: u32) {
        self.img.put_rgb((x as i32, y as i32), [255, 255, 255]);
    }
    fn turn_lamp_off(&mut self, x: u32, y: u32) {
        self.img.put_rgb((x as i32, y as i32), [0, 0, 0]);
    }
}

fn paste4(x: u32, y: u32, img: &mut LimageRgb, texture: &LimageRgb) {
    img.paste((32 * x as i32, 32 * y as i32), texture);
    img.paste((32 * x as i32 + 16, 32 * y as i32), texture);
    img.paste((32 * x as i32, 32 * y as i32 + 16), texture);
    img.paste((32 * x as i32 + 16, 32 * y as i32 + 16), texture);
}
