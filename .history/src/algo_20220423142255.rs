use std::collections::HashMap;
use std::vec;
use crate::obs::OBS;
use crate::sys::System;

pub fn BFS(sys:System,obs:&OBS)->(){
    let mut deqeue: Vec<Vec<usize>> = vec![vec![0;1];sys.len()];
    // insert all gates
    for gate_idx in deqeue.iter() {

    }
    deqeue.sort();
    for i in 0..sys.len(){
        deqeue[i][0]= i;
    }
    println!("{:?}",deqeue);
    
    
}