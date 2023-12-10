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
    
    def range(self, left, right):
        # print("entries", self.entries)
        # [79, 92]
        # [50, 97] => [52, 99]
        # [98, 100] => [50, 52]

        intervals = []
        for [destination, source, r] in self.entries:
            start = source
            end = source + r

            # print(left, right, start, end)

            # [79, 92]
            # [50, 97] -> [52, 99]
            if left >= start and right <= end:
                new_left = destination + (left - start)
                new_right = destination + (right - start)
                intervals.append((new_left, new_right))
                break
            # [57, 69] -> [left, right]
            # [53, 60] -> [49, 56]
            # [start, end] -> [destination + l_offset, destination + right_offset]
            elif left >= start and left <= end:
                # DOESNT SKIP ANYTHING!
                # [81, 94]
                # [0, 15]
                new_left = destination + (left - start)
                new_right = destination + (end  - 1 - start )
                # intervals.append((left, end - 1, new_left, new_right))
                intervals.append((new_left, new_right))
                # create a new interval
                left = end
            # [63, 69] -> [left, right]
            # [65, 68] -> [87, 90]
            elif left <= end and right > end:
                intervals.append((left, start - 1))
                print("insert: ", left)
                print("c")
                new_left = destination + (start - start)
                new_right = destination + (end - start)
                # intervals.append((start, end, new_left, new_right))
                intervals.append((new_left, new_right))
                print("insert: ", new_left)
                left = end 
            # [63, 67] -> [left, right]
            # [65, 68] -> [87, 90]
            elif left <= end and right <= end and right >= start:
                print("d")
                # [0, 50]
                # [100, 300] -> [x, x]
                print("insert: ", left, start, end, left, right )
                intervals.append((left, start - 1))
                new_left = destination + (start - start)
                new_right = destination + (right - start)
                print("insert: ", new_left)
                # intervals.append((start, right, new_left, new_right))
                intervals.append((new_left, new_right))
                break
        if len(intervals) == 0:
            intervals.append((left, right))
        return intervals

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

def dest(seed):
    value = seed
    for map in maps:
        value = map.destination(value)
    return value

def part2():
    ans = []
    for i in range(0, len(seeds), 2):
        start = seeds[i]
        end = seeds[i] + seeds[i + 1]

        left = start
        right = end - 1
        intervals = [(left, right)]
        for map in maps:
            new_intervals = []
            for (left, right) in intervals:
                a = map.range(left, right)
                for x in a:
                    new_intervals.append(x)
            intervals = new_intervals
            print(intervals)

        # break
        for i in intervals:
            ans.append(i)

    print(min(ans))


part2()
# m = AlmanacMap([
#     [60, 56, 37],
#     # starts exactly where it should
#     [56, 93, 4],
# ])

# print(m.range(46, 56))