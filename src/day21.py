
garden = []

with open("input/21_small.txt") as f:
    for line in f:
        line = line.strip()
        if line:
            garden.append([t for t in line])


def print_garden(garden, visited=set()):
    for r, row in enumerate(garden):
        for c, tile in enumerate(row):
            if (r, c) in visited:
                print('O', end='')
            else:
                print(tile, end='')
        print()



RIGHT = (0, 1)
LEFT = (0, -1)
TOP = (-1, 0)
DOWN = (1, 0)

def add(a, b):
    return (a[0] + b[0], a[1] + b[1])

def preprocess(garden):
    rows = len(garden)
    cols = len(garden[0])

    for r in range(rows):
        for c in range(cols):
            if garden[r][c] == 'S':
                return (rows, cols, (r, c))
            
    raise Exception("did not find start")

from collections import deque


print_garden(garden, visited=set())

def part1(garden, max_steps=6):
    rows, cols, start = preprocess(garden)
    def is_valid(new_pos):
     return new_pos[0] >= 0 and new_pos[0] < rows and new_pos[1] >= 0 and new_pos[1] < cols


    visited = {
        start: 0
    }
    queue = deque([start])

    level = 0
    while queue and level < max_steps:
        nodes = len(queue)
        print(nodes, level)
        for _ in range(nodes):
            curr = queue.popleft()

            for dir in [TOP, DOWN, RIGHT, LEFT]:
                new_pos = add(curr, dir)
                (new_r, new_c) = new_pos

                if not is_valid(new_pos):
                    continue
                if garden[new_r][new_c] == '#':
                    continue
                
                if level not in visited:
                    visited[level] = set() 
                visited[level].add(new_pos)
                queue.append((new_pos))
        level += 1

    # for l in range(level):
    #     print("Plots at level: ", l, " = ", len(visited[l]))
    #     print_garden(garden, visited[l])


    return len(visited[level - 1])

visited = part1(garden, 6)
print("Part 1: ", visited)


