use crate::obs::OBS;
use crate::gates::{GateOptions,activate_gate};
use create::sys::System;

pub fn BFS(sys:System,obs_list:Vec<OBS>)->(){
    for obs in obs_list.iter() {
        let mut changes =  vec![false;sys.len()];
        println!("{:?}",  sys.activate(obs,changes));
    }
}