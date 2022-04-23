mod obs;
mod sys;
mod gates;
use obs::{OBS};
use sys::{System};

use std::{env};

fn main() {
    // get params from  CLI
    let args: Vec<String> = env::args().collect();
    if args.len() < 2{
        panic!("Not Enogh Params As Been Suply To The Program");
    }
    let sys_path: &String = &args[1];
    let obs_path:  &String = &args[2];
    // open obs file 
    let obs_list = OBS::list_from_file(obs_path);
    // get system represntation
    let sys = System::new(sys_path);
    println!("{:?}",sys);
    // run and find obs 
    for obs in obs_list.iter() {
        println!("{:?}", obs);
    }

}