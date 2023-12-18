from collections import defaultdict, deque
from heapq import heappush, heappop

heatmap = []
with open("input/17_big.txt") as f:
    for line in f:
        line = line.strip()
        if line:
            heatmap.append([int(c) for c in line])


def print_heatmap(heatmap):
    for row in heatmap:
        for col in row:
            print(col, end='')
        print()

print_heatmap(heatmap)

RIGHT = (0, 1)
LEFT = (0, -1)
TOP = (-1, 0)
DOWN = (1, 0)

def add(a, b):
    return (a[0] + b[0], a[1] + b[1])

def add4(a, b):
    return (a[0] + b[0] * 4, a[1] + b[1] * 4)

rows = len(heatmap)
cols = len(heatmap[0])

def print_hh(heatmap, parents, signs, target):
    heatmap = [c[:] for c in heatmap]
    node = target
    while node != (0, 0):        
        print(node)
        if signs[node][0] == DOWN:
            heatmap[node[0]][node[1]] = 'v'
        elif signs[node][0] == TOP:
            heatmap[node[0]][node[1]] = '^'
        elif signs[node][0] == RIGHT:
            heatmap[node[0]][node[1]] = '>'
        else:
            heatmap[node[0]][node[1]] = '<'

        node = parents[node]
    print_heatmap(heatmap)
        
STRIKE_LIMIT = 10
def is_valid(new_pos):
    return new_pos[0] >= 0 and new_pos[0] < rows and new_pos[1] >= 0 and new_pos[1] < cols

def get_neighbours(pos, direction, strike):
    new_dirs = []

    if direction is None:
        new_dirs = [RIGHT, DOWN]
    else:
        if direction == RIGHT:
            new_dirs = [TOP, DOWN]
        elif direction == LEFT:
            new_dirs = [DOWN, TOP]
        elif direction == DOWN:
            new_dirs = [RIGHT, LEFT]
        elif direction == TOP:
            new_dirs = [LEFT, RIGHT]
        else:
            raise Exception("xd")

    if direction != None:
        if strike + 1 <= STRIKE_LIMIT:
            new_dirs.append(direction)

    neigbhours = []
    for new_dir in new_dirs:
        new_pos = add(pos, new_dir)
        if is_valid(new_pos):
            neigbhours.append((new_pos, new_dir, strike + 1 if new_dir == direction else 1, heatmap[new_pos[0]][new_pos[1]]))

    return neigbhours

def get_neighbours_v2(pos, direction, strike):
    new_dirs = []

    if direction is None:
        new_dirs = [RIGHT, DOWN]
    else:
        if direction == RIGHT:
            new_dirs = [TOP, DOWN]
        elif direction == LEFT:
            new_dirs = [DOWN, TOP]
        elif direction == DOWN:
            new_dirs = [RIGHT, LEFT]
        elif direction == TOP:
            new_dirs = [LEFT, RIGHT]
        else:
            raise Exception("xd")

    if direction != None:
        if strike + 1 <= STRIKE_LIMIT:
            new_dirs.append(direction)

    neigbhours = []
    for new_dir in new_dirs:
        if new_dir == direction:
            new_pos = add(pos, new_dir)
            if is_valid(new_pos):
                neigbhours.append((new_pos, new_dir, strike + 1, heatmap[new_pos[0]][new_pos[1]]))
        else:
            new_pos = add4(pos, new_dir)
            if is_valid(new_pos):
                cost = 0
                if new_dir == RIGHT:
                    for x in range(1, 4 + 1):
                        cost += heatmap[pos[0]][pos[1] + x]
                elif new_dir == LEFT: 
                    for x in range(1, 4 + 1):
                        cost += heatmap[pos[0]][pos[1] - x]
                elif new_dir == DOWN: 
                    for x in range(1, 4 + 1):
                        cost += heatmap[pos[0] + x][pos[1]]
                elif new_dir == TOP: 
                    for x in range(1, 4 + 1):
                        cost += heatmap[pos[0] - x][pos[1]]
                neigbhours.append((new_pos, new_dir, 4, cost))

    return neigbhours

def dijkstra(heatmap):
    start = ((0, 0), None, 0)

    queue = [(0, start)]
    distances = defaultdict(lambda: float('inf'))
    distances[start] = 0
    
    rows = len(heatmap)
    cols = len(heatmap[0])
    target = (rows - 1, cols - 1)

    while len(queue) > 0:
        distance, state = heappop(queue)
        node, prev_dir, strike = state

        if node == target:
            return distance

        for neighbor, next_dir, new_strike, cost in get_neighbours_v2(node, prev_dir, strike):
            nbh = (neighbor, next_dir, new_strike)
            d = distance + cost
            if d < distances[nbh]:
                distances[nbh] = d
                heappush(queue, (d, nbh))

    raise Exception("NO.")



# ans = dfs(heatmap, (0, 0), RIGHT, 0, set(), {})
# print(f"Part 1: {ans}")

end = (rows - 1, cols - 1)
# print_hh(heatmap, parents, signs, end)
ans = dijkstra(heatmap)
print("---------------")
# print_hh(heatmap, parents, signs, end)
print("Part 1: ", ans)
