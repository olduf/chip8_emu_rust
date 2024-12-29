use crate::cpu::Cpu;
use crate::interfaces::{Controller, Renderer, Synthetizer};
use crate::mmu::Mmu;
use crate::registers::Registers;
use std::fs;
use std::time::{Duration, SystemTime, SystemTimeError};

pub struct Emulator {
    cpu: Cpu,
    mmu: Mmu,
    registers: Registers,
}

impl Emulator {
    pub fn new(rom_path: &String) -> Self {
        let result = load_file_to_vector(rom_path);

        if result.is_ok() {
            let rom = result.unwrap();

            Self {
                cpu: Cpu {},
                mmu: Mmu::new(&rom),
                registers: Registers::new(),
            }
        } else {
            panic!("Could not load file: {}", rom_path);
        }
    }

    pub fn step_frame(
        &mut self,
        instructions_per_frame: u32,
        controller: &impl Controller,
        renderer: &mut impl Renderer,
        synthetizer: &mut impl Synthetizer,
    ) -> Result<Duration, SystemTimeError> {
        let start_time = SystemTime::now();

        // handle input
        for i in 0..16 {
            self.mmu.set_key_down(i, controller.is_key_down(i));
        }

        // run frame
        for _ in 0..instructions_per_frame {
            self.cpu.tick(&mut self.mmu, &mut self.registers);
        }

        // render screen
        renderer.render(self.mmu.get_screen());

        // handle sound
        if self.mmu.get_sound_timer() > 0 {
            synthetizer.play();
        } else {
            synthetizer.stop();
        }

        // decrement timers
        self.mmu.decrement_delay_timer();
        self.mmu.decrement_sound_timer();

        SystemTime::now().duration_since(start_time)
    }
}

fn load_file_to_vector(path: &String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let data: Vec<u8> = fs::read(path)?;

    Ok(data)
}
