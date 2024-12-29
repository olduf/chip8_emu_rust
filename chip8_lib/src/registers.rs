pub struct Registers {
    vx: Vec<u8>,

    // address are 12 bits wide
    i: u16,
    pc: u16,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            vx: vec![0; 16],
            i: 0,
            pc: 0x200,
        }
    }

    pub fn get_vx(&self, index: usize) -> u8 {
        self.vx[index]
    }

    pub fn set_vx(&mut self, index: usize, value: u8) {
        self.vx[index] = value;
    }

    pub fn get_i(&self) -> u16 {
        self.i
    }

    pub fn set_i(&mut self, value: u16) {
        self.i = value;
    }

    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    pub fn set_pc(&mut self, value: u16) {
        self.pc = value;
    }
}
