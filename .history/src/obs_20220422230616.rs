use std::{fs};

#[derive(Debug)]
pub struct OBS{
    id: u16,
    sys: String,
    input: Vec<bool>,
    output: Vec<bool>,
}

// Lazy object
impl OBS{


    fn line_to_obs(line: &str) -> (){
        let line =  line.replace(&['(',')',']','['], "");
        let line: Vec<&str> = line.split(",").collect();
        let sys_id = line[0];
        let input:Vec<bool> = Vec::new();
        let output: Vec<bool> = Vec::new();

        println!("{:?}",line);
        // return OBS::new(id,sys_id,);
    }

    pub fn list_from_file(file_path:&str) -> Vec<OBS>{
        let content = fs::read_to_string(file_path).expect("Unable to read file");
        let content = content.replace(".", "");
        let n_rows: usize = content.lines().count() as usize;
        let obs_list: Vec<OBS> = Vec::new();
        for line in content.lines() {
            OBS::line_to_obs(line)
            // obs_list.push(OBS::line_to_obs(line));
        }
        obs_list 
    }


    pub fn new(id: u16,
        sys: String,
        input: Vec<bool>,
        output: Vec<bool>) -> OBS{
        OBS{id,sys,input,output}
    }
}