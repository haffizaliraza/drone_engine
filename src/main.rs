mod motor;
mod pid;
mod drone;
mod dynamics;

use drone::Drone;
use std::thread;
use std::time::Duration;

fn main() {
    let mut drone = Drone::new();
    drone.take_off();

    let dt = 0.1;
    let sleep_duration = Duration::from_millis((dt * 1000.0) as u64);

    simulate_hover_stabilization(&mut drone, dt, 10, sleep_duration);
    simulate_motion(&mut drone, dt, sleep_duration, |drone| drone.move_forward(), 20);
    simulate_motion(&mut drone, dt, sleep_duration, |drone| drone.turn_left(), 20);

    drone.land();
}

fn simulate_hover_stabilization(drone: &mut Drone, dt: f64, iterations: usize, sleep: Duration) {
    for _ in 0..iterations {
        drone.simulate_drift();
        drone.stabilize(dt);
        drone.update_physics(dt);
        thread::sleep(sleep);
    }
}

fn simulate_motion<F>(drone: &mut Drone, dt: f64, sleep: Duration, action: F, steps: usize)
where
    F: Fn(&mut Drone),
{
    action(drone);
    for _ in 0..steps {
        drone.update_physics(dt);
        thread::sleep(sleep);
    }
}
