from lib2to3.pgen2 import token
import time
import numpy as np
from collections import namedtuple
from typing import Callable


OBS = namedtuple("OBS", ["id", "sys_id","input","output"]) 
System = namedtuple("SYS", ["id","gates"])
GateInfo = namedtuple("GateInfo", ["gate","input","output"])

def list_from_file(obs_path: str) -> list[OBS]:
    def line_to_obs(line: str):
        line = line.replace('.[]()','').split(',')
        sys_id: str = line[0]
        id: str = line[1]
        input: list[bool] = list()
        output: list[bool] = list()
        for token in line[1:]:
            is_pos: bool = not ('-' in token)
            if 'i' in token:
                input.append(is_pos)
            elif 'o' in token:
                output.append(is_pos)
        input: np.array[bool] = np.array(input)
        output: np.array[bool] = np.array(output)
        return OBS(id, sys_id, input, output)
    
    with open(obs_path,"r") as obs_file:
        content: list[str] = obs_file.read().replace(".","").split("\n")
        obs_list: list[OBS] = list()
        for line in content:
            obs_list.append(line_to_obs(line))
        return obs_list 

def new_sys(sys_path: str) -> System:
    with open(sys_path,"r") as obs_file:
        content: list[str] = obs_file.read().replace(".","").replace("[","").replace("]","").replace(" ", "").split("\n")
        id: str = content[0]
        gates: list[GateInfo] = list()
        content: list[str] = content[3:]
        for line in content:
            tokens: list[str] = line.split(',')
            if tokens[-1] == "":
                del tokens[-1]
            gate: Callable = None
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
            gate_id: str  = tokens[1]
            output: str = tokens[2]
            inputs: list[str] = tokens[3:]
            gates.append(GateInfo(gate,inputs,output))
        return System(id,gates)
    
def activate(sys: System,input: OBS,change: np.array) -> list[bool]:
    i: list[bool] = input.input
    z: np.array[bool] = np.full((len(sys.gates)),False)
    out: np.array[bool] = np.full((len(input.output)),False)
    for idx,info in enumerate(sys.gates):
        gate_input: list[bool] = list(map(lambda x: i[int(x[1:])-1] if 'i' in x else z[int(x[1:])-1],info[1]))
        index: int = int(info[2][1:])-1
        activate_out: bool = info[0](gate_input)
        if change[idx]:
            activate_out = not activate_out
        if 'o' in info[2]:
            out[index] = activate_out
        else:
            z[index] = activate_out
    return out
    
def bfs(sys: System,obs: OBS,now: float,stop_time: float) -> tuple[int,list[set]]:
    deqeue: list[set] = list()
    minimal_set: list[set] = list()
    elapsed: float = time.time() * 1000
    # insert all tokens
    for idx in range(len(sys)):
        deqeue.append({idx})
    while len(deqeue) > 0 and (elapsed-now)< stop_time:
        combination: set = deqeue.pop(0);
        contains_combination: bool = False
        for set_ in minimal_set:
            if contains_combination: 
                break
            contains_combination |= len(set_.difference(combination)) == 0 
        if contains_combination:
            continue
        changes: np.array[bool] = np.full((len(sys.gates)),False)
        for val in combination:
            changes[val] = True
        out: list[bool] = activate(sys,obs,changes)
        matching: bool = all(map(lambda x: x[0] == x[1],zip(out,obs.output)))
        if matching:
            minimal_set.append(combination);
        else:
            for idx in range(0,len(sys.gates)):
                if not(idx in combination):
                    x: set = combination.copy()
                    x.add(idx)
                    deqeue.append(x)
        elapsed: float = time.time() * 1000
    return (elapsed-now,minimal_set)