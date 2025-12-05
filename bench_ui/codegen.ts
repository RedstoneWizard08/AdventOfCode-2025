const problems = [
    "Secret Enterance", // 1
    "Gift Shop", // 2
    "Lobby", // 3
    "Printing Department", // 4
    "Cafeteria", // 5
    // "??", // 6
    // "??", // 7
    // "??", // 8
    // "??", // 9
    // "??", // 10
    // "??", // 11
    // "??", // 12
];

const imports = [
    'import type { Problem } from "./types";',
];

const defs = [];
const arr = [];

const unindent = (s: string) =>
    s.split("\n").map((it) =>
        (it.length > 8 && it.substring(0, 8).trim().length == 0
            ? it.substring(8)
            : it).trimEnd()
    )
        .join("\n").trim();

for (let i = 0; i < problems.length; i++) {
    const name = problems[i];

    imports.push(`
        import day_${i}_part_1_est from "../../target/criterion/problem 1 - part 1/new/estimates.json";
        import day_${i}_part_2_est from "../../target/criterion/problem 1 - part 2/new/estimates.json";
        import day_${i}_part_1_info from "../../target/criterion/problem 1 - part 1/new/benchmark.json";
        import day_${i}_part_2_info from "../../target/criterion/problem 1 - part 2/new/benchmark.json";
    `);

    defs.push(`
        const problem${i}: Problem = {
            day: ${i},
            name: "${name}",

            part1: {
                new: day_${i}_part_1_est,
                info: day_${i}_part_1_info,
            },

            part2: {
                new: day_${i}_part_2_est,
                info: day_${i}_part_2_info,
            },
        };
    `);

    arr.push(`problem${i}`);
}

const imp = imports.map(unindent).join("\n\n");
const def = defs.map(unindent).join("\n\n");
const ret = `export const problems = [${arr.map(unindent).join(", ")}];`;
const full = [imp, def, ret].join("\n\n");

await Bun.write("src/problems.ts", full);
