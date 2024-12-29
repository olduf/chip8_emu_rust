use chip8_lib::interfaces::Synthetizer;
use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};

pub struct SdlSynthetizer {
    audio_device: AudioDevice<SquareWave>,
    playing: bool,
}

impl SdlSynthetizer {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let audio_subsystem = sdl_context.audio().unwrap();

        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1), // mono
            samples: None,     // default sample size
        };

        let result = audio_subsystem.open_playback(None, &desired_spec, |spec| {
            // initialize the audio callback
            SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25,
            }
        });

        if result.is_ok() {
            Self {
                audio_device: result.unwrap(),
                playing: false,
            }
        } else {
            panic!("Could not create audio device.");
        }
    }
}

impl Synthetizer for SdlSynthetizer {
    fn play(&mut self) {
        if !self.playing {
            self.playing = true;
            self.audio_device.resume();
        }
    }

    fn stop(&mut self) {
        if self.playing {
            self.playing = false;
            self.audio_device.pause();
        }
    }
}

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}
