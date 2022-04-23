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
        let combination = deqeue.pop().unwrap();
        let mut contains_combination = false;
        for i in 0..minimal_set.iter(){
            if contains_combination{
                break;
            }
            contains_combination |= true;
        }
        let combination_as_string = combination.iter().map(|x|format!(",{}",x)).collect();
        if minimal_set.contains(&combination_as_string){
            continue;
        }
        let mut changes = vec![false;sys.len()];
        combination.iter().for_each(|val| changes[*val] = true);
        let out = sys.activate(obs,changes);
        let matching = out.iter().zip(&obs.output).filter(|&(a, b)| a != b).count();
        if matching == 0{
            minimal_set.push(combination_as_string);
        }else{
            for i in 0..sys.len(){
                let as_str = &format!(",{}",i);
                if !minimal_set.contains(as_str) && !combination_as_string.contains(as_str){
                    let mut x = combination.clone();
                    x.push(i);
                    deqeue.push(x);
                }
            }
        }
    }
    println!("{:?}",minimal_set);
}