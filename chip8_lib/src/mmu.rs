pub struct Mmu {
    key_pressed: Vec<bool>,

    memory: Vec<u8>,
    screen: Vec<u8>,

    delay_timer: u8,
    sound_timer: u8,

    stack: Vec<u16>,
    stack_pointer: usize,
}

impl Mmu {
    pub fn new(rom: &Vec<u8>) -> Self {
        let mut temp: Vec<u8> = vec![0; 4096];

        // copy rom content into memory (starting at 0x200)
        for index in 0..rom.len() {
            temp[0x200 + index] = rom[index];
        }

        // Setup font in memory - conventionally stored in [0x50 - 0x9F]
        // 0
        temp[0x50] = 0xF0;
        temp[0x51] = 0x90;
        temp[0x52] = 0x90;
        temp[0x53] = 0x90;
        temp[0x54] = 0xF0;
        // 1
        temp[0x55] = 0x20;
        temp[0x56] = 0x60;
        temp[0x57] = 0x20;
        temp[0x58] = 0x20;
        temp[0x59] = 0x70;
        // 2
        temp[0x5A] = 0xF0;
        temp[0x5B] = 0x10;
        temp[0x5C] = 0xF0;
        temp[0x5D] = 0x80;
        temp[0x5E] = 0xF0;
        // 3
        temp[0x5F] = 0xF0;
        temp[0x60] = 0x10;
        temp[0x61] = 0xF0;
        temp[0x62] = 0x10;
        temp[0x63] = 0xF0;
        // 4
        temp[0x64] = 0x90;
        temp[0x65] = 0x90;
        temp[0x66] = 0xF0;
        temp[0x67] = 0x10;
        temp[0x68] = 0x10;
        // 5
        temp[0x69] = 0xF0;
        temp[0x6A] = 0x80;
        temp[0x6B] = 0xF0;
        temp[0x6C] = 0x10;
        temp[0x6D] = 0xF0;
        // 6
        temp[0x6E] = 0xF0;
        temp[0x6F] = 0x80;
        temp[0x70] = 0xF0;
        temp[0x71] = 0x90;
        temp[0x72] = 0xF0;
        // 7
        temp[0x73] = 0xF0;
        temp[0x74] = 0x10;
        temp[0x75] = 0x20;
        temp[0x76] = 0x40;
        temp[0x77] = 0x40;
        // 8
        temp[0x78] = 0xF0;
        temp[0x79] = 0x90;
        temp[0x7A] = 0xF0;
        temp[0x7B] = 0x90;
        temp[0x7C] = 0xF0;
        // 9
        temp[0x7D] = 0xF0;
        temp[0x7E] = 0x90;
        temp[0x7F] = 0xF0;
        temp[0x80] = 0x10;
        temp[0x81] = 0xF0;
        // A
        temp[0x82] = 0xF0;
        temp[0x83] = 0x90;
        temp[0x84] = 0xF0;
        temp[0x85] = 0x90;
        temp[0x86] = 0x90;
        // B
        temp[0x87] = 0xE0;
        temp[0x88] = 0x90;
        temp[0x89] = 0xE0;
        temp[0x8A] = 0x90;
        temp[0x8B] = 0xE0;
        // C
        temp[0x8C] = 0xF0;
        temp[0x8D] = 0x80;
        temp[0x8E] = 0x80;
        temp[0x8F] = 0x80;
        temp[0x90] = 0xF0;
        // D
        temp[0x91] = 0xE0;
        temp[0x92] = 0x90;
        temp[0x93] = 0x90;
        temp[0x94] = 0x90;
        temp[0x95] = 0xE0;
        // E
        temp[0x96] = 0xF0;
        temp[0x97] = 0x80;
        temp[0x98] = 0xF0;
        temp[0x99] = 0x80;
        temp[0x9A] = 0xF0;
        // F
        temp[0x9B] = 0xF0;
        temp[0x9C] = 0x80;
        temp[0x9D] = 0xF0;
        temp[0x9E] = 0x80;
        temp[0x9F] = 0x80;

        Self {
            key_pressed: vec![false; 16],
            memory: temp.clone(),
            screen: vec![0; 32 * 64],
            delay_timer: 0,
            sound_timer: 0,
            stack: vec![0; 16],
            stack_pointer: 0,
        }
    }

    pub fn get_byte(&self, address: usize) -> u8 {
        self.memory[address & 0x0FFF]
    }

    pub fn set_byte(&mut self, address: usize, value: u8) {
        self.memory[address & 0x0FFF] = value;
    }

    pub fn get_short(&self, address: usize) -> u16 {
        let masked_address: usize = address & 0x0FFF;

        ((self.memory[masked_address] as u16) << 8) | self.memory[masked_address + 1] as u16
    }

    pub fn get_screen(&self) -> &Vec<u8> {
        &self.screen
    }

    pub fn set_screen(&mut self, new_screen: Vec<u8>) {
        self.screen = new_screen;
    }

    pub fn get_screen_at(&self, index: usize) -> u8 {
        self.screen[index]
    }

    pub fn set_screen_at(&mut self, index: usize, value: u8) {
        self.screen[index] = value;
    }

    pub fn push_on_stack(&mut self, value: u16) {
        if self.stack_pointer < 16 {
            self.stack[self.stack_pointer] = value;
            self.stack_pointer = self.stack_pointer + 1;
        }
    }

    pub fn pop_stack(&mut self) -> u16 {
        let mut value: u16 = 0;

        if self.stack_pointer > 0 {
            value = self.stack[self.stack_pointer - 1];
            self.stack_pointer = self.stack_pointer - 1;
        }

        value
    }

    pub fn get_delay_timer(&self) -> u8 {
        self.delay_timer
    }

    pub fn set_delay_timer(&mut self, value: u8) {
        self.delay_timer = value;
    }

    pub fn decrement_delay_timer(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
    }

    pub fn get_sound_timer(&self) -> u8 {
        self.sound_timer
    }

    pub fn set_sound_timer(&mut self, value: u8) {
        self.sound_timer = value;
    }

    pub fn decrement_sound_timer(&mut self) {
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    pub fn is_key_down(&self, index: usize) -> bool {
        self.key_pressed[index & 0x0F]
    }

    pub fn set_key_down(&mut self, index: usize, value: bool) {
        self.key_pressed[index & 0x0F] = value;
    }
}
