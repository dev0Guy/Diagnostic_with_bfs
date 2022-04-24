import argparse
import csv
import time
from utils import *

if __name__ == "__main__":
    # ======== Creeate Praser ========
    parser = argparse.ArgumentParser()
    parser.add_argument("--observation-path", type=str, required=True)
    parser.add_argument("--system-path", type=str, required=True)
    args = parser.parse_args()
    # ======= Get Params From Parser =======
    observation_path: str = args.observation_path
    system_path: str = args.system_path
    # ======= Get OBS List From File =======
    obs_list: list[OBS] = list_from_file(observation_path)
    sys: System = new_sys(system_path)
    # ======= Run On All Observersions =======
    with open(f'out/{sys.id}.csv', 'w', encoding='UTF8') as csv_file:
        fields = ['System Name', 'Observation no.', 'Number of Diagnoses', 'Runtime (ms)']
        # create the csv writer
        writer = csv.writer(csv_file)
        for idx, obs in enumerate(obs_list):
            now: float = time.time() * 1000
            stop_time = 2*60*1000
            elapsed, info = bfs(sys, obs, now, stop_time)
            diagnostic = len(info)
            minimal_set = min(map(lambda x: len(x), info))
            row = [sys.id, idx+1, diagnostic, minimal_set, elapsed]
            writer.writerow(row)
    # ======= Run Commands =======
        # python3 python/script.py --observation-path  "data_for_exercises/circuits/Data_Observations/c17_iscas85.obs" --system-path "data_for_exercises/circuits/Data_Systems/c17.sys"
        # python3 python/script.py --observation-path  "data_for_exercises/circuits/Data_Observations/74182_iscas85.obs" --system-path "data_for_exercises/circuits/Data_Systems/74182.sys"
        # python3 python/script.py --observation-path  "data_for_exercises/circuits/Data_Observations/74181_iscas85.obs" --system-path "data_for_exercises/circuits/Data_Systems/74181.sys"
        # python3 python/script.py --observation-path  "data_for_exercises/circuits/Data_Observations/74182_iscas83.obs" --system-path "data_for_exercises/circuits/Data_Systems/74183.sys"





