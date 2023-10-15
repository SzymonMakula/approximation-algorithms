import json

import matplotlib.pyplot as plt
import numpy as np


def get_limit(object):
    limit = object["limit"]
    len = object["len"]
    print(limit, len)
    return (len / limit) * 100


def get_len(object):
    return object["len"]


avg_time_count = []

pathname = f"../dist/knapsack/dynamic/estimate.json"
result_data = open(pathname)
json_data = json.load(result_data)

runs = json_data["estimates"]
length = np.array(list(map(get_limit, runs)))
limits = np.array(list(map(get_limit, runs)))
print(length)
t = np.array(range(100))
s = length

fig, ax = plt.subplots()

bar = ax.bar(t, s)

ax.legend(title='Egzemplarz NSM')
ax.set_ylabel('Osiągnięty % limitu długości tablicy A[n]')
ax.set_title(f'Stosunek długości tablicy A[n] do jej maksymalnej długości')
ax.set_xlabel('Numer egzemplarza')

plt.show()
