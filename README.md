🛩️ Drone Engine in Rust
A basic Rust project that simulates a quadcopter drone engine with:

✅ Modular design (motor, pid, drone)

✅ PID controllers for pitch, roll, yaw, and altitude stabilization

✅ Basic movement controls (forward, backward, turning)

✅ Simulated environmental drift to test stabilization

📂 Project Structure
bash
Copy

drone_engine/
│── Cargo.toml
└── src/
    ├── main.rs       # Entry point
    ├── motor.rs      # Motor control logic
    ├── pid.rs        # PID controller implementation
    ├── drone.rs      # Drone logic and stabilization
🚀 Getting Started
1️⃣ Install Rust
Make sure you have Rust installed:

bash
Copy

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
2️⃣ Clone and Build
bash
Copy

git clone https://github.com/yourusername/drone_engine.git
cd drone_engine
cargo build
3️⃣ Run
bash
Copy

cargo run
⚙️ Features
Motor control: Adjust speeds for each motor individually

Stabilization: PID controllers keep drone level and maintain altitude

Movement: Forward, backward, turning left/right, hovering

Simulation: Adds artificial drift to test PID corrections


