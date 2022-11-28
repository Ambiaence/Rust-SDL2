mod sdl_two;
use rustSDL::bitmap::Bitmap;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let sdl = sdl_two::SdlTwo::new();  
    let mut map = Bitmap::new(800, 600);
    sleep(Duration::from_secs(1));
    sdl.drawBitmap(&map);
    sleep(Duration::from_secs(1));
    for i in 0..=100 {
        for y in 0..=3 {
            map.drawPixel(400 + y, i, 255 as u8, 0x66 as u8, 0x99 as u8, 0xff as u8 );
        }
    }
    sleep(Duration::from_secs(5));
    for i in 0..=100 {
        for y in 0..=3 {
            map.drawPixel(400 + y, i, 255 as u8, 0x66 as u8, 0x99 as u8, 0xff as u8 );
    sdl.drawBitmap(&map);
    sleep(Duration::from_secs(5));

}
