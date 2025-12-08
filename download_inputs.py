import os
import requests

token = os.environ["AOC_SESSION"]
sess = requests.Session()
sess.cookies["session"] = token

days = [1, 2, 3, 4, 5, 6, 7]

for day in days:
    url = f"https://adventofcode.com/2025/day/{day}/input"
    data = sess.get(url).text

    with open(f"problem{day}/input.txt", "w") as f:
        f.write(data)
