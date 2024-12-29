pub trait Controller {
    fn is_key_down(&self, key_index: usize) -> bool;
}

pub trait Renderer {
    fn render(&mut self, screen: &Vec<u8>);
}

pub trait Synthetizer {
    fn play(&mut self);
    fn stop(&mut self);
}
