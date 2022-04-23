

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

fn activate(gate: &GatesOptions)-> GateOption{
    match gate{
        GatesOptions::Inverter => ,
        GatesOptions::Buffer,
        GatesOptions::And2,
        GatesOptions::And3,
        GatesOptions::And4,
        GatesOptions::And5,
        GatesOptions::NAnd2,
        GatesOptions::NAnd3,
        GatesOptions::NAnd4,
        GatesOptions::NAnd5,
        GatesOptions::Xor2,
        GatesOptions::Xor3,
        GatesOptions::Xor4,
        GatesOptions::Xor5,
        GatesOptions::Or2,
        GatesOptions::Or3,
        GatesOptions::Or4,
        GatesOptions::Or5,
        GatesOptions::NOr2,
        GatesOptions::NOr3,
        GatesOptions::NOr4,
        GatesOptions::NOr5,
    }
}