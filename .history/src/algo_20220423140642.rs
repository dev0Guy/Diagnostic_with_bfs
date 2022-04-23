use std::collections::HashMap;
use crate::obs::OBS;
use crate::sys::System;

pub fn BFS(sys:System,obs_list:Vec<OBS>)->(){
    let mut options: HashMap<usize,Vec<usize>> = HashMap::new();
    let mut minimal_set: Vec<String> = Vec::new();
    for index in 0..sys.len(){
        options.insert(index, Vec::new());
    }
    println!("{:?}", options);
    // for obs in obs_list.iter() {
    //     let mut changes =  vec![false;sys.len()];
    //     println!("{:?}",  sys.activate(obs,changes));
    // }
}