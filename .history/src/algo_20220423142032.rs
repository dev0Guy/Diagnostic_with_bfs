use std::collections::HashMap;
use std::vec;
use crate::obs::OBS;
use crate::sys::System;

pub fn BFS(sys:System,obs:OBS)->(){
    let mut deqeue: Vec<Vec<usize>> = Vec::new();
    for i in 0..sys.len(){
        deqeue.push(vec![i;1])
    }
    
    
}