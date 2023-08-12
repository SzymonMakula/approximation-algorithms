import {mkdirSync, readdirSync, readFileSync, writeFileSync} from "fs";
import {Benchmark} from "./types";

const problems = readdirSync("../dist/tsp")

problems.forEach(problemName => {
    const benchmarks = readdirSync(`../dist/tsp/${problemName}/`)
    const time_record: Record<string, number[]> = {}

    benchmarks.forEach(benchmark => {
        const fileTextData = readFileSync(`../dist/tsp/${problemName}/${benchmark}`).toString()
        const file = JSON.parse(fileTextData) as Benchmark

        const executionTimes = file.run_results.map(run => run.elapsed_micros)
        const averageTime = (executionTimes.reduce((acc, currentValue) => acc + currentValue, 0) / file.runs) / 1000
        time_record[file.data_set.dimension] = time_record[file.data_set.dimension] ? [...time_record[file.data_set.dimension], averageTime] : [averageTime]
    })

    Object.keys(time_record).forEach(key => {
        time_record[key] = [time_record[key].reduce((acc, cur) => acc + cur, 0) / time_record[key].length]
    })
    const output = {
        dimension: Object.keys(time_record),
        time: Object.values(time_record).map(valueArray => valueArray[0])
    }
    try {
        mkdirSync(`../dist/calculations/tsp/${problemName}`, {recursive: true})
    } catch (err) {
        // Directory already exists
    }
    writeFileSync(`../dist/calculations/tsp/${problemName}/approx_time.json`, JSON.stringify(output))
})

