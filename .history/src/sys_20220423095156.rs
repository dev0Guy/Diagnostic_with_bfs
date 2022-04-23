
use crate::obs::OBS;

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
            // let gate: GatesOptions = match token[0]{

            // }
            println!("{:?}", tokens);

        }

    }

}




#[derive(Debug)]
pub struct Gate{
    operator: GateOption,
    name: String,
    input: Vec<String>,
    output: String,
}







mod gates{
    impl Gate{

        pub fn new(operator:GateOption,name: String, input: Vec<String>,output: String) -> Gate{
            Gate{
                operator,name,input,output
            }
        }
    
        fn activate(&self,input: &Vec<bool>) -> bool{
            activate(self.operator)
        }
    }
    
    
    #[derive(Debug)]
    pub enum GatesOption{
        Inverter,
        Buffer,
        And2,
        And3,
        And4,
        And5,
        NAnd2,
        NAnd3,
        NAnd4,
        NAnd5,
        Xor2,
        Xor3,
        Xor4,
        Xor5,
        Or2,
        Or3,
        Or4,
        Or5,
        NOr2,
        NOr3,
        NOr4,
        NOr5,
    }
    
    fn activate(gate: &GatesOption,input: Vec<bool>)-> bool{
        match gate{
            GatesOptions::Inverter => !input[0],
            GatesOptions::Buffer => input[0],
            GatesOptions::And2 => input[0] & input[1],
            GatesOptions::And3 => input[0] & input[1] & input[2],
            GatesOptions::And4 => input[0] & input[1] & input[2] & input[3],
            GatesOptions::And5 => input[0] & input[1] & input[2] & input[3] & input[4],
            GatesOptions::NAnd2 => !(input[0] & input[1]),
            GatesOptions::NAnd3 => !(input[0] & input[1] & input[2]),
            GatesOptions::NAnd4 => !(input[0] & input[1] & input[2] & input[3]),
            GatesOptions::NAnd5 => !(input[0] & input[1] & input[2] & input[3] & input[4]),
            GatesOptions::Xor2 => input[0] ^ input[1],
            GatesOptions::Xor3 => input[0] ^ input[1] ^ input[2],
            GatesOptions::Xor4 => input[0] ^ input[1] ^ input[2] ^ input[3],
            GatesOptions::Xor5 => input[0] ^ input[1] ^ input[2] ^ input[3] ^ input[4],
            GatesOptions::Or2 => input[0] | input[1],
            GatesOptions::Or3 => input[0] | input[1] | input[2],
            GatesOptions::Or4 => input[0] | input[1] | input[2] | input[3],
            GatesOptions::Or5 => input[0] | input[1] | input[2] | input[3] | input[4],
            GatesOptions::NOr2 => !(input[0] | input[1]),
            GatesOptions::NOr3 => !(input[0] | input[1] | input[2]),
            GatesOptions::NOr4 => !(input[0] | input[1] | input[2] | input[3]),
            GatesOptions::NOr5 => !(input[0] | input[1] | input[2] | input[3] | input[4]),
        }
    }
}
