
use crate::obs::OBS;
use crate::gates::{GateOptions,activate_gate};
// mod gates;
// use obs::OBS;
use std::{fs,collections::HashMap};

#[derive(Debug)]
struct GateInfo(GateOptions,Vec<String>,String);

#[derive(Debug)]
pub struct System{
    id: String,
    gates: Vec<(String, GateInfo)>,
}


impl System{


    pub fn activate(&self, input: &OBS) -> (){
        let i = &input.input;
        let z = vec![false;self.gates.len()];
        println!("{:?}",input);
        for (key,info) in (&self.gates).iter().rev(){
            let gate_input: Vec<bool> = (info.1).iter().map(|i_|i[i_[1..i_.len()].parse::<usize>().unwrap()-1]).collect();
            println!("{:?}",gate_input);
            let t = activate_gate(&info.0,&gate_input);
            println!("({},{:?})",key,info);
        }
        // outputs

    }

    pub fn new(path: &str) -> System{
        let content = fs::read_to_string(path).expect("Unable to read file");
        let content = content.replace(&['.',']','['], "");
        let mut content: Vec<&str> = content.lines().rev().collect();
        let id =format!("{}",content.pop().unwrap());
        let _ = content.pop().unwrap();
        let _ = content.pop().unwrap();
        let mut gates: Vec<(String, GateInfo)> = Vec::new();
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
            let gate_id = format!("{}",tokens.pop().unwrap());
            let output = format!("{}", tokens.pop().unwrap());
            let inputs: Vec<String> = tokens.iter().map(|&s|s.into()).filter(|x| x!="").collect();
            let gate_info = GateInfo(gate,inputs,output);
            gates.push((gate_id, gate_info));
        }
        System{id,gates}
    }  

}



