use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::cell::RefCell;

pub struct SdlTwo { 
    sdl_context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    canvas: RefCell<sdl2::render::Canvas<sdl2::video::Window>>,
    event_pump: RefCell<sdl2::EventPump>, 
}

impl SdlTwo {
    pub fn new() -> SdlTwo{
        //Create things on the outside and then pass ownership to SdlTwo
        let con = sdl2::init().unwrap();
        let vid = con.video().unwrap();
        let win = vid.window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .build()
            .unwrap();

        let can = win.into_canvas().build().unwrap();
        let eve = con.event_pump().unwrap();

        let instance = SdlTwo {
            sdl_context: con,
            video_subsystem: vid,
            canvas: RefCell::new(can), 
            event_pump: RefCell::new(eve),
        };   
        instance
    }

   pub fn draw(&self) {
        self.canvas.borrow_mut().set_draw_color(Color::RGB(0, 255, 255));
        self.canvas.borrow_mut().clear();
        self.canvas.borrow_mut().present();
        loop{}
    }

}