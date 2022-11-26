mod sdl_two;
mod bitmap;

fn main() {
    let map = bitmap::Bitmap::new(800, 600);
    let sdl = sdl_two::SdlTwo::new();  
    sdl.draw();
}
