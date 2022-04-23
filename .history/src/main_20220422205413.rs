use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let sys_path = &args[0];
    let obs_path = &args[1];
}