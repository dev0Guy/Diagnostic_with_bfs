
use crate::obs::OBS;
use crate::gates::GateOptions;
// mod gates;
// use obs::OBS;
use std::{fs, collections::HashMap};

#[derive(Debug)]
struct GateInfo(Vec<String>,String);

#[derive(Debug)]
pub struct System{
    id: String,
    id_to_gate_option: HashMap<String,GateInfo>,
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
        let mut id_to_gate_option: HashMap<&str, GateInfo> = HashMap::new();
        for line in content.iter() {
            let mut tokens: Vec<&str> = line.split(',').rev().collect();
            let gate: GateOptions = match tokens.pop().unwrap(){
                "inverter" => GateOptions::Inverter,
                "buffer" => GateOptions::Buffer,
                "and2" => GateOptions::And2,
                "and3" => GateOptions::And3,
                "and4" => GateOptions::And4,
                "and5" => GateOptions::And5,
                "nand2" => GateOptions::NAnd2,
                "nand3" => GateOptions::NAnd3,
                "nand4" => GateOptions::NAnd4,
                "nand5" => GateOptions::NAnd5,
                "xor2" => GateOptions::Xor2,
                "xor3" => GateOptions::Xor3,
                "xor4" => GateOptions::Xor4,
                "xor5" => GateOptions::Xor5,
                "or2" => GateOptions::Or2,
                "or3" => GateOptions::Or3,
                "or4" => GateOptions::Or4,
                "or5" => GateOptions::Or5,
                "nor2" => GateOptions::NOr2,
                "nor3" => GateOptions::NOr3,
                "nor4" => GateOptions::NOr4,
                "nor5" => GateOptions::NOr5,
                _ => panic!("FUCK"),
            };
            println!("{:?}",tokens);
            let id = tokens.pop().unwrap();
            let output = tokens.pop().unwrap();

            let gate_info = GateInfo(vec!["".to_owned();4],format!("{}", output),);
            id_to_gate_option.insert(id, gate_info);
        }
        println!("{:?}",id_to_gate_option);
    }   

}



