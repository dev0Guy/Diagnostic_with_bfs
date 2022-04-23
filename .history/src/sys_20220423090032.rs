
mod obs;
use obs::OBS;
use std::collections::HashMap;

type Callback = fn(String) -> bool;


#[derive(Debug)]
struct System{
    id: String,
    mapper: HashMap<String,Callback>,
    gates: Vector<Gates>,

}


impl System{


    fn activate(&self, input: &OBS){
    }

    pub fn new(path: &str) -> System{

    }

}