times = [46807866]
distances = [ 214117714021024]

import math

ans = 1
for idx in range(len(times)):
    max_time = times[idx]
    min_distance = distances[idx]


    delta = math.sqrt(max_time ** 2 - 4 * min_distance)
    x1 = (max_time - delta) / 2
    x2 = (delta + max_time) / 2

    ways = math.floor(x2 - x1)
    print(f"Number of ways: {ways}")

# print(f"Part1: {ans}")