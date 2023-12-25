
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


    visited = set((start, 0, 0))
    queue = deque([(start, max_steps, 0, 0)])
    haha = set()

    while queue:
        curr, s, rm, cm = queue.popleft()

        if s % 2 == 0:
            haha.add((curr, rm, cm))

        if s == 0:
            continue
        for dir in [TOP, DOWN, RIGHT, LEFT]:
            new_pos = add(curr, dir)
            
            # part 2
            if new_pos[0] >= rows:
                new_pos = (0, new_pos[1])
                rm += 1
            elif new_pos[0] < 0:
                new_pos = (rows - 1, new_pos[1])
                rm -= 1
            
            if new_pos[1] >= cols:
                new_pos = (new_pos[0], 0)
                cm += 1
            elif new_pos[1] < 0:
                new_pos = (new_pos[0], cols - 1)
                cm -= 1

            (new_r, new_c) = new_pos

            # if not is_valid(new_pos):
                # continue
            if garden[new_r][new_c] == '#':
                continue
            
            if (new_pos, rm, cm) in visited:
                continue

            visited.add((new_pos, rm, cm))
            queue.append((new_pos, s - 1, rm, cm))

    return haha

visited = part1(garden, 10)
# print_garden(garden, visited)
print(len(visited))

