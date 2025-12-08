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

_Done using the benchmark in the `benches` crate (`cargo bench -p benches`)._

Note that these benchmarks will change depending on your puzzle input and
hardware.

NOTE! The benchmarks on the GitHub Pages viewer were performed on GitHub
Actions' hardware (which I do not know), and results differ from my laptop. This
README is the best place to find good numbers on consumer hardware.

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
problem 1 - part 1      time:   [161.99 µs 167.77 µs 173.59 µs]
problem 1 - part 2      time:   [183.58 µs 192.25 µs 202.20 µs]

problem 2 - part 1      time:   [4.0160 ms 4.0646 ms 4.1149 ms]
problem 2 - part 2      time:   [16.455 ms 16.608 ms 16.792 ms]

problem 3 - part 1      time:   [110.41 µs 113.50 µs 116.48 µs]
problem 3 - part 2      time:   [170.25 µs 174.57 µs 179.35 µs]

problem 4 - part 1      time:   [1.8690 µs 1.8759 µs 1.8838 µs]
problem 4 - part 2      time:   [2.4132 ms 2.4225 ms 2.4318 ms]

problem 5 - part 1      time:   [52.296 µs 52.543 µs 52.798 µs]
problem 5 - part 2      time:   [22.457 µs 22.969 µs 23.709 µs]

problem 6 - part 1      time:   [15.054 µs 15.510 µs 16.077 µs]
problem 6 - part 2      time:   [38.603 µs 39.732 µs 41.121 µs]
```
