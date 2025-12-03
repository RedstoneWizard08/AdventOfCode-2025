#!/usr/bin/env python3

# Run individual benches for each crate.
# If you're trying to update the README, or collect overall stats, just run
# the benchmarks in the `benches` crate.

import json
import subprocess

cmd = [
    "cargo",
    "bench",
    "--no-default-features",
    "--features",
    "bench",
    "--no-run",
    "--message-format",
    "json"
]

data = subprocess.check_output(cmd)
data = data.decode("utf-8")
lines = data.strip().splitlines()
execs: dict[str, str] = {}

for line in lines:
    data = json.loads(line)

    if "target" not in data:
        continue

    item = data["target"]
    name = item["name"]

    if item["kind"][0] != "bench":
        continue

    executable = data["executable"]
    execs[name] = executable

execs = dict(sorted(execs.items()))

for (name, bin) in execs.items():
    subprocess.run([bin, "--bench"])
