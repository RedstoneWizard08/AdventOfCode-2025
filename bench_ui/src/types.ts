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
    new: FullEstimate;
    info: BenchInfo;
}

export interface Problem {
    day: number;
    name: string;
    part1: ProblemPart;
    part2: ProblemPart;
}
