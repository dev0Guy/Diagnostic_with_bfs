use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let sys_path = &args[1];
    let obs_path = &args[2];
    println!("{}", sys_path);
    // println!("{}", obs_path);
}