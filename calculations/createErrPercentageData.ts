import {mkdirSync, readdirSync, readFileSync, writeFileSync} from "fs";
import {Benchmark} from "./types";

const problems = readdirSync("../dist/tsp")

problems.forEach(problemName => {
    const benchmarks = readdirSync(`../dist/tsp/${problemName}/`)
    const error_record: Record<string, number[]> = {}

    benchmarks.forEach(benchmark => {
        const fileTextData = readFileSync(`../dist/tsp/${problemName}/${benchmark}`).toString()
        const file = JSON.parse(fileTextData) as Benchmark


        if (benchmark === "pr439.json" && problemName === "christofides") {
            file.run_results[0].tour.forEach((stop, index) => {
                if (file.run_results[0].tour[index] !== file.run_results[1].tour[index]) {
                    console.log("first ", file.run_results[0].tour[index], "second", file.run_results[1].tour[index])
                }
            })
        }
        const received_values = file.run_results.map(run => run.result)
        const averageResult = received_values.reduce((acc, currentValue) => acc + currentValue, 0) / file.runs
        const errPercentage = Number(Math.abs((file.data_set.optimum - averageResult) / averageResult) * 100)
        error_record[file.data_set.dimension] = error_record[file.data_set.dimension] ? [...error_record[file.data_set.dimension], errPercentage] : [errPercentage]
    })

    Object.keys(error_record).forEach(key => {
        error_record[key] = [error_record[key].reduce((acc, cur) => acc + cur, 0) / error_record[key].length]
    })
    const output = {
        dimension: Object.keys(error_record),
        errorPercentage: Object.values(error_record).map(valueArray => Number(valueArray[0].toFixed(2)))
    }
    try {
        mkdirSync(`../dist/calculations/tsp/${problemName}`, {recursive: true})
    } catch (err) {
        // Directory already exists
    }
    writeFileSync(`../dist/calculations/tsp/${problemName}/approx_err.json`, JSON.stringify(output))
})

