
mod obs;
use obs::OBS;
use std::collections::HashMap;

type Callback = fn(String) -> Option<()>;


#[derive(Debug)]
struct System{
    id: String,
    mapper: HashMap<String,>,

}


impl System{


    fn activate(&self, input: &OBS){
    }

    pub fn new(path: &str) -> System{

    }

}