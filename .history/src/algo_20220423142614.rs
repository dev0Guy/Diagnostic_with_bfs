use std::collections::HashMap;
use std::vec;
use crate::obs::OBS;
use crate::sys::System;

pub fn BFS(sys:System,obs:&OBS)->(){
    let mut deqeue: Vec<Vec<usize>> = vec![vec![0;1];sys.len()];
    // insert all token gates
    deqeue.iter_mut().enumerate().for_each(|(idx,cell)| cell[0] = idx);
    while  deqeue.is_empty(){
        
    }
    println!("{:?}",deqeue);
}