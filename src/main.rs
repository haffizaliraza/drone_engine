mod motor;
mod pid;
mod drone;

use drone::Drone;
use std::thread;
use std::time::Duration;

fn main() {
    let mut drone = Drone::new();
    drone.take_off();

    for _ in 0..10 {
        drone.simulate_drift();
        drone.stabilize(0.1);
        thread::sleep(Duration::from_millis(500));
    }

    drone.move_forward();
    thread::sleep(Duration::from_secs(2));

    drone.turn_left();
    thread::sleep(Duration::from_secs(2));

    drone.land();
}
