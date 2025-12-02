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

_Done using `hyperfine [binary] --warmup 100 -N`_\
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
  Time (mean ± σ):     625.4 µs ± 107.0 µs    [User: 370.3 µs, System: 169.8 µs]
  Range (min … max):   493.4 µs … 2323.8 µs    5350 runs

# Problem 1 - Part 2
Benchmark 1: target/release/problem1_part2
  Time (mean ± σ):     980.6 µs ± 224.3 µs    [User: 689.1 µs, System: 523.8 µs]
  Range (min … max):   632.3 µs … 2699.7 µs    1383 runs

# Problem 2 - Part 1
Benchmark 1: target/release/problem2_part1
  Time (mean ± σ):       9.9 ms ±   0.8 ms    [User: 56.5 ms, System: 2.6 ms]
  Range (min … max):     8.1 ms …  13.7 ms    291 runs

# Problem 2 - Part 2
Benchmark 1: target/release/problem2_part2
  Time (mean ± σ):      55.6 ms ±   1.8 ms    [User: 377.0 ms, System: 3.6 ms]
  Range (min … max):    53.0 ms …  62.7 ms    53 runs
```
