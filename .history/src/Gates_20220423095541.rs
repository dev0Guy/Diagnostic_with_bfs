
#[derive(Debug)]
pub struct Gate{
    operator: GateOptions,
    name: String,
    input: Vec<String>,
    output: String,
}

impl Gate{

    pub fn new(operator:GateOptions,name: String, input: Vec<String>,output: String) -> Gate{
        Gate{
            operator,name,input,output
        }
    }

    fn activate(&self,input: &Vec<bool>) -> bool{
        activate(&self.operator,input)
    }
}


#[derive(Debug)]
enum GateOptions{
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

fn activate(gate: &GateOptions,input: &Vec<bool>)-> bool{
    match gate{
        GateOptions::Inverter => !input[0],
        GateOptions::Buffer => input[0],
        GateOptions::And2 => input[0] & input[1],
        GateOptions::And3 => input[0] & input[1] & input[2],
        GateOptions::And4 => input[0] & input[1] & input[2] & input[3],
        GateOptions::And5 => input[0] & input[1] & input[2] & input[3] & input[4],
        GateOptions::NAnd2 => !(input[0] & input[1]),
        GatesOptions::NAnd3 => !(input[0] & input[1] & input[2]),
        GatesOptions::NAnd4 => !(input[0] & input[1] & input[2] & input[3]),
        GatesOptions::NAnd5 => !(input[0] & input[1] & input[2] & input[3] & input[4]),
        GatesOptions::Xor2 => input[0] ^ input[1],
        GatesOptions::Xor3 => input[0] ^ input[1] ^ input[2],
        GatesOptions::Xor4 => input[0] ^ input[1] ^ input[2] ^ input[3],
        GateOptions::Xor5 => input[0] ^ input[1] ^ input[2] ^ input[3] ^ input[4],
        GateOptions::Or2 => input[0] | input[1],
        GateOptions::Or3 => input[0] | input[1] | input[2],
        GateOptions::Or4 => input[0] | input[1] | input[2] | input[3],
        GateOptions::Or5 => input[0] | input[1] | input[2] | input[3] | input[4],
        GateOptions::NOr2 => !(input[0] | input[1]),
        GatesOptions::NOr3 => !(input[0] | input[1] | input[2]),
        GatesOptions::NOr4 => !(input[0] | input[1] | input[2] | input[3]),
        GatesOptions::NOr5 => !(input[0] | input[1] | input[2] | input[3] | input[4]),
    }
}
