

bricks = []
with open("input/22.txt") as f:
    for line in f:
        line = line.strip()
        if line:
          bricks.append(list(map(int, line.replace("~", ",").split(","))))

bricks.sort(key=lambda b: b[2])
# print(bricks)

def intersects(a, b):
    return max(a[0], b[0]) <= min(a[3], b[3]) and max(a[1], b[1]) <= min(a[4], b[4])

for index, brick in enumerate(bricks):
    max_z = 1

    for under_brick in bricks[:index]:
        if intersects(brick, under_brick):
            max_z = max(max_z, under_brick[5] + 1)
    
    brick[5] += max_z - brick[2]
    brick[2] = max_z


bricks.sort(key=lambda b: b[2])

for brick in bricks:
    assert brick[2] <= brick[5]


children = {}
parents = {}

for j, upper in enumerate(bricks):
    for i, lower in enumerate(bricks[:j]):
        if intersects(lower, upper) and upper[2] == lower[5] + 1:
            lower = tuple(lower)
            upper = tuple(upper)
            if upper not in children:
                children[upper] = []
            if lower not in parents:
                parents[lower] = []

            children[upper].append(lower)
            parents[lower].append(upper)

p1 = 0
for brick in bricks:
    brick = tuple(brick)
    if brick not in parents or len(parents[brick]) == 0:
        # print("no parents", brick)
        p1 += 1
        continue

    valid = True
    for parent in parents[brick]:
        # print(len(children[parent]))
        if len(children[parent]) == 1:
            valid = False
            break

    if valid:
        p1 += 1

from collections import deque
def determine(start_brick):
    if start_brick not in parents:
        return 0

    queue = deque(brick for brick in parents[start_brick] if len(children[brick]) == 1)
    falling = set(queue)
    falling.add(start_brick)

    while queue:
        brick = queue.popleft()
        # print("processing ", brick)
        if brick in parents:
            for parent in parents[brick]:
                f = True
                for child in children[parent]:
                    if child not in falling:
                        f = False
                        break
                if f and parent not in falling:
                    falling.add(parent)
                    queue.append(parent)

    return len(falling) - 1
# print(bricks)
# print(levels)
print(p1)
# print(bricks)

a = 0
for brick in bricks:
    a += determine(tuple(brick))

print("P2: ", a)
# print(determine((1, 0, 1, 1, 2, 1)))


# print(children[(0, 1, 4, 2, 1, 4)])
# 0, 4
# 2, 6
# 2 <= 4

# 4, 6
# 7, 8
# 7 <= 6 
# 4, 6
# 4, 5
# 4 <= 5