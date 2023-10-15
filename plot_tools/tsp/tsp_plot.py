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
    print(sys.argv[2])
    if sys.argv[2] == "t":
        chosen_algorithm = "dtree"
    if sys.argv[2] == "c":
        chosen_algorithm = "christofides"


print(chosen_algorithm)

problem_pathname = f"../datasets/tsp/problems/{problem_name}.tsp"
result_pathname = f"../dist/tsp/{chosen_algorithm}/{problem_name}.json"

result_data = open(result_pathname)
json_data = json.load(result_data)
fist_problem = json_data["run_results"][0]
tour = fist_problem["tour"]

problem = tsplib95.load(problem_pathname)
G = problem.get_graph()
sol = nx.algorithms.approximation.christofides(G)


pos = nx.get_node_attributes(G, "coord")
edge_list = list(nx.utils.pairwise(tour))

plt.figure(1)
plt.ylabel('Współrzędne Y')
plt.xlabel('Współrzędne X')
plt.axis("on")
plt.title(f"Aproksymowana marszruta {problem_name}")


nx.draw_networkx(
    G,
    pos,
    edgelist=edge_list,
    with_labels=False,
    edge_color="red",
    node_size=2,
    width=1,
)

if json_data["data_set"]["opt_tour"] is not None:
    plt.figure(2)
    opt_tour = json_data["data_set"]["opt_tour"]
    plt.title(f"Optymalna marszruta {problem_name}")
    plt.ylabel('Współrzędne Y')
    plt.xlabel('Współrzędne X')
    opt_edge_list = list(nx.utils.pairwise(opt_tour))
    nx.draw_networkx(
        G,
        pos,
        edgelist=opt_edge_list,
        with_labels=False,
        edge_color="green",
        node_size=2,
        width=1,
    )

plt.show()