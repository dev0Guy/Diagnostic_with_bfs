use std::collections::HashSet;
use std::vec;
use crate::obs::OBS;
use crate::sys::System;
use std::time::{Instant,Duration};


pub fn bfs(sys:&System,obs:&OBS,now:Instant,stop_time: usize)->  (usize,Vec<HashSet<usize>>){
    let mut deqeue: Vec<HashSet<usize>> = vec![HashSet::new();sys.len()];
    let mut minimal_set: Vec<HashSet<usize>> = Vec::new();
    let mut  elapsed =  now.elapsed().as_micros() as usize;
    // insert all token gates
    for (idx,set) in deqeue.iter_mut().enumerate(){
        set.insert(idx);
    }
    while !deqeue.is_empty() && elapsed < stop_time{
        // get current gate combination
        let combination = deqeue.remove(0);
        // asume not in minimal_set
        let mut contains_combination = false;
        for set in &minimal_set{
            if contains_combination{break;}
            contains_combination |= set.difference(&combination).count() == 0;
        }
        if contains_combination{continue;}
        let mut changes = vec![false;sys.len()];
        combination.iter().for_each(|val| changes[*val] = true);
        let out = sys.activate(obs,changes);
        let matching = out.iter().zip(&obs.output).filter(|&(a, b)| a != b).count();
        if matching == 0{
            minimal_set.push(combination);
        }else{
            for idx in 0..sys.len()-1{
                if !combination.contains(&idx){
                    let mut x = combination.clone();
                    x.insert(idx);
                    deqeue.push(x);
                }
            }
        }
        elapsed = now.elapsed().as_micros() as usize;
    }
    (elapsed,minimal_set)
}