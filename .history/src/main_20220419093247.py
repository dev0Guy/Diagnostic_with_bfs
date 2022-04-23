
PATH = "./data_for_exercises/circuits"

SYSTEM_PATH = f'{PATH}/Data_Systems'
OBSERVATION_PATH = f'{PATH}/Data_Observations'

if __name__ == '__main__':
    file_name = "74181.sys";
    
    with open(f'{SYSTEM_PATH}/{file_name}') as f:
        text = f.read().replace("\n","").split('.')
        sys_id, inputs, outputs, gates,_ = text
        inputs = inputs[1:-1].split(',')
        outputs = outputs[1:-1].split(',')
        gates = gates[1:-1].split(',')
        input_dict = {name: 0 for name in inputs}
        # output_dict = {name: 0 for name in outputs}
        print(input_dict)
        print(output_dict)
        
        # print(inputs)
        # print(outputs)
        # print(gates)
        
        
        