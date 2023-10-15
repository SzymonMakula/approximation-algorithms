# Approximation algorithms
Rust implmentation of various approximation algorithms:
- Christofides algorithm
- Double tree algorithm
- Greedy knapsack
- FPTAS kanpsack
- Dynamic knapsack

Uses Blossom V algortihm from `Vladimir Kolmogorov. "Blossom V: A new implementation of a minimum cost perfect matching algorithm." In Mathematical Programming Computation (MPC), July 2009, 1(1):43-67.`. Source code and license may be found at /approx_algorithms/lib

## Running algortihms
To run bnechmarking tests, go to Rust project folder:
```bash
cd approx_algorithms
```
Then run:
```bash
cargo run
```
This should produce benchmaking data in `/dist` folder, located in root directory. You may use this day to produce plots using scripts included in `plot_tools`
