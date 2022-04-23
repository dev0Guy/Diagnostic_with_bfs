#[derive(Debug)]
struct OBS{
}


impl OBS{
    pub fn new(file_path: &str) -> OBS{
        OBS{
            file_path: file_path,
        }
    }
}