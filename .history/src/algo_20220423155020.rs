use std::collections::HashMap;
use std::vec;
use crate::obs::OBS;
use crate::sys::System;

pub fn BFS(sys:System,obs:&OBS)->(){
    let mut deqeue: Vec<Vec<usize>> = vec![vec![0;1];sys.len()];
    let mut minimal_set: Vec<String> = Vec::new();
    // insert all token gates
    deqeue.iter_mut().enumerate().for_each(|(idx,cell)| cell[0] = idx);
    while !deqeue.is_empty(){
        let mut changes = vec![false;sys.len()];
        let combination = deqeue.pop().unwrap();
        combination.iter().for_each(|val| changes[*val] = true);
        let out = sys.activate(obs,changes);
        let matching = out.iter().zip(&obs.output).filter(|&(a, b)| a != b).count();
        let combination_as_string = combination.iter().map(|x|format!(",{}",x)).collect();
        if matching == 0{
            minimal_set.push(combination_as_string);
        }else{
            for i in 0..sys.len(){
                if !minimal_set.contains(&format!("{}",i)){
                    let mut x = combination.clone();
                    x.push(i);
                    deqeue.push(x);
                }
            }
        }

    }
    println!("{:?}",minimal_set);
}