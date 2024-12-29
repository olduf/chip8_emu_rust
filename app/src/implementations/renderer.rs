use chip8_lib::interfaces::Renderer;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::Canvas;
use sdl2::surface::Surface;
use sdl2::video::{Window, WindowContext};

pub struct SdlRenderer {
    canvas: Canvas<Window>,
    pixel_format: sdl2::pixels::PixelFormatEnum,
    pitch: u32,
    texture_creator: sdl2::render::TextureCreator<WindowContext>,
}

impl SdlRenderer {
    pub fn new(input_canvas: Canvas<Window>) -> Self {
        let new_texture_creator = input_canvas.texture_creator();

        Self {
            canvas: input_canvas,
            pixel_format: PixelFormatEnum::RGB24,
            pitch: PixelFormatEnum::RGB24.byte_size_of_pixels(64) as u32,
            texture_creator: new_texture_creator,
        }
    }
}

impl Renderer for SdlRenderer {
    fn render(&mut self, screen: &Vec<u8>) {
        let mut buffer: Vec<u8> = vec![0; 32 * 64 * 3];

        for i in 0..32 * 64 {
            if screen[i] != 0 {
                buffer[i * 3] = 0xFF;
                buffer[i * 3 + 1] = 0xFF;
                buffer[i * 3 + 2] = 0xFF;
            };
        }

        self.canvas.clear();

        let surface =
            Surface::from_data(&mut buffer, 64, 32, self.pitch, self.pixel_format).unwrap();
        let texture = surface.as_texture(&self.texture_creator).unwrap();

        self.canvas.copy(&texture, None, None).unwrap();
        self.canvas.present();
    }
}
