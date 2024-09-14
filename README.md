# Rust Gateway for MountainCarAI

This project creates a simple Rust gateway to interact with the RxInfer example Julia project [MountainCarAI](https://github.com/sustia-llc/MountainCarAI), which creates and runs Active Inference agents. This project binds to the Mountain Car simulation (Julia function run_simulation) from Rust via [jlrs](https://github.com/Taaitaaiger/jlrs).

## Prerequisites

1. Rust: Make sure you have Rust installed on your system. You can install it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

2. Julia: Ensure Julia 1.10.5 is installed on your system. You will need to download the full archive from [https://julialang.org/downloads/](https://julialang.org/downloads/) and extract and deploy per [jlrs](https://github.com/Taaitaaiger/jlrs) requirements:
   - linux:  extract archive to $HOME and add these commands to .bashrc:
   ```
   export JULIA_DIR="$HOME/julia-1.10.5"
   export PATH="$JULIA_DIR/bin:$PATH"
   export LD_LIBRARY_PATH=$JULIA_DIR/lib${LD_LIBRARY_PATH:+:${LD_LIBRARY_PATH}}
   ```

3. Clone this repository:
   ```
   git clone https://github.com/your-username/jlrs-mountain-car.git
   ```
4. MountainCarAI: Clone the MountainCarAI project at the same directory level as this project:

   ```
   git clone https://github.com/sustia-llc/MountainCarAI.git
   ```

   Your directory structure should look like this:

   ```
   parent_directory/
   ├── jlrs-mountain-car/
   └── MountainCarAI/
   ```

## Run the simulation: 
   ```
   cd jlrs-mountain-car
   cargo run
   ```