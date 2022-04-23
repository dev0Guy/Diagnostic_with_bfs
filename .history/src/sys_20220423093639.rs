
// mod obs;
use crate::obs::OBS;
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
        println!("{:?}", content);
        let id = content.pop().unwrap();
        let input = content.pop().unwrap();
        let output = content.pop().unwrap();
        println!("{:?}", id);
        println!("{:?}", output);
        println!("{:?}", input);

    }

}