use crate::motor::Motor;
use crate::pid::PIDController;
use crate::dynamics::Dynamics;


pub struct Drone {
    pub dynamics: Dynamics,
    pub motors: [Motor; 4],
    pid_pitch: PIDController,
    pid_roll: PIDController,
    pid_yaw: PIDController,
    pid_altitude: PIDController,
    pub pitch: f32,
    pub roll: f32,
    pub yaw: f32,
    pub altitude: f32,
}

impl Drone {
    pub fn new() -> Self {
        Self {
            dynamics: Dynamics::new(),
            motors: [Motor::new(), Motor::new(), Motor::new(), Motor::new()],
            pid_pitch: PIDController::new(1.0, 0.0, 0.2),
            pid_roll: PIDController::new(1.0, 0.0, 0.2),
            pid_yaw: PIDController::new(1.0, 0.0, 0.2),
            pid_altitude: PIDController::new(1.2, 0.0, 0.3),
            pitch: 0.0,
            roll: 0.0,
            yaw: 0.0,
            altitude: 0.0,
        }
    }
    pub fn update_physics(&mut self, dt: f32) {
        let thrusts: [f32; 4] = self.motors.iter().map(|m| m.speed * 0.1).collect::<Vec<_>>().try_into().unwrap(); // convert % to force
        self.dynamics.update(thrusts, dt);
        println!(
            "Pos: ({:.2}, {:.2}, {:.2}) | Orient: Pitch {:.2}, Roll {:.2}, Yaw {:.2}",
            self.dynamics.x, self.dynamics.y, self.dynamics.z,
            self.dynamics.pitch, self.dynamics.roll, self.dynamics.yaw
        );
    }

    fn set_all_motors(&mut self, speed: f32) {
        for motor in &mut self.motors {
            motor.set_speed(speed);
        }
    }

    pub fn take_off(&mut self) {
        println!("Taking off...");
        self.set_all_motors(50.0);
        self.altitude = 1.0;
    }

    pub fn land(&mut self) {
        println!("Landing...");
        self.set_all_motors(0.0);
        self.altitude = 0.0;
    }

    pub fn stabilize(&mut self, dt: f32) {
        let pitch_correction = self.pid_pitch.update(0.0, self.pitch, dt);
        let roll_correction = self.pid_roll.update(0.0, self.roll, dt);
        let yaw_correction = self.pid_yaw.update(0.0, self.yaw, dt);
        let altitude_correction = self.pid_altitude.update(5.0, self.altitude, dt); // target altitude = 5m

        let base_speed = 50.0 + altitude_correction;

        self.motors[0].set_speed(base_speed - pitch_correction - roll_correction + yaw_correction);
        self.motors[1].set_speed(base_speed - pitch_correction + roll_correction - yaw_correction);
        self.motors[2].set_speed(base_speed + pitch_correction - roll_correction - yaw_correction);
        self.motors[3].set_speed(base_speed + pitch_correction + roll_correction + yaw_correction);

        println!(
            "Stabilizing | Pitch: {:.2}, Roll: {:.2}, Yaw: {:.2}, Altitude: {:.2}",
            self.pitch, self.roll, self.yaw, self.altitude
        );
    }

    pub fn simulate_drift(&mut self) {
        self.pitch += 0.3;
        self.roll -= 0.2;
        self.yaw += 0.1;
        self.altitude -= 0.2;
    }

    pub fn move_forward(&mut self) {
        println!("Moving forward...");
        self.motors[2].set_speed(self.motors[2].speed + 5.0); // back motors push forward
        self.motors[3].set_speed(self.motors[3].speed + 5.0);
    }

    pub fn move_backward(&mut self) {
        println!("Moving backward...");
        self.motors[0].set_speed(self.motors[0].speed + 5.0);
        self.motors[1].set_speed(self.motors[1].speed + 5.0);
    }

    pub fn turn_left(&mut self) {
        println!("Turning left...");
        self.motors[0].set_speed(self.motors[0].speed - 5.0);
        self.motors[2].set_speed(self.motors[2].speed - 5.0);
    }

    pub fn turn_right(&mut self) {
        println!("Turning right...");
        self.motors[1].set_speed(self.motors[1].speed - 5.0);
        self.motors[3].set_speed(self.motors[3].speed - 5.0);
    }

    pub fn hover(&mut self) {
        println!("Hovering...");
        self.set_all_motors(50.0);
    }
}
