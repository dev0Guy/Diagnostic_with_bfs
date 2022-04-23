use std::{fs};

#[derive(Debug)]
pub struct OBS{
    ids: Vec<u16>,
    sys: String,
    input: Vec<Vec<bool>>,
    output: Vec<Vec<bool>>,
}

// Lazy object
impl OBS{

    fn open_file(file_path:&str) -> (usize,usize,usize,String,String){
        let content = fs::read_to_string(file_path).expect("Unable to read file");
        let n_rows: usize = content.lines().count() as usize;
        let mut first_line = "asda";
        for line in content.lines(){
            first_line = line;
            first_line.replace(".", "");
            break;
        }
        for token in 
        // let array_val = splitted[2..];
        let sys_name = "23213".to_owned();
        let input_size: usize= 4;
        let output_size: usize= 4;
        (n_rows,input_size,output_size,sys_name,content)
    }

    pub fn new(file_path: &str) -> OBS{
        // open file
        let (n_rows,input_size,output_size,sys ,content) =  OBS::open_file(file_path);
        // create obs params
        let mut ids = vec![0;n_rows];
        let input = vec![vec![false;input_size];n_rows];
        let output = vec![vec![false;output_size];n_rows];
        // run on all lines and save inputs, outputs
        for id in 0..n_rows{
            ids[id] = (id + 1) as u16;
            // input[id] = 
        }
        OBS{
            ids,input,output,sys,
        }
    }
}