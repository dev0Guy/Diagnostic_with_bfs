use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2{
        panic!("Not enogh params as been suply to the proogram");
    let sys_path = &args[1];
    let obs_path = &args[2];
    println!("{}", sys_path);
    println!("{}", obs_path);
}