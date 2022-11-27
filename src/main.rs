mod sdl_two;
mod bitmap;

fn main() {
    let sdl = sdl_two::SdlTwo::new();  
    sdl.draw();
}
