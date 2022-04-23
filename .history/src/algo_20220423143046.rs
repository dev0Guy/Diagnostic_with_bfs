use std::collections::HashMap;
use std::vec;
use crate::obs::OBS;
use crate::sys::System;

pub fn BFS(sys:System,obs:&OBS)->(){
    let mut deqeue: Vec<Vec<usize>> = vec![vec![0;1];sys.len()];
    // insert all token gates
    deqeue.iter_mut().enumerate().for_each(|(idx,cell)| cell[0] = idx);
    while !deqeue.is_empty(){
        let changes = vec![false;sys.len()];
        let combination = deqeue.pop().unwrap();
        combination.iter().for_each(|val| changes[val] = true);
        sys.activate(obs,changes);
    }
    println!("{:?}",deqeue);
}