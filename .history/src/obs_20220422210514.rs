#[derive(Debug)]
struct OBS{
    file_path: &str,
}


impl OBS{
    pub fn new(file_path: &str) -> OBS{
        OBS{
            file_path: file_path,
        }
    }
}