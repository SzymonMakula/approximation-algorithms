import tsplib95
import matplotlib.pyplot as plt
import networkx as nx
import sys
import json

if len(sys.argv) < 2:
    print("Please provide valid filename")
    sys.exit(1)

problem_name = sys.argv[1]
chosen_algorithm = "christofides"
if len(sys.argv) == 3:
    if sys.argv[2] == "t":
        chosen_algorithm = "dtree"
    if sys.argv[2] == "c":
        chosen_algorithm = "christofides"

problem_pathname = f"../tsp/datasets/problems/{problem_name}.tsp"
result_pathname = f"../../dist/tsp/{chosen_algorithm}/{problem_name}.json"

result_data = open(result_pathname)
json_data = json.load(result_data)
fist_problem = json_data["run_results"][0]
tour = fist_problem["tour"]

problem = tsplib95.load(problem_pathname)
G = problem.get_graph()
sol = nx.algorithms.approximation.christofides(G)

print(sys.argv[1])

pos = nx.get_node_attributes(G, "coord")
edge_list = list(nx.utils.pairwise(tour))

nx.draw_networkx(
    G,
    pos,
    edgelist=edge_list,
    with_labels=False,
    edge_color="red",
    node_size=2,
    width=1,
)

plt.show()