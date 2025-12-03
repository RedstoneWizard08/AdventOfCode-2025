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
problem 1 - part 1      time:   [184.25 µs 186.28 µs 188.59 µs]
                        change: [−4.9045% −1.8701% +0.9031%] (p = 0.23 > 0.05)
                        No change in performance detected.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

problem 1 - part 2      time:   [182.11 µs 183.55 µs 185.13 µs]
                        change: [−2.6381% −0.7939% +0.9747%] (p = 0.40 > 0.05)
                        No change in performance detected.
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild

problem 2 - part 1      time:   [5.9707 ms 6.1142 ms 6.2692 ms]
                        change: [+4.8244% +11.335% +17.450%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe

problem 2 - part 2      time:   [21.433 ms 22.307 ms 23.429 ms]
                        change: [−13.632% −8.2239% −2.1797%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) high mild
  5 (5.00%) high severe

problem 3 - part 1      time:   [113.84 µs 115.36 µs 117.19 µs]
                        change: [+6.2286% +9.3676% +12.426%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe

problem 3 - part 2      time:   [182.72 µs 191.42 µs 200.75 µs]
                        change: [−4.0542% −1.4368% +1.4042%] (p = 0.32 > 0.05)
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  2 (2.00%) high mild
  7 (7.00%) high severe
```
