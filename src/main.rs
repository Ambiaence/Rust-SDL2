mod sdl_two;
use rustSDL::bitmap::Bitmap;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let sdl = sdl_two::SdlTwo::new();  
    let mut map = Bitmap::new(800, 600);
    for i in 100..=200 {
        for j in 0..=10 {
            map.drawPixel(200 + j, i, 255 as u8, 0x66 as u8, 0x99 as u8, 0xff as u8 );
        }
    }
    sdl.drawBitmap(&map);
    sleep(Duration::from_secs(10));

}
