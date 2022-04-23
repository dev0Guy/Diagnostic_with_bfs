#[derive(Debug)]
pub enum GatesOptions{
    Inverter(bool),
    Buffer(bool),
    And2(bool,bool),
    And3(bool,bool,bool),
    And4(bool,bool,bool,bool),
    And5(bool,bool,bool,bool,bool),
    NAnd2(bool,bool),
    NAnd3(bool,bool,bool),
    NAnd4(bool,bool,bool,bool),
    NAnd5(bool,bool,bool,bool,bool),
    Xor2(bool,bool),
    Xor3(bool,bool,bool),
    Xor4(bool,bool,bool,bool),
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


pub fn convert_from_str() -> GatesOptions{
    return GatesOptions::And2(false,false)
}

pub fn activate(token: GatesOptions) -> bool{
    return match token{
        GatesOptions::Inverter(in1)=> !in1,
        GatesOptions::Buffer(in1)=> !in1,
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