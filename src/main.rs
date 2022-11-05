
use lamp_display::RedstoneDisplay;

fn main() {
    let mut display = RedstoneDisplay::new(64, 64).unwrap();
    for (x, y) in limage::circle((8, 8), 6) {
        display.turn_lamp_on(x as u32, y as u32);
    }
    for (x, y) in limage::path(&vec![(1,1),(50, 39), (10, 30), (59, 2)]) {
        display.turn_lamp_on(x as u32, y as u32);
    }
    
    display.save("test.png");
    println!("Hello, world!");
}
