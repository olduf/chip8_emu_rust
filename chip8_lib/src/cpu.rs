use crate::mmu::Mmu;
use crate::registers::Registers;
use rand::Rng;

pub struct Cpu {}

impl Cpu {
    pub fn tick(&mut self, mmu: &mut Mmu, registers: &mut Registers) {
        let pc = registers.get_pc();
        let op_code = mmu.get_short(pc as usize);

        // increment pc after reading the opcode
        registers.set_pc(pc + 2);

        // get the correct instruction to execute
        let instruction = match op_code & 0xF000 {
            0x0000 => match op_code & 0x00FF {
                // CLS
                0xE0 => instruction_00e0,
                // RET
                0xEE => instruction_00ee,
                // no match
                _ => instruction_invalid,
            },
            // JP addr
            0x1000 => instruction_1nnn,
            // CALL addr
            0x2000 => instruction_2nnn,
            // SE Vx, byte
            0x3000 => instruction_3nnn,
            // SNE Vx, byte
            0x4000 => instruction_4nnn,
            // SE Vx, Vy
            0x5000 => instruction_5xy0,
            // LD Vx, byte
            0x6000 => instruction_6xnn,
            // ADD Vx, byte
            0x7000 => instruction_7xnn,
            // Logical and arithmetic instructions
            0x8000 => match op_code & 0x000F {
                // LD Vx, Vy
                0x0 => instruction_8xy0,
                // OR Vx, Vy
                0x1 => instruction_8xy1,
                // AND Vx, Vy
                0x2 => instruction_8xy2,
                // XOR Vx, Vy
                0x3 => instruction_8xy3,
                // ADD Vx, Vy
                0x4 => instruction_8xy4,
                // SUB Vx, Vy
                0x5 => instruction_8xy5,
                // SHR Vx, {, Vy}
                0x6 => instruction_8xy6,
                // SUBN Vx, Vy
                0x7 => instruction_8xy7,
                // SHL Vx {, Vy}
                0xE => instruction_8xye,
                // no match
                _ => instruction_invalid,
            },
            // SNE Vx, Vy
            0x9000 => instruction_9xy0,
            // LD I, addr
            0xA000 => instruction_annn,
            // JP V0, addr
            0xB000 => instruction_bnnn,
            // RND Vx, byte
            0xC000 => instruction_cxnn,
            // DRW Vx, Vy, nibble
            0xD000 => instruction_dxyn,
            0xE000 => match op_code & 0x00FF {
                // SKP Vx
                0x9E => instruction_ex9e,
                // SKNP Vx
                0xA1 => instruction_exa1,
                // no match
                _ => instruction_invalid,
            },
            0xF000 => match op_code & 0x00FF {
                // LD Vx, DT
                0x07 => instruction_fx07,
                // LD Vx, K -> Get key (blocking)
                0x0A => instruction_fx0a,
                // LD DT, Vx
                0x15 => instruction_fx15,
                // LD ST, Vx
                0x18 => instruction_fx18,
                // ADD I, Vx
                0x1E => instruction_fx1e,
                // LD F, Vx
                0x29 => instruction_fx29,
                // LD B, Vx
                0x33 => instruction_fx33,
                // LD [I], Vx
                0x55 => instruction_fx55,
                // LD Vx, [I]
                0x65 => instruction_fx65,
                // no match
                _ => instruction_invalid,
            },
            _ => instruction_invalid,
        };

        // excute instruction
        instruction(mmu, registers, op_code);
    }
}

fn instruction_invalid(_mmu: &mut Mmu, registers: &mut Registers, op_code: u16) {
    println!(
        "invalid opcode: {:#04x} at {:#04x}",
        op_code,
        registers.get_pc() - 2
    );
}

fn instruction_00e0(mmu: &mut Mmu, _registers: &mut Registers, _op_code: u16) {
    mmu.set_screen(vec![0; 32 * 64]);
}

fn instruction_00ee(mmu: &mut Mmu, registers: &mut Registers, _op_code: u16) {
    registers.set_pc(mmu.pop_stack() & 0x0FFF);
}

