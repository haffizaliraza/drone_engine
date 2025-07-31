#[derive(Debug)]
pub struct Motor {
    pub speed: f32, // percentage (0–100)
}

impl Motor {
    pub fn new() -> Self {
        Self { speed: 0.0 }
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed.clamp(0.0, 100.0);
        println!("Motor speed set to {:.1}%", self.speed);
    }
}
