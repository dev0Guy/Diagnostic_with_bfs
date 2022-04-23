
use crate::obs::OBS;
use crate::gates::GateOptions;
// mod gates;
// use obs::OBS;
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
            let gate: GateOptions = match tokens[0]{
                "Inverter" => GateOptions::Inverter,
                "Buffer" => GateOptions::Buffer,
                "And2" => GateOptions::And2,
                "And3" => GateOptions::And3,
                "And4" => GateOptions::And4,
                "And5" => GateOptions::And5,
                "NAnd2" => GateOptions::NAnd2,
                "NAnd3" => GateOptions::NAnd3,
                "NAnd4" => GateOptions::NAnd4,
                "NAnd5" => GateOptions::NAnd5,
                "Xor2" => GateOptions::Xor2,
                "Xor3" => GateOptions::Xor3,
                "Xor4" => GateOptions::Xor4,
                "Xor5" => GateOptions::Xor5,
                "Or2" => GateOptions::Or2,
                "Or3" => GateOptions::Or3,
                "Or4" => GateOptions::Or4,
                "Or5" => GateOptions::Or5,
                "NOr2" => GateOptions::NOr2,
                "NOr3" => GateOptions::NOr3,
                "NOr4" => GateOptions::NOr4,
                "NOr5" => GateOptions::NOr5,
                _ => panic!("FUCK")
            };

        }

    }

}



