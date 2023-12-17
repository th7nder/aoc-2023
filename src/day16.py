import sys

grid = []

with open("input/16.txt") as f:
    for line in f:
        line = line.strip()
        if line:
            grid.append([t for t in line])


def print_grid(grid, energized=set()):
    for (row_idx, row) in enumerate(grid):
        for (col_idx, col) in enumerate(row):
            if (row_idx, col_idx) in energized:
                print('#', end='')
            else:
                print(col, end='')
        print()

RIGHT = (0, 1)
LEFT = (0, -1)
TOP = (-1, 0)
DOWN = (1, 0)


energized = set()

def add(a, b):
    return (a[0] + b[0], a[1] + b[1])

sys.setrecursionlimit(99999999)

def beamify(grid, pos, dir, visited):
    rows = len(grid)
    cols = len(grid[0])
    x, y = pos
    if x < 0 or x >= rows or y < 0 or y >= cols:
        return
    
    # print_grid(grid, energized)
    # print("---------")

    energized.add(pos)
    if ((pos, dir)) in visited:
        return
    
    visited.add((pos, dir))

    current_tile = grid[x][y]

    if current_tile == '.':
        beamify(grid, add(pos, dir), dir, visited)
    elif current_tile == '/':
        if dir == RIGHT:
            beamify(grid, add(pos, TOP), TOP, visited)
        elif dir == LEFT:
            beamify(grid, add(pos, DOWN), DOWN, visited)
        elif dir == TOP:
            beamify(grid, add(pos, RIGHT), RIGHT, visited)
        elif dir == DOWN:
            beamify(grid, add(pos, LEFT), LEFT, visited)
        else:
            raise Exception("should not happen")
    elif current_tile == '\\':
        if dir == RIGHT:
            beamify(grid, add(pos, DOWN), DOWN, visited)
        elif dir == LEFT:
            beamify(grid, add(pos, TOP), TOP, visited)
        elif dir == TOP:
            beamify(grid, add(pos, LEFT), LEFT, visited)
        elif dir == DOWN:
            beamify(grid, add(pos, RIGHT), RIGHT, visited)
        else:
            raise Exception("should not happen")
    elif current_tile == '-':
        if dir == RIGHT or dir == LEFT:
            beamify(grid, add(pos, dir), dir, visited)
        else:
            beamify(grid, add(pos, LEFT), LEFT, visited)
            beamify(grid, add(pos, RIGHT), RIGHT, visited)
    elif current_tile == '|':
        if dir == TOP or dir == DOWN:
            beamify(grid, add(pos, dir), dir, visited)
        else:
            beamify(grid, add(pos, TOP), TOP, visited)
            beamify(grid, add(pos, DOWN), DOWN, visited)


rows = len(grid)
cols = len(grid[0])

ans = 0

for row in range(rows):
    energized.clear()
    beamify(grid, (row, 0), RIGHT, set())
    v = len(energized)
    ans = max(ans, v)
    print(v)

for row in range(rows):
    energized.clear()
    beamify(grid, (row, cols - 1), LEFT, set())
    v = len(energized)
    ans = max(ans, v)
    print(v)

for col in range(cols):
    energized.clear()
    beamify(grid, (0, col), DOWN, set())
    v = len(energized)
    ans = max(ans, v)
    print(v)

for col in range(cols):
    energized.clear()
    beamify(grid, (rows - 1, col), TOP, set())
    v = len(energized)
    ans = max(ans, v)
    print(v)

print(f"Ans: {ans}")

# print_grid(grid, energized=energized)
# print(f"part 1: {len(energized)}")