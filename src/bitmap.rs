use std::cell::RefCell;

struct pixel {
}

pub struct Bitmap {
    width: u32,
    height: u32,
    components: RefCell<Vec<u8>>,
}

impl Bitmap {
    pub fn new(w: u32, h: u32) -> Self { 
        let number_of_pixels = (4*h*w) as usize;

        Self {   
            width: w,
            height: h,
            components: RefCell::new(vec![0;number_of_pixels]), 
       }
    }

    pub fn clear(&self, ) {
            
    }
    
    pub fn DrawPixel(&self, x: u32, y: u32, a: u8, r: u8, g: u8, b: u8) {
        let i = (x + y * self.width * 4) as usize;
        self.components.borrow_mut()[i] = a; 
        self.components.borrow_mut()[i + 1] = r; 
        self.components.borrow_mut()[i + 2] = g; 
        self.components.borrow_mut()[i + 3] = b; 
    }
}
