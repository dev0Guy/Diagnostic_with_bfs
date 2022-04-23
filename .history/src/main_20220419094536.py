
PATH = "./data_for_exercises/circuits"

SYSTEM_PATH = f'{PATH}/Data_Systems'
OBSERVATION_PATH = f'{PATH}/Data_Observations'


def opt_to_func(id, inputs):
    if id == 'nand2':
        val = True
        for input in inputs:
            
        return val



if __name__ == '__main__':
    file_name = "c17.sys";
    
    with open(f'{SYSTEM_PATH}/{file_name}') as f:
        text = f.read().replace("\n","").split('.')
        sys_id, inputs, outputs, gates,_ = text
        # 
        gates = gates[2:-2].split('],[')
        inputs = inputs[1:-1].split(',')
        outputs = outputs[1:-1].split(',')
        
        inputs_val = {name: 0 for name in inputs}
        outputs_val = {name: -1 for name in outputs}
        
        opt_val = {}
        # 
        for gate in gates:
            gate = gate.split(',')
            opt,out_id,inputs = gate[0],gate[2], gate[3:]
            opt_val[out_id] = 0
            print(opt,out_id,inputs)
            # opt,_,out,inputs = gate
        
        