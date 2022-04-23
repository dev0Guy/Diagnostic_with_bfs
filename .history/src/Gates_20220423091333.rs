

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

    fn activate(&self,input: &Vec<bool>){
        activate(self.operator)
    }

    fn change_inputs(& mut self){
        for idx in 0..self.operator.iter().len() {
            println!("{}",idx)
        }
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
    Xor5(bool,bool,bool,bool,bool),
    Or2(bool,bool),
    Or3(bool,bool,bool),
    Or4(bool,bool,bool,bool),
    Or5(bool,bool,bool,bool,bool),
    NOr2(bool,bool),
    NOr3(bool,bool,bool),
    NOr4(bool,bool,bool,bool),
    NOr5(bool,bool,bool,bool,bool),
}

fn change_input(gate: &GatesOptions)->GateOption{

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