

#[derive(Debug)]
pub struct Gate{
    operator: GateOption,
    name: String,
    input: Vec<String>,
    output: String,
}

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
enum GatesOptions{
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

fn activate(gate: &GatesOptions,input: Vec<bool>)-> bool{
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
        GatesOptions::NOr2,
        GatesOptions::NOr3,
        GatesOptions::NOr4,
        GatesOptions::NOr5,
    }
}

fn activate(token: GatesOptions,input: Vec<bool>) -> bool{
    return match token{
        GatesOptions::Inverter(in1)=> !in1,
        GatesOptions::Buffer(in1)=> in1,
        GatesOptions::And2(in1,in2)=> in1 && in2,
        GatesOptions::And3(in1,in2,in3)=> in1 & in2 & in3,
        GatesOptions::And4(in1,in2,in3,in4) => in1 & in2 & in3 & in4,
        GatesOptions::And5(in1,in2,in3,in4,in5) => in1 & in2 & in3 & in4 & in5,
        GatesOptions::NAnd2(in1,in2) => !(in1 & in2),
        GatesOptions::NAnd3(in1,in2,in3)=> !(in1 & in2 & in3),
        GatesOptions::NAnd4(in1,in2,in3,in4)=> !(in1 & in2 & in3 & in4),
        GatesOptions::NAnd5(in1,in2,in3,in4,in5) => !(in1 & in2 & in3 & in4 & in5),
        GatesOptions::Xor2(in1,in2)=> in1 ^ in2,
        GatesOptions::Xor3(in1,in2,in3)=> in1 ^ in2 ^ in3,
        GatesOptions::Xor4(in1,in2,in3,in4)=> in1 ^ in2 ^ in3 ^ in4,
        GatesOptions::Xor5(in1,in2,in3,in4,in5)=> in1 ^ in2 ^ in3 ^ in4 ^ in5,
        GatesOptions::Or2(in1,in2) => in1 | in2,
        GatesOptions::Or3(in1,in2,in3)=> in1 | in2 | in3,
        GatesOptions::Or4(in1,in2,in3,in4)=> in1 | in2 | in3 | in4,
        GatesOptions::Or5(in1,in2,in3,in4,in5) => in1 | in2 | in3 | in4 | in5,
        GatesOptions::NOr2(in1,in2)=> !(in1 | in2),
        GatesOptions::NOr3(in1,in2,in3)=> !(in1 | in2 | in3),
        GatesOptions::NOr4(in1,in2,in3,in4)=> !(in1 | in2 | in3 | in4),
        GatesOptions::NOr5(in1,in2,in3,in4,in5)=> !(in1 | in2 | in3 | in4 | in5),
    }
}