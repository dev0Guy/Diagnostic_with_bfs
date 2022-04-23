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
        // get current gate combination
        let combination = deqeue.pop().unwrap();
        // asume not in minimal_set
        let mut contains_combination = false;
        println!("{}", combination);
        for val in minimal_set.iter(){
            println!("{}", val);
            if contains_combination{break;}
            let mut tmp = true;
            for token in combination.iter(){
                if !tmp{break;}
                tmp &= val.contains(&format!(",{}",token));
            }
            println!("{}",tmp);
            contains_combination |= tmp;
        }
        if contains_combination{continue;}
        let combination_as_string = combination.iter().map(|x|format!(",{}",x)).collect();
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