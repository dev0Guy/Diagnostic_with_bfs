
PATH = "./data_for_exercises/circuits"

SYSTEM_PATH = f'{PATH}/Data_Systems'
OBSERVATION_PATH = f'{PATH}/Data_Observations'

if __name__ == '__main__':
    file_name = "74181.sys";
    
    with open(f'{SYSTEM_PATH}/{file_name}') as f:
        text = f.read().replace("\n","").split('.')
        sys_id, inputs, outputs, gates,_ = text
        inputs = inputs[1:-1].split(',')[1:-1]
        outputs = outputs[1:-1].split(',')[1:-1]
        gates = gates[1:-1].split(',')[1:-1]
        
        print(inputs)
        print(outputs)
        print(gates)
        
        
        