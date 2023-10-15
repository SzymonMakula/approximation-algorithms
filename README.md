# Approximation algorithms
Rust implmentation of various approximation algorithms:
- Christofides algorithm
- Double tree algorithm
- Greedy knapsack
- FPTAS kanpsack
- Dynamic knapsack

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
