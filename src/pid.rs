#[derive(Debug)]
pub struct PIDController {
    kp: f32,
    ki: f32,
    kd: f32,
    integral: f32,
    previous_error: f32,
}

impl PIDController {
    pub fn new(kp: f32, ki: f32, kd: f32) -> Self {
        Self {
            kp,
            ki,
            kd,
            integral: 0.0,
            previous_error: 0.0,
        }
    }

    pub fn update(&mut self, target: f32, current: f32, dt: f32) -> f32 {
        let error = target - current;
        self.integral += error * dt;
        let derivative = (error - self.previous_error) / dt;
        self.previous_error = error;

        self.kp * error + self.ki * self.integral + self.kd * derivative
    }
}