fn instruction_1nnn(_mmu: &mut Mmu, registers: &mut Registers, op_code: u16) {
    registers.set_pc(op_code & 0x0FFF);
}

fn instruction_2nnn(mmu: &mut Mmu, registers: &mut Registers, op_code: u16) {
    mmu.push_on_stack(registers.get_pc());
    registers.set_pc(op_code & 0x0FFF);
}

fn instruction_3nnn(_mmu: &mut Mmu, registers: &mut Registers, op_code: u16) {
    let index = ((op_code & 0x0F00) >> 8) as usize;
    let value = (op_code & 0x00FF) as u8;

    if registers.get_vx(index) == value {
        let pc = registers.get_pc();
        registers.set_pc(pc + 2);
    }
}

fn instruction_4nnn(_mmu: &mut Mmu, registers: &mut Registers, op_code: u16) {
    let index = ((op_code & 0x0F00) >> 8) as usize;
    let value = (op_code & 0x00FF) as u8;

    if registers.get_vx(index) != value {
        let pc = registers.get_pc();
        registers.set_pc(pc + 2);
    }
}

fn instruction_5xy0(_mmu: &mut Mmu, registers: &mut Registers, op_code: u16) {
    let x_index = ((op_code & 0x0F00) >> 8) as usize;
    let y_index = ((op_code & 0x00F0) >> 4) as usize;

    if registers.get_vx(x_index) == registers.get_vx(y_index) {
        let pc = registers.get_pc();
        registers.set_pc(pc + 2);
    }
}

fn instruction_6xnn(_mmu: &mut Mmu, registers: &mut Registers, op_code: u16) {
    let index = ((op_code & 0x0F00) >> 8) as usize;
    let value = (op_code & 0x00FF) as u8;

    registers.set_vx(index, value);
}

fn instruction_7xnn(_mmu: &mut Mmu, registers: &mut Registers, op_code: u16) {
    let index = ((op_code & 0x0F00) >> 8) as usize;
    let operand = (op_code & 0x00FF) as u8;

    let original_value = registers.get_vx(index);

    registers.set_vx(index, original_value.overflowing_add(operand).0);
}

fn instruction_8xy0(_mmu: &mut Mmu, registers: &mut Registers, op_code: u16) {
    let x_index = ((op_code & 0x0F00) >> 8) as usize;
    let y_index = ((op_code & 0x00F0) >> 4) as usize;

    registers.set_vx(x_index, registers.get_vx(y_index));
}

fn instruction_8xy1(_mmu: &mut Mmu, registers: &mut Registers, op_code: u16) {
    let x_index = ((op_code & 0x0F00) >> 8) as usize;
    let x_value = registers.get_vx(x_index);

    let y_index = ((op_code & 0x00F0) >> 4) as usize;
    let y_value = registers.get_vx(y_index);

    registers.set_vx(x_index, (x_value | y_value) as u8);
}

fn instruction_8xy2(_mmu: &mut Mmu, registers: &mut Registers, op_code: u16) {
    let x_index = ((op_code & 0x0F00) >> 8) as usize;
    let x_value = registers.get_vx(x_index);

    let y_index = ((op_code & 0x00F0) >> 4) as usize;
    let y_value = registers.get_vx(y_index);

    registers.set_vx(x_index, (x_value & y_value) as u8);
}

fn instruction_8xy3(_mmu: &mut Mmu, registers: &mut Registers, op_code: u16) {
    let x_index = ((op_code & 0x0F00) >> 8) as usize;
    let x_value = registers.get_vx(x_index);

    let y_index = ((op_code & 0x00F0) >> 4) as usize;
    let y_value = registers.get_vx(y_index);

    registers.set_vx(x_index, (x_value ^ y_value) as u8);
}

