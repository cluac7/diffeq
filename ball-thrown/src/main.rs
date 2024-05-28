use std::io::stdin;

// EQUATION:
// \ddot h(t) = -9.8
// h(t) = \frac{1}{2}(9.8)(t^2) + v_0 t + y_0

// Constants
const G: f64 = -9.8;

fn height(time: f64, accuracy: f64, starting_vel: f64) -> f64 {
    let mut height: f64 = 0.0;
    let mut vel: f64 = starting_vel;
    let timestep = 1.0/accuracy;

    let steps: u64 = (time / timestep) as u64;
    
    for _n in 0..steps {
        height += vel * timestep;
        vel += G * timestep;
    }
    
    if height >= 0.0 {height} else {0.0}
}

fn main() {
    let mut time = String::new();
    let mut accuracy = String::new();
    let mut starting_vel = String::new();

    println!("Enter the total time (s), then desired accuracy (s^-1), then the starting velocity (ms^-1): ");

    stdin()
        .read_line(&mut time)
        .ok()
        .expect("failed to read total time");
    stdin()
        .read_line(&mut accuracy)
        .ok()
        .expect("failed to read total time");
    stdin()
        .read_line(&mut starting_vel)
        .ok()
        .expect("failed to read total time");

    let final_height = height(
        time.trim().parse::<f64>().unwrap(),
        accuracy.trim().parse::<f64>().unwrap(),
        starting_vel.trim().parse::<f64>().unwrap(),
    );

    println!("The final height is {final_height}");
}
