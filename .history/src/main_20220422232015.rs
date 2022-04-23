use std::{env};
mod obs;

fn main() {
    // get params from  CLI
    let args: Vec<String> = env::args().collect();
    if args.len() < 2{
        panic!("Not Enogh Params As Been Suply To The Program");
    }
    let sys_path: &String = &args[1];
    let obs_path:  &String = &args[2];
    // open obs file 
    let x = obs::OBS::list_from_file(obs_path);
    println!("{:#?}",x)
    // open sys file
}