fn instruction_8xy4(_mmu: &mut Mmu, registers: &mut Registers, op_code: u16) {
    let x_index = ((op_code & 0x0F00) >> 8) as usize;
    let x_value = registers.get_vx(x_index);

    let y_index = ((op_code & 0x00F0) >> 4) as usize;
    let y_value = registers.get_vx(y_index);

    let result = x_value.overflowing_add(y_value);
    registers.set_vx(x_index, result.0);

    // set carry in the last register
    registers.set_vx(0xF, result.1 as u8);
}

fn instruction_8xy5(_mmu: &mut Mmu, registers: &mut Registers, op_code: u16) {
    let x_index = ((op_code & 0x0F00) >> 8) as usize;
    let x_value = registers.get_vx(x_index);

    let y_index = ((op_code & 0x00F0) >> 4) as usize;
    let y_value = registers.get_vx(y_index);

    let result = x_value.overflowing_sub(y_value);
    registers.set_vx(x_index, result.0);

    // set carry in the last register
    registers.set_vx(0xF, !result.1 as u8);
}

fn instruction_8xy6(_mmu: &mut Mmu, registers: &mut Registers, op_code: u16) {
    let index = ((op_code & 0x0F00) >> 8) as usize;
    let value = registers.get_vx(index);

    registers.set_vx(index, (value >> 1) as u8);
    registers.set_vx(0xF, (value & 0b00000001) as u8);
}

fn instruction_8xy7(_mmu: &mut Mmu, registers: &mut Registers, op_code: u16) {
    let x_index = ((op_code & 0x0F00) >> 8) as usize;
    let x_value = registers.get_vx(x_index);

    let y_index = ((op_code & 0x00F0) >> 4) as usize;
    let y_value = registers.get_vx(y_index);

    let result = y_value.overflowing_sub(x_value);
    registers.set_vx(x_index, result.0);

    // set carry in the last register
    registers.set_vx(0xF, !result.1 as u8);
}

fn instruction_8xye(_mmu: &mut Mmu, registers: &mut Registers, op_code: u16) {
    let index = ((op_code & 0x0F00) >> 8) as usize;
    let value = registers.get_vx(index);

    registers.set_vx(index, (value << 1) as u8);
    registers.set_vx(0xF, ((value & 0b10000000) >> 7) as u8);
}

fn instruction_9xy0(_mmu: &mut Mmu, registers: &mut Registers, op_code: u16) {
    let x_index = ((op_code & 0x0F00) >> 8) as usize;
    let y_index = ((op_code & 0x00F0) >> 4) as usize;

    if registers.get_vx(x_index) != registers.get_vx(y_index) {
        let pc = registers.get_pc();
        registers.set_pc(pc + 2);
    }
}

fn instruction_annn(_mmu: &mut Mmu, registers: &mut Registers, op_code: u16) {
    let value = (op_code & 0x0FFF) as u16;

    registers.set_i(value);
}

fn instruction_bnnn(_mmu: &mut Mmu, registers: &mut Registers, op_code: u16) {
    let value = (op_code & 0x0FFF) as u16;

    registers.set_pc(value + registers.get_vx(0) as u16);
}

fn instruction_cxnn(_mmu: &mut Mmu, registers: &mut Registers, op_code: u16) {
    let index = ((op_code & 0x0F00) >> 8) as usize;
    let modulo = (op_code & 0x00FF) as u8;

    let rng_value = rand::thread_rng().gen_range(1..=0xFF);
    registers.set_vx(index, (rng_value % modulo) as u8)
}

fn instruction_dxyn(mmu: &mut Mmu, registers: &mut Registers, op_code: u16) {
    let x_index = ((op_code & 0x0F00) >> 8) as usize;
    let x_value = (registers.get_vx(x_index) % 64) as u16;

    let y_index = ((op_code & 0x00F0) >> 4) as usize;
    let y_value = (registers.get_vx(y_index) % 32) as u16;

    let n = op_code & 0x000F;

    // reset carry flag
    registers.set_vx(0xF, 0);

    for j in 0..n {
        let pixel = mmu.get_byte((registers.get_i() + j) as usize);

        for i in 0..8 {
            if (pixel & (0x80 >> i)) != 0 {
                let index = (x_value + i + ((y_value + j) * 64)) as usize;
                let value = mmu.get_screen_at(index);

                if value != 0 {
                    registers.set_vx(0xF, 1);
                }

                mmu.set_screen_at(index, value ^ 1);
            }
        }
    }
}

