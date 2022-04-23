#[derive(Debug)]

pub struct OBS{
    input: Vec<bool>,
    output: Vec<bool>,
}


impl OBS{
    pub fn new(file_path: &str) -> OBS{
        OBS{
            input: vec![false;10],

        }
    }
}