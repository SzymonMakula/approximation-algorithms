import json

import matplotlib.pyplot as plt

pathname = f"../dist/knapsack/fptas/0.4/knapPI_3_100_1000.json"

result_data = open(pathname)
json_data = json.load(result_data)


def get_time(object):
    return object["time_micros"] / 1000000


def get_capacity(object):
    return object["capacity"]


runs = json_data["runs"]
times = list(map(get_time, runs))
capacities = list(map(get_capacity, runs))
indices = list(range(len(runs)))
print(capacities)

t = capacities
s = times

fig, ax = plt.subplots()

ax.scatter(t, s)

ax.legend(title='ε = 0.1')
ax.set_xlabel('Pojemność plecaka')
ax.set_ylabel('Czas wykonania (ms)')
ax.set_title('Czas wykonania algorytmu Greedy Knapsack')

plt.show()