fn instruction_ex9e(mmu: &mut Mmu, registers: &mut Registers, op_code: u16) {
    let index = ((op_code & 0x0F00) >> 8) as usize;
    let key = registers.get_vx(index) as usize;

    if mmu.is_key_down(key) {
        let pc = registers.get_pc();
        registers.set_pc(pc + 2);
    }
}

fn instruction_exa1(mmu: &mut Mmu, registers: &mut Registers, op_code: u16) {
    let index = ((op_code & 0x0F00) >> 8) as usize;
    let key = registers.get_vx(index) as usize;

    if !mmu.is_key_down(key) {
        let pc = registers.get_pc();
        registers.set_pc(pc + 2);
    }
}

fn instruction_fx07(mmu: &mut Mmu, registers: &mut Registers, op_code: u16) {
    let index = ((op_code & 0x0F00) >> 8) as usize;

    registers.set_vx(index, mmu.get_delay_timer());
}

fn instruction_fx0a(mmu: &mut Mmu, registers: &mut Registers, op_code: u16) {
    let index = ((op_code & 0x0F00) >> 8) as usize;
    let key = registers.get_vx(index) as usize;

    if !mmu.is_key_down(key) {
        let pc = registers.get_pc();
        registers.set_pc(pc - 2);
    }
}

fn instruction_fx15(mmu: &mut Mmu, registers: &mut Registers, op_code: u16) {
    let index = ((op_code & 0x0F00) >> 8) as usize;

    mmu.set_delay_timer(registers.get_vx(index));
}

fn instruction_fx18(mmu: &mut Mmu, registers: &mut Registers, op_code: u16) {
    let index = ((op_code & 0x0F00) >> 8) as usize;

    mmu.set_sound_timer(registers.get_vx(index));
}

fn instruction_fx1e(_mmu: &mut Mmu, registers: &mut Registers, op_code: u16) {
    let index = ((op_code & 0x0F00) >> 8) as usize;
    let result = registers.get_i() + (registers.get_vx(index) as u16);
    let overflow = (result > 0x0FFF) as u8;

    registers.set_i(result & 0x0FFF);
    registers.set_vx(0xF, overflow);
}

fn instruction_fx29(_mmu: &mut Mmu, registers: &mut Registers, op_code: u16) {
    let index = ((op_code & 0x0F00) >> 8) as usize;

    // 0x50 is the memory location we used for the font, 5 is the size of a font in bytes
    registers.set_i(0x50 + (registers.get_vx(index) as u16));
}

fn instruction_fx33(mmu: &mut Mmu, registers: &mut Registers, op_code: u16) {
    let index = ((op_code & 0x0F00) >> 8) as usize;
    let value = registers.get_vx(index);

    let units: u8 = value % 10;
    let tens: u8 = ((value - units) % 100) / 10;
    let hundreds: u8 = (value - tens * 10 - units) / 100;

    let i = registers.get_i() as usize;

    mmu.set_byte(i, hundreds);
    mmu.set_byte(i + 1, tens);
    mmu.set_byte(i + 2, units);
}

fn instruction_fx55(mmu: &mut Mmu, registers: &mut Registers, op_code: u16) {
    let index = ((op_code & 0x0F00) >> 8) as usize;

    let register_i_value = registers.get_i() as usize;

    for i in 0..=index {
        mmu.set_byte(register_i_value + i, registers.get_vx(i));
    }
}

fn instruction_fx65(mmu: &mut Mmu, registers: &mut Registers, op_code: u16) {
    let index = ((op_code & 0x0F00) >> 8) as u16;
    let i = registers.get_i();

    for idx in 0..=index {
        registers.set_vx(idx as usize, mmu.get_byte((i + idx) as usize));
    }
}
