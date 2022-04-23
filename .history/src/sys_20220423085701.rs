
mod obs;
use obs::OBS;
use std::collections::HashMap;

#[derive(Debug)]
struct System{
    id: String,
    mapper: HashMap<String>,

}


impl System{


    fn activate(&self, input: &OBS){
    }

    pub fn new(path: &str) -> System{

    }

}