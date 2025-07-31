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

    let dt = 0.1; // 100 ms physics update timestep

    // Hover stabilization loop
    for _ in 0..10 {
        drone.simulate_drift();
        drone.stabilize(dt);

        // NEW: update physics for each iteration
        drone.update_physics(dt);

        thread::sleep(Duration::from_millis((dt * 1000.0) as u64));
    }

    // Move forward while updating physics
    drone.move_forward();
    for _ in 0..20 {
        drone.update_physics(dt);
        thread::sleep(Duration::from_millis((dt * 1000.0) as u64));
    }

    // Turn left with physics updates
    drone.turn_left();
    for _ in 0..20 {
        drone.update_physics(dt);
        thread::sleep(Duration::from_millis((dt * 1000.0) as u64));
    }

    drone.land();
}
