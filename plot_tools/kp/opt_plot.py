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
    case _:
        algorithm_name = "FPTAS"


def get_result(object):
    return object["result"]


def get_opt(object):
    return object["optimum_value"]


filename = sys.argv[1]

SETS = ["knapPI_1_100_1000", "knapPI_1_100_10000", "knapPI_1_1000_1000", "knapPI_1_1000_10000", "knapPI_3_100_1000",
        "knapPI_3_100_10000", "knapPI_3_1000_1000"]
SET_NAMES = ["NSS", "NSM", "NMS", "NMM", "CSS", "CSM", "CMS"]

opt_count = []
for set in SETS:
    pathname = f"../dist/knapsack/{filename}/{set}.json"
    try:
        result_data = open(pathname)
        json_data = json.load(result_data)

        runs = json_data["runs"]
        opts = list(map(get_opt, runs))
        results = list(map(get_result, runs))
        a = np.array(results)
        b = np.array(opts)
        same = np.sum(a == b)
        opt_count.append(same)
    except:
        opt_count.append(0)

t = SET_NAMES
s = opt_count

fig, ax = plt.subplots()

bar = ax.bar(t, s)
ax.bar_label(bar, fmt='{:,.0f}')

ax.legend(title='ε=0.1')
ax.set_xlabel('Nazwa zbioru')
ax.set_ylabel('Ilość optymalnych rozwiązań')
ax.set_title(f'Ilość optymalnych rozwiązań algorytmu {algorithm_name}')

plt.show()
