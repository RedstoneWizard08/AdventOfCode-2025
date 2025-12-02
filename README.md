# Advent of Code 2025

These are my answers for the
[Advent of Code 2025](https://adventofcode.com/2025)!

Go solve these yourself, first! It's fun to figure it out! Then you can take a
look here. :D

## Competition

I'm half-competing with [@rhysdh540](https://github.com/rhysdh540) to build the
fastest solutions (he's using Kotlin, I'm using Rust). It's a friendly
competition, nothing at stake, just for fun. :)

Go check out his solutions here: https://github.com/rhysdh540/AdventOfCode

## Benchmarks

_Done using `hyperfine [binary] --warmup 10 -N`_\
_Binaries were built using `cargo build --release`_

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
  Time (mean ± σ):       1.9 ms ±   0.7 ms    [User: 0.7 ms, System: 0.9 ms]
  Range (min … max):     0.7 ms …   8.1 ms    1493 runs

# Problem 1 - Part 2
Benchmark 1: target/release/problem1_part2
  Time (mean ± σ):       1.2 ms ±   0.2 ms    [User: 0.8 ms, System: 0.3 ms]
  Range (min … max):     1.0 ms …   2.8 ms    1990 runs

# Problem 2 - Part 1
Benchmark 1: target/release/problem2_part1
  Time (mean ± σ):      13.5 ms ±   3.4 ms    [User: 66.1 ms, System: 3.9 ms]
  Range (min … max):    10.0 ms …  40.2 ms    222 runs

# Problem 2 - Part 2
Benchmark 1: target/release/problem2_part2
  Time (mean ± σ):      74.7 ms ±   3.8 ms    [User: 474.0 ms, System: 5.3 ms]
  Range (min … max):    69.7 ms …  88.8 ms    41 runs
```
