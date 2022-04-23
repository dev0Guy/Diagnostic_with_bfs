
mod obs;
mod gates;
use obs::OBS;
use gates::GatesOptions;
use std::collections::HashMap;

type Callback = fn(&System,String) -> bool;


#[derive(Debug)]
struct System{
    id: String,
    mapper: HashMap<String,Callback>,
    gates: Vector<GatesOptions>,

}


impl System{


    fn activate(&self, input: &OBS){
    }

    pub fn new(path: &str) -> System{

    }

}