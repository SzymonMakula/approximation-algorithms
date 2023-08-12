import {mkdirSync, readdirSync, readFileSync, writeFileSync} from "fs";
import {Benchmark} from "./types";


const problems = readdirSync("../dist/tsp")
problems.forEach(problemName => {
    const benchmarks = readdirSync(`../dist/tsp/${problemName}/`)
    benchmarks.forEach(benchmark => {
        const fileTextData = readFileSync(`../dist/tsp/${problemName}/${benchmark}`).toString()
        const file = JSON.parse(fileTextData) as Benchmark

        const executionTimes = file.run_results.map(run => run.elapsed_micros)
        const received_values = file.run_results.map(run => run.result)

        const averageResult = received_values.reduce((acc, currentValue) => acc + currentValue, 0) / file.runs
        const averageTime = executionTimes.reduce((acc, currentValue) => acc + currentValue, 0) / file.runs

        const sortedTime = executionTimes.sort().reverse()
        const medianTime = (sortedTime[sortedTime.length / 2] + sortedTime[sortedTime.length / 2 + 1]) / 2
        const errPercentage = Number(Math.abs((file.data_set.optimum - averageResult) / averageResult) * 100).toFixed(2)
        console.log("median ", medianTime)
        console.log("Average result", averageResult)
        console.log("average time", averageTime)
        console.log("optimum", file.data_set.optimum)
        console.log("percetnage error ", errPercentage, "%")
        const output = {
            averageTime: Number(averageTime / 1000).toFixed(2),
            medianTime: Number(medianTime / 1000).toFixed(2),
            averageValue: Number(averageResult).toFixed(2),
            optimum: file.data_set.optimum,
            errPercentage
        }

        try {
            mkdirSync(`../dist/calculations/tsp/${problemName}`, {recursive: true})
        } catch (err) {
            // Directory already exists
        }
        writeFileSync(`../dist/calculations/tsp/${problemName}/${benchmark}`, JSON.stringify(output))
    })
})