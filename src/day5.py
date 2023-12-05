import math 

class AlmanacMap:
    def __init__(self, entries):
        self.entries = entries
        self.entries.sort(key=lambda k: k[1])

    def destination(self, value):
        for [destination, source, r] in self.entries:
            if value < source:
                return value
            
            if value >= source and value < source + r:
                return destination + (value - source)

        return value
    


seeds = []
maps = []

with open("input/5.txt") as f:
    current_entries = []

    for line in f:
        line = str(line).strip()
        if line.find("seeds: ") != -1:
            seeds = [int(num) for num in line.split("seeds: ")[1].split(" ")]
        elif line.find("map:") != -1:
            current_entries = list()
        elif len(current_entries) > 0 and line == "":
            maps.append(AlmanacMap(current_entries))
        elif line != "":
            current_entries.append([int(num) for num in line.split(" ")])

# print(am.destination(101))
# ans = []
# for seed in seeds:
#     value = seed
#     for map in maps:
#         value = map.destination(value)
#     ans.append((value, seed))

# ans.sort()
# print(f"part1: {ans[0][0]}")
# print(seeds)

def dest(seed):
    value = seed
    for map in maps:
        value = map.destination(value)
    return value
    
ans = None
for i in range(0, len(seeds), 2):
    start = seeds[i]
    end = seeds[i] + seeds[i + 1]

    # left = start
    # right = end - 1
    # minimum = dest(left)
    # while left <= right:
    #     mid = left + (right - left) // 2
    #     v = dest(mid)
    #     minimum = min(v, minimum)
    #     if v >= dest(left) and v < dest(right):
    #         left = mid + 1
    #     else:
    #         right = mid - 1

    # print(f"calc {minimum}")
    for seed in range(start, end):
        location = dest(seed)
        if ans is None:
            ans = (location, seed)
        else:
            ans = min(ans, (location, seed))
        print(seed, location)
    print("NEW PAIR")
print(ans)