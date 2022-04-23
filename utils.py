from lib2to3.pgen2 import token
import time
import numpy as np
from collections import namedtuple

OBS = namedtuple("OBS", ["id", "sys_id","input","output"]) 
System = namedtuple("SYS", ["id","gates"])
GateInfo = namedtuple("GateInfo", ["gate","input","output"])


def list_from_file(obs_path: str) -> list[OBS]:
    def line_to_obs(line: str):
        line = line.replace('.[]()','').split(',')
        sys_id,id = line[0],line[1]
        input, output = list(), list()
        for token in line[1:]:
            is_pos = not ('-' in token)
            if 'i' in token:
                input.append(is_pos)
            else:
                output.append(is_pos)
        input = np.array(input)
        output = np.array(output)
        return OBS(id, sys_id, input, output)
    
    with open(obs_path,"r") as obs_file:
        content = obs_file.read().replace(".","").split("\n")
        obs_list = list()
        for line in content:
            obs_list.append(line_to_obs(line))
        return obs_list 

def new_sys(sys_path: str):
    with open(sys_path,"r") as obs_file:
        content = obs_file.read().replace(".","").replace("[","").replace("]","").replace(" ", "").split("\n")
        id = content[0]
        gates = list()
        content = content[3:]
        for line in content:
            tokens = line.split(',')
            if tokens[-1] == "":
                del tokens[-1]
            gate = None
            if len(tokens) ==0 :
                continue
            if tokens[0] == "inverter":
                gate = lambda x: not x 
            elif tokens[0] == "buffer":
                gate = lambda x: x
            elif 'nand' in tokens[0]:
                gate = lambda x: not all(x)
            elif 'and' in tokens[0]:
                gate = lambda x: all(x)
            elif 'nor' in tokens[0]:
                gate = lambda x: not any(x)
            elif 'or' in tokens[0]:
                gate = lambda x: any(x)
            elif 'xor' in tokens[0]:
                gate = lambda x: x[0] ^ x[1]
            gate_id = tokens[1]
            output = tokens[2]
            inputs = tokens[3:]
            gates.append(GateInfo(gate,inputs,output))
        return System(id,gates)
    
    
def bfs(sys,obs,now,stop_time):
    deqeue = list()
    minimal_set = list()
    elapsed = round(time.time() * 1000)
    # insert all tokens
    for idx in range(0,len(sys)):
        deqeue.append({idx})
    while len(deqeue) > 0 and elapsed < stop_time:
        combination = deqeue.pop(0);
        contains_combination = False
        for set_ in minimal_set:
            if contains_combination: 
                break
            contains_combination |= len(set_.difference(combination)) == 0 
        if contains_combination:
            continue
        
        let mut changes = vec![false;sys.len()];
        combination.iter().for_each(|val| changes[*val] = true);
        let out = sys.activate(obs,changes);
        let matching = out.iter().zip(&obs.output).filter(|&(a, b)| a != b).count();
        if matching == 0{
            minimal_set.push(combination);
        }else{
            for idx in 0..sys.len()-1{
                if !combination.contains(&idx){
                    let mut x = combination.clone();
                    x.insert(idx);
                    deqeue.push(x);
                }
            }
        }
        elapsed = now.elapsed().as_micros() as usize;
    }
    (elapsed,minimal_set)