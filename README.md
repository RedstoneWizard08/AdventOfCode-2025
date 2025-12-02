# Advent of Code 2025

These are my answers for the
[Advent of Code 2025](https://adventofcode.com/2025)!

Go solve these yourself, first! It's fun to figure it out! Then you can take a
look here. :D

## Benchmarks

_Done using `hyperfine [binary] --warmup 10 -N`_ _Binaries were built using
`cargo build --release`_

Note that these benchmarks will change depending on your puzzle input and
hardware.

```sh
# Hardware
CPU: Intel Core i7-1165G7 @ 4.7 GHz
GPU: Integrated (Intel Iris Xe @ 1.3 GHz)
RAM: 16 GB Samsung DRAM @ 4267 MT/s
Disk: 1 TB SSD - btrfs
OS: CachyOS Linux
Kernel: 6.17.7-3-cachyos
Desktop: KDE Plasma 6.5.2 (wayland)
Shell: fish 4.2.0
```

```sh
# Problem 1 - Part 1
Benchmark 1: target/release/problem1_part1
  Time (mean ± σ):       2.3 ms ±   0.6 ms    [User: 0.9 ms, System: 1.0 ms]
  Range (min … max):     0.8 ms …  10.0 ms    1338 runs

# Problem 1 - Part 2
Benchmark 1: target/release/problem1_part2
  Time (mean ± σ):       3.7 ms ±   1.1 ms    [User: 2.3 ms, System: 1.0 ms]
  Range (min … max):     1.5 ms …   8.7 ms    869 runs

# Problem 2 - Part 1
Benchmark 1: target/release/problem2_part1
  Time (mean ± σ):      13.5 ms ±   3.4 ms    [User: 66.1 ms, System: 3.9 ms]
  Range (min … max):    10.0 ms …  40.2 ms    222 runs

# Problem 2 - Part 2
Benchmark 1: target/release/problem2_part2
  Time (mean ± σ):      74.7 ms ±   3.8 ms    [User: 474.0 ms, System: 5.3 ms]
  Range (min … max):    69.7 ms …  88.8 ms    41 runs
```
