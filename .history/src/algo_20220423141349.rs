use std::collections::HashMap;
use crate::obs::OBS;
use crate::sys::System;

pub fn BFS(sys:System,obs_list:Vec<OBS>)->(){
    for obs in obs_list.iter() {
        let inputs = &obs.input;
        let output = &obs.output;input;

    }
    // for obs in obs_list.iter() {
    //     let mut changes =  vec![false;sys.len()];
    //     println!("{:?}",  sys.activate(obs,changes));
    // }
}