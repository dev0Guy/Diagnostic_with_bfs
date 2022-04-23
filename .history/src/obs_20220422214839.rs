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
        let num: u16 = content.lines().count() as usize;
        return (num, content);
    }



    pub fn new(file_path: &str) -> OBS{
        // open file
        let (n_rows, content) =  OBS::open_file(file_path);
        let ids = vec![0;n_rows];
        println!("{}",n_rows);
        OBS{
            ids: vec![10;1],
            sys: "C_17".to_owned(),
            input: vec![false;10],
            output: vec![false;2]
        }
    }
}