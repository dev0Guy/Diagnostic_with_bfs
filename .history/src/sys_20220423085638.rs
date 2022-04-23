
mod obs;
use obs::OBS;

#[derive(Debug)]
struct System{
    id: String,
    mapper: Mapper<String>,
    
}


impl System{


    fn activate(&self, input: &OBS){
    }

    pub fn new(path: &str) -> System{

    }

}