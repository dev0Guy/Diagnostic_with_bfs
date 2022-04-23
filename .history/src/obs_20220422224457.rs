use std::{fs};

#[derive(Debug)]
pub struct OBS{
    id: u16,
    sys: String,
    input: Vec<bool>,
    output: Vec<bool>,
}

// Lazy object
impl OBS{

    pub fn list_from_file(file_path:&str) -> Vec<Option<&OBS>>{
        let content = fs::read_to_string(file_path).expect("Unable to read file");
        let n_rows: usize = content.lines().count() as usize;
        let obs_list: Vec<Option<&OBS>> = vec![None;n_rows];
        for line in content.lines() {

        }


        obs_list 
    }


    pub fn new(id: u16,
        sys: String,
        input: Vec<bool>,
        output: Vec<bool>) -> OBS{
        OBS{id,sys,input,output}
    }
}