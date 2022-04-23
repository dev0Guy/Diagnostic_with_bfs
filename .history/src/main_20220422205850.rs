use std::env;

fn main() {
    // get params from  CLI
    let args: Vec<String> = env::args().collect();
    if args.len() < 2{
        panic!("Not Enogh Params As Been Suply To The Program");
    }
    let sys_path = &args[1];
    let obs_path = &args[2];
    println!("{}", sys_path);
    // open obs file 

    // open sys file
}