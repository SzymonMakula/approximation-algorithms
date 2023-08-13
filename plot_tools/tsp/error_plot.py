import matplotlib.pyplot as plt
import json

pathname = f"../dist/calculations/tsp/christofides/approx_err.json"

result_data = open(pathname)
json_data = json.load(result_data)

# Data for plotting
t = json_data["dimension"]
s = json_data["errorPercentage"]


fig, ax = plt.subplots()




ax.bar(t, s)

ax.legend(title='Błąd procentowy jest uśredniony')
ax.set_xlabel('Liczba wierzchołków')
ax.set_ylabel('Błąd procentowy %')
ax.set_title('Błąd procentowy algorytmu Christofidesa')

plt.show()