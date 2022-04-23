use std::collections::HashMap;
use std::vec;
use crate::obs::OBS;
use crate::sys::System;

pub fn BFS(sys:System,obs:&OBS)->(){
    let mut deqeue: Vec<Vec<usize>> = vec![vec![0;1];sys.len()];
    deqeue.iter_mut().enumerate().map(|(idx,cell)| cell[0] = idx);
    for (idx,cell) in deqeue.iter_mut().enumerate() {
        cell[0] = idx
    }
    println!("{:?}",deqeue);
}