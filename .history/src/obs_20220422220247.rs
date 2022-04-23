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

    fn open_file(file_path:&str) -> (usize,usize,usize,String){
        let content = fs::read_to_string(file_path).expect("Unable to read file");
        let n_rows: usize = content.lines().count() as usize;
        first_line =  content.lines()[0];
        let input_size: usize= 4;
        let output_size: usize= 4;
        (n_rows,input_size,output_size,content)
    }

    pub fn new(file_path: &str) -> OBS{
        // open file
        let (n_rows,input_size,output_size, content) =  OBS::open_file(file_path);
        // create obs params
        let mut ids = vec![0;n_rows];
        let mut input = vec![vec![false;3];n_rows];
        let mut output = vec![vec![false;3];n_rows];
        let mut sys  = "c_17".to_owned();
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