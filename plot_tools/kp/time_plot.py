import json
import sys

import matplotlib.pyplot as plt
import numpy as np

# Valid: greedy, ftpas/e-value, dynamic, dynamicW
if len(sys.argv) < 1:
    print("Please provide algorithm type")
    sys.exit(1)

algorithm_name = "Greedy Knapsack"
match sys.argv[1]:
    case "greedy":
        algorithm_name = "Greedy Knapsack"
    case "dynamic":
        algorithm_name = "Dynamic Knapsack"
    case "dynamicW":
        algorithm_name = "Dynamic Knapsack O(nW)"
    case _:
        algorithm_name = "FPTAS"


def get_time(object):
    return object["time_micros"] / 1000


def get_capacity(object):
    return object["capacity"]


filename = sys.argv[1]

# "knapPI_3_1000_10000"
# CMM

SETS = ["knapPI_1_100_1000", "knapPI_1_100_10000", "knapPI_1_1000_1000", "knapPI_1_1000_10000", "knapPI_3_100_1000",
        "knapPI_3_100_10000", "knapPI_3_1000_1000"]
SET_NAMES = ["NSS", "NSM", "NMS", "NMM", "CSS", "CSM", "CMS"]

avg_time_count = []
for set in SETS:
    pathname = f"../dist/knapsack/{filename}/{set}.json"
    try:
        result_data = open(pathname)
        json_data = json.load(result_data)

        runs = json_data["runs"]
        time = np.array(list(map(get_time, runs)))
        avg_time = time.mean()
        avg_time_count.append(avg_time)
    except:
        avg_time_count.append(0)

t = SET_NAMES
s = avg_time_count

fig, ax = plt.subplots()

bar = ax.bar(t, s)
ax.bar_label(bar, fmt='{:,.0f}')

# ax.legend(title='ε=0.1')
ax.set_xlabel('Nazwa zbioru')
ax.set_ylabel('Średni czas rozwiązania (ms)')
ax.set_title(f'Średni czas działania algorytmu {algorithm_name}')
plt.show()
