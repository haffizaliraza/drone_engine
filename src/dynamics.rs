#[derive(Debug)]
pub struct Dynamics {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub pitch: f32,
    pub roll: f32,
    pub yaw: f32,
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
    pub pitch_rate: f32,
    pub roll_rate: f32,
    pub yaw_rate: f32,
    mass: f32,
    gravity: f32,
}

impl Dynamics {
    pub fn new() -> Self {
        Self {
            x: 0.0, y: 0.0, z: 0.0,
            pitch: 0.0, roll: 0.0, yaw: 0.0,
            vx: 0.0, vy: 0.0, vz: 0.0,
            pitch_rate: 0.0, roll_rate: 0.0, yaw_rate: 0.0,
            mass: 1.5,           // kg
            gravity: 9.81,       // m/s²
        }
    }

    pub fn update(&mut self, motor_thrusts: [f32; 4], dt: f32) {
        let total_thrust: f32 = motor_thrusts.iter().sum();

        // Net force (upward thrust minus gravity)
        let fz = total_thrust - self.mass * self.gravity;

        // Update vertical velocity and position
        self.vz += (fz / self.mass) * dt;
        self.z += self.vz * dt;

        // Torques for pitch, roll, yaw
        let pitch_torque = (motor_thrusts[2] + motor_thrusts[3]) - (motor_thrusts[0] + motor_thrusts[1]);
        let roll_torque = (motor_thrusts[1] + motor_thrusts[3]) - (motor_thrusts[0] + motor_thrusts[2]);
        let yaw_torque = (motor_thrusts[0] + motor_thrusts[3]) - (motor_thrusts[1] + motor_thrusts[2]);

        // Simplified rotational inertia
        let ixx = 0.02;
        let iyy = 0.02;
        let izz = 0.04;

        self.pitch_rate += (pitch_torque / ixx) * dt;
        self.roll_rate += (roll_torque / iyy) * dt;
        self.yaw_rate += (yaw_torque / izz) * dt;

        self.pitch += self.pitch_rate * dt;
        self.roll += self.roll_rate * dt;
        self.yaw += self.yaw_rate * dt;

        // Simple planar motion
        let fx = self.pitch.sin() * total_thrust;
        let fy = self.roll.sin() * total_thrust;

        self.vx += (fx / self.mass) * dt;
        self.vy += (fy / self.mass) * dt;
        self.x += self.vx * dt;
        self.y += self.vy * dt;
    }
}
