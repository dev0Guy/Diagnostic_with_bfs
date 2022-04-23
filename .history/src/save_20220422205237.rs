use std::fs;
mod Gates;
use Gates::GatesOptions as GatesOptions;
use std::io::{Error, ErrorKind};

fn decompose(v: Vec<&str>) -> [&str; 5] {
    v.try_into()
        .unwrap_or_else(|v: Vec<&str>| panic!("Expected a Vec of length {} but it was {}", 5, v.len()))
}

fn remove_first_and_last(input: &str) -> &str{
    return &input[1..input.len() - 1];
}

    fn line_convertor(line: Vec<&str>) -> Result<GatesOptions,Error>{
        let line_size: usize = line.len();
        if line.len() <= 3{
            return Err(Error::new(ErrorKind::AddrInUse,"Line Have To be More Then 4: (LogicType, GateID, GateOutput | GateInputs)"));
        }
        let operation: &str = line[0];
        let gate_id: &str = line[1];
        let gate_output: &str = line[2];
        let inputs: Vec<&str> = line.iter().enumerate().filter(|(i, _)| *i >= 3 ).map(|(_, e)| *e).collect();
        println!("{:?}",inputs);
        // return match operation {
        //     "and2" => Ok(Gates::GatesOptions::And2(inputs[0],inputs[1])),
        // };
        // println!("opt: {}, id: {}, out: {}, in:{:?}",operation,gate_id,gate_output,inputs);
        return Ok(Gates::GatesOptions::And2(false,false))
    }

fn main() -> std::io::Result<()> {
    let global_path: &str = "./data_for_exercises/circuits";
    
    let system_path : &str=  "Data_Systems";
    let observation_path : &str=  "Data_Observations";
    let file_name : &str=  "74181.sys";


    let system_path : String =  format!("{}/{}",global_path,system_path);
    let observation_path : String =  format!("{}/{}",global_path,observation_path);
    let file_path : String =  format!("{}/{}",system_path,file_name);


    println!("{}",system_path);
    println!("{}",observation_path);
    println!("{}",file_path);

    let data = fs::read_to_string(file_path).expect("Unable to read file");
    // remove line breaks from text
    let data = data.replace("\n", "");
    let data = data.split(".");
    // split by dots
    let data: Vec<&str> = data.collect();
    let [system_id,input_lst,output_lst,gate_lst, _ ] = decompose(data);
    let input_lst: Vec<&str>  = remove_first_and_last(input_lst).split(',').collect();
    let output_lst: Vec<&str> = remove_first_and_last(output_lst).split(',').collect();
    let gate_lst: Vec<&str> = remove_first_and_last(gate_lst).split(",[").collect();
    let mut operations: Vec<GatesOptions> = Vec::new();
    for line in gate_lst.iter() {
        let operation = line_convertor(line.split(',').collect()).unwrap();
        operations.push(operation);
    }
    println!("{:?}",operations);

    // let (system_id,input_lst,output_lst,gate_lst) = (data[0],data[1],data[2],data[3]);
    Ok(())
}


// fn main() {
//     let gate = GatesOptions::Xor3(true,false,false);
//     println!("{}", Gates::activate(gate))
// }