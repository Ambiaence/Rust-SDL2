pub struct Bitmap {
    pub width: u32,
    pub height: u32,
    pub number_of_pixels: usize,
    pub components: Vec<u8>,
}

impl Bitmap {
    pub fn new(w: u32, h: u32) -> Self { 
        let p = (4*h*w) as usize;
        Self {   
            width: w,
            height: h,
            number_of_pixels: p,
            components: vec![0;p], 
       }
    }

    pub fn clear(&mut self, shade: u8 ) {
        self.components = vec![shade; self.number_of_pixels]
    }
    
    pub fn drawPixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8, a: u8) {
        let i = (x + y * self.width * 4) as usize;
        self.components[i + 0] = r; 
        self.components[i + 1] = g; 
        self.components[i + 2] = b; 
        self.components[i + 3] = a; 
    }
}
