
garden = []

with open("input/21.txt") as f:
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


# print_garden(garden, visited=set())

def fill(start, max_steps=6):
    rows = len(garden)
    cols = len(garden[0])
    def is_valid(new_pos):
     return new_pos[0] >= 0 and new_pos[0] < rows and new_pos[1] >= 0 and new_pos[1] < cols


    visited = set((start))
    queue = deque([(start, max_steps)])
    ans = set()

    while queue:
        curr, s = queue.popleft()

        if s % 2 == 0:
            ans.add(curr)

        if s == 0:
            continue

        for dir in [TOP, DOWN, RIGHT, LEFT]:
            new_pos = add(curr, dir)
            (new_r, new_c) = new_pos

            if not is_valid(new_pos):
                continue

            if garden[new_r][new_c] == '#':
                continue
            
            if (new_pos) in visited:
                continue

            visited.add((new_pos))
            queue.append((new_pos, s - 1))

    return len(ans)

# print("Part 1: ", len(fill(garden, 64)))

steps = 26501365

rows, cols, (sr, sc) = preprocess(garden)
assert rows == cols
assert sr == rows // 2
assert sc == cols // 2

size = rows

grid_width = (steps - sr) // size - 1


odd = (grid_width // 2 * 2 + 1) ** 2
even = ((grid_width + 1) // 2 * 2) ** 2

odd_points = fill((sr, sc), size * 2 + 1)
even_points = fill((sr, sc), size * 2)

top = fill((size - 1, sc), size - 1)
right = fill((sr, 0), size - 1)
bottom = fill((0, sc), size - 1)
left = fill((sr, size - 1), size - 1)

small_tr = fill((size - 1, 0), size // 2 - 1)
small_tl = fill((size - 1, size - 1), size // 2 - 1)

small_br = fill((0, 0), size // 2 - 1)
small_bl = fill((0, size - 1), size // 2 - 1 )


big_tr = fill((size - 1, 0), size // 2 - 1 + size)
big_tl = fill((size - 1, size - 1), size // 2 - 1 + size)
big_br = fill((0, 0), size // 2 - 1 + size)
big_bl = fill((0, size - 1), size // 2 - 1 + size)

# 2 // 2 * 2 + 1 ** 2

ans = (
    odd * odd_points 
    + even * even_points
    + right + left + bottom + top
    + (grid_width + 1) * (small_tl + small_tr + small_bl + small_br)
    + (grid_width) * (big_tl + big_tr + big_bl + big_br)
)

print(ans)

