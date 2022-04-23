use std::fs;

#[derive(Debug)]
pub struct OBS{
    ids: Vec<u16>,
    sys: String,
    input: Vec<bool>,
    output: Vec<bool>,
}


impl OBS{

    fn open_file(file_path:&str) -> (usize,String){
        let content = fs::read_to_string(file_path).expect("Unable to read file");
        let num: usize = content.lines().count() as usize;
        return (num, content);
    }



    pub fn new(file_path: &str) -> OBS{
        // open file
        let (n_rows, content) =  OBS::open_file(file_path);
        let ids = vec![0;n_rows];
        let input = vec![false;n_rows];
        let output = vec![false;n_rows];
        let sys  = "c_17".to_owned(); 
        println!("{}",n_rows);
        OBS{
            ids,inputs,output,sys,
        }
    }
}