#[derive(Debug)]

pub struct OBS{
    ids: Vec<u16>,
    sys: String,
    input: Vec<bool>,
    output: Vec<bool>,
}


impl OBS{

    fn open_file() -> (u16,String){
        return (8, "hello world".to_owned());
    }



    pub fn new(file_path: &str) -> OBS{
        // open file 
        //get number of row in file
        // 
        OBS{
            ids: vec![10;1],
            sys: 'C_17'.to_owned(),
            input: vec![false;10],
            output: vec![false;2]
        }
    }
}