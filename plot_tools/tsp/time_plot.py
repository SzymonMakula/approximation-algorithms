import matplotlib.pyplot as plt
import json

pathname = f"../dist/calculations/tsp/christofides/approx_time.json"

result_data = open(pathname)
json_data = json.load(result_data)

# Data for plotting
t = json_data["dimension"]
s = json_data["time"]


fig, ax = plt.subplots()




ax.bar(t, s)

ax.legend(title='Czas wykonania jest uśredniony')
ax.set_xlabel('Liczba wierzchołków')
ax.set_ylabel('Czas wykonania algorytmu (ms)')
ax.set_title('Czas wykonania algorytmu Christofidesa w zależności od rozmiaru danych wejściowych')

plt.show()