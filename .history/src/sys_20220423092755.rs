
mod obs;
mod gates;
use obs::OBS;
use gates::GatesOptions;
use std::collections::HashMap;
use std::{fs};

type Callback = fn(&System,String) -> bool;

#[derive(Debug)]
pub struct System{
    id: String,
    mapper: HashMap<String,Callback>,
    gates: Vec<GatesOptions>,
}


impl System{


    fn activate(&self, input: &OBS){
    }

    pub fn new(path: &str) -> (){
        let content = fs::read_to_string(path).expect("Unable to read file");

    }

}