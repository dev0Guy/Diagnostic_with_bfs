use std::{fs, slice::range};

#[derive(Debug)]
pub struct OBS{
    ids: Vec<u16>,
    sys: String,
    input: Vec<bool>,
    output: Vec<bool>,
}

// Lazy object
impl OBS{

    fn open_file(file_path:&str) -> (usize,String){
        let content = fs::read_to_string(file_path).expect("Unable to read file");
        let n_rows: usize = content.lines().count() as usize;
        (n_rows,content)
    }


    pub fn new(file_path: &str) -> OBS{
        // open file
        let (n_rows, content) =  OBS::open_file(file_path);
        // create obs params
        let ids = vec![0;n_rows];
        let input = vec![false;n_rows];
        let output = vec![false;n_rows];
        let sys  = "c_17".to_owned();
        // run on all lines and save inputs, outputs
        for id in 0..n_rows{
            ids[id] = id;
        }
        OBS{
            ids,input,output,sys,
        }
    }
}