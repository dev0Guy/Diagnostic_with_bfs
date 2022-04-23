
// mod obs;
// use crate::obs::OBS;
use crate::gates::GatesOptions;

// mod gates;
// use obs::OBS;
use std::collections::HashMap;
use std::{fs};

type Callback = fn(&System,String) -> bool;

#[derive(Debug)]
pub struct System{
    id: String,
    // mapper: HashMap<String,Callback>,
    // gates: Vec<GatesOptions>,
}


impl System{


    fn activate(&self, input: &OBS){
    }

    pub fn new(path: &str) -> (){
        let content = fs::read_to_string(path).expect("Unable to read file");
        let content = content.replace(&['.',']','['], "");
        let mut content: Vec<&str> = content.lines().rev().collect();
        let _ = content.pop().unwrap();
        let _ = content.pop().unwrap();
        let _ = content.pop().unwrap();
        let z: Vec<&str> = Vec::new();
        let z: Vec<&str> = Vec::new();

        for line in content.iter() {
            let tokens: Vec<&str> = line.split(',').collect();
            // let gate: GatesOptions = match token[0]{

            // }
            println!("{:?}", tokens);

        }

    }

}