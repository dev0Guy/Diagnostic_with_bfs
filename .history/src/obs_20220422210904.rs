#[derive(Debug)]

pub struct OBS{
    input: Vec<bool>,
    val: i32
}


impl OBS{
    pub fn new(file_path: &str) -> OBS{
        OBS{val:5,}
    }
}