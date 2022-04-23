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

    pub fn list_from_file(file_path:&str) -> Vec<OBS>{

    }


    fn open_file(file_path:&str) -> (usize,usize,usize,String,String){
        let content = fs::read_to_string(file_path).expect("Unable to read file");
        let n_rows: usize = content.lines().count() as usize;
        let mut first_line = "asda";
        for line in content.lines(){
            first_line = line;
            // first_line = first_line.split("[").collect();
        }
        // let array_val = splitted[2..];
        let sys_name = "23213".to_owned();
        let input_size: usize= 4;
        let output_size: usize= 4;
        (n_rows,input_size,output_size,sys_name,content)
    }

    pub fn new(id: u16,
        sys: String,
        input: Vec<bool>,
        output: Vec<bool>) -> OBS{
        OBS{id: u16,
            sys: String,
            input: Vec<bool>,
            output: Vec<bool>,}
    }
}