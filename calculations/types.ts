type RunResult = {
    elapsed_micros: number;
    result: number;
    tour: number[]
}

export type Benchmark = {
    runs: number,
    data_set: {
        name: string,
        dimension: number,
        optimum: number,
    },
    run_results: RunResult[]
}
