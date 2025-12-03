import b1p1_change from "../../target/criterion/problem 1 - part 1/change/estimates.json";
import b1p2_change from "../../target/criterion/problem 1 - part 2/change/estimates.json";
import b2p1_change from "../../target/criterion/problem 2 - part 1/change/estimates.json";
import b2p2_change from "../../target/criterion/problem 2 - part 2/change/estimates.json";
import b3p1_change from "../../target/criterion/problem 3 - part 1/change/estimates.json";
import b3p2_change from "../../target/criterion/problem 3 - part 2/change/estimates.json";

import b1p1_new from "../../target/criterion/problem 1 - part 1/new/estimates.json";
import b1p2_new from "../../target/criterion/problem 1 - part 2/new/estimates.json";
import b2p1_new from "../../target/criterion/problem 2 - part 1/new/estimates.json";
import b2p2_new from "../../target/criterion/problem 2 - part 2/new/estimates.json";
import b3p1_new from "../../target/criterion/problem 3 - part 1/new/estimates.json";
import b3p2_new from "../../target/criterion/problem 3 - part 2/new/estimates.json";

import b1p1_info from "../../target/criterion/problem 1 - part 1/new/benchmark.json";
import b1p2_info from "../../target/criterion/problem 1 - part 2/new/benchmark.json";
import b2p1_info from "../../target/criterion/problem 2 - part 1/new/benchmark.json";
import b2p2_info from "../../target/criterion/problem 2 - part 2/new/benchmark.json";
import b3p1_info from "../../target/criterion/problem 3 - part 1/new/benchmark.json";
import b3p2_info from "../../target/criterion/problem 3 - part 2/new/benchmark.json";

export interface Confidence {
    confidence_level: number;
    lower_bound: number;
    upper_bound: number;
}

export interface EstimateInfo {
    confidence_interval: Confidence;
    point_estimate: number;
    standard_error: number;
}

export interface ChangeEstimate {
    mean: EstimateInfo;
    median: EstimateInfo;
}

export interface FullEstimate extends ChangeEstimate {
    median_abs_dev: EstimateInfo;
    slope: EstimateInfo | null;
    std_dev: EstimateInfo;
}

export interface BenchInfo {
    group_id: string;
    function_id?: unknown;
    value_str?: unknown;
    throughput?: unknown;
    full_id: string;
    directory_name: string;
    title: string;
}

export interface ProblemPart {
    change: ChangeEstimate;
    new: FullEstimate;
    info: BenchInfo;
}

export interface Problem {
    day: number;
    name: string;
    part1: ProblemPart;
    part2: ProblemPart;
}

const problem1: Problem = {
    day: 1,
    name: "Secret Enterance",

    part1: {
        change: b1p1_change,
        new: b1p1_new,
        info: b1p1_info,
    },

    part2: {
        change: b1p2_change,
        new: b1p2_new,
        info: b1p2_info,
    },
};

const problem2: Problem = {
    day: 2,
    name: "Gift Shop",

    part1: {
        change: b2p1_change,
        new: b2p1_new,
        info: b2p1_info,
    },

    part2: {
        change: b2p2_change,
        new: b2p2_new,
        info: b2p2_info,
    },
};

const problem3: Problem = {
    day: 3,
    name: "Lobby",

    part1: {
        change: b3p1_change,
        new: b3p1_new,
        info: b3p1_info,
    },

    part2: {
        change: b3p2_change,
        new: b3p2_new,
        info: b3p2_info,
    },
};

export const problems = [problem1, problem2, problem3];
