import json
import sys

import numpy as np
from matplotlib import pyplot as plt

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
        "knapPI_3_100_10000", "knapPI_3_1000_1000", "knapPI_3_1000_10000"]
SET_NAMES = ["NSS", "NSM", "NMS", "NMM", "CSS", "CSM", "CMS", "CMM"]

errs_max = []
for set in SETS:
    pathname = f"../dist/knapsack/{filename}/{set}.json"
    try:
        result_data = open(pathname)
        json_data = json.load(result_data)
        errors = []

        runs = json_data["runs"]
        opts = list(map(get_opt, runs))
        results = list(map(get_result, runs))
        for run in runs:
            expected = run["optimum_value"]
            received = run["result"]
            err = abs((expected - received) / received) * 100
            errors.append(err)
        max_err = np.array(errors).max()
        errs_max.append(max_err)
    except:
        errs_max.append(0)

t = SET_NAMES
s = errs_max

fig, ax = plt.subplots()

ax.bar(t, s)

ax.legend(title='ε=0.1')
ax.set_xlabel('Nazwa zbioru')
ax.set_ylabel('Maksymalny błąd procentowy (%)')
ax.set_title(f'Średni bład procentowy rozwiązań algorytmu {algorithm_name}')

plt.show()
