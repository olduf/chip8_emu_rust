use chip8_lib::emulator::Emulator;

use clap::Parser;

use crate::implementations::controller::SdlController;
use crate::implementations::renderer::SdlRenderer;
use crate::implementations::synthetizer::SdlSynthetizer;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod implementations;

const DEFAULT_DURATION: Duration = Duration::new(0, 1_000_000_000u32 / 60);

// maybe an enum for this? -> 420Hz: 7, 480Hz: 8, 540Hz: 9, 600Hz: 10, 660Hz: 11, 720Hz: 12, 780Hz: 13
// in steps of 60Hz to facilitate frame speed
const INSTRUCTIONS_PER_FRAME: u32 = 10;

fn main() -> Result<(), String> {
    // cli arguments
    let args = Args::parse();

    // SDL initialization
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let mut event_pump = sdl_context.event_pump()?;

    let window = video_subsystem
        .window("rust-chip8-sdl2", 1024, 512)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");

    // initialize implementations et create the emulator
    let mut controller = SdlController::new();
    let mut renderer = SdlRenderer::new(canvas);
    let mut synthetizer = SdlSynthetizer::new();
    let mut chip8_emulator = Emulator::new(&args.path);

    // main loop
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }

        // set input state
        controller.set_keys_state(event_pump.keyboard_state());

        // emulator execution
        let time_elapsed = chip8_emulator.step_frame(
            INSTRUCTIONS_PER_FRAME,
            &controller,
            &mut renderer,
            &mut synthetizer,
        );

        // sleep until 1/16th of a second has passed
        if time_elapsed.is_ok() {
            ::std::thread::sleep(DEFAULT_DURATION.abs_diff(time_elapsed.unwrap()));
        } else {
            ::std::thread::sleep(DEFAULT_DURATION);
        }
    }

    Ok(())
}

// cli arguments struct
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    path: String,
}
