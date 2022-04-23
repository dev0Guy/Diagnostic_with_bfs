pub fn BFS(sys:System,obs_list:Vec<OBS>)->Vec<Vec<uszie>>{
    for obs in obs_list.iter() {
        let mut changes =  vec![false;sys.len()];
        println!("{:?}",  sys.activate(obs,changes));
    }
}