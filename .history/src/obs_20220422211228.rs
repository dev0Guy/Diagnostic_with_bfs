#[derive(Debug)]

pub struct OBS{
    ids: Vec<i8>,
    sys: String,
    input: Vec<bool>,
    output: Vec<bool>,
}


impl OBS{

    fn open_file() -> (i32,String){
        return (8, "hello world".to_owned());
    }



    pub fn new(file_path: &str) -> OBS{
        // open file 
        //get number of row in file
        // 
        OBS{
            input: vec![false;10],
            output: vec![false;2]
        }
    }
}