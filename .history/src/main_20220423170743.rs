mod obs;
mod sys;
mod gates;
mod algo;
use std::time::Instant;
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
    // run and find obs 
    let mut obs_minimal: Vec<(usize,usize,usize,usize)> = Vec::new();
    for (idx,obs) in obs_list.iter().enumerate() {
        let now = Instant::now();
        let info =algo::BFS(&sys,&obs);
        let diagnostic = info.len();
        let minimal_set = info.iter().min().unwrap().len()/2;
        let elapsed: usize = now.elapsed().as_micros() as usize;
        let t = (idx,diagnostic,minimal_set,elapsed);
        println!("{}",t);
        obs_minimal.push(t);
    }
    println!("{:?}", obs_minimal.len());
}


// cargo run --release "data_for_exercises/circuits/Data_Systems/c17.sys" "data_for_exercises/circuits/Data_Observations/c17_iscas85.obs"
// cargo run --release "data_for_exercises/circuits/Data_Systems/74182.sys" "data_for_exercises/circuits/Data_Observations/74182_iscas85.obs"
