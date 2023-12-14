rocks = []

with open("input/14.txt") as f:
    for line in f:
        l = line.strip()
        if l:
            rocks.append([rock for rock in l])


def print_rocks(rocks):
    for row in rocks:
        for rock in row:
            print(rock, end='')
        print()

    print()


def load(rocks):
    rows = len(rocks)

    # O(n^2)
    ans = 0
    for (row_idx, row) in enumerate(rocks):
        for (col_idx, rock) in enumerate(row):
            if rocks[row_idx][col_idx] == 'O':
                ans += (rows - row_idx)
    return ans 

def load_hash(rocks):
    rows = len(rocks)

    # O(n^2)
    ans = 0
    for (row_idx, row) in enumerate(rocks):
        for (col_idx, rock) in enumerate(row):
            if rocks[row_idx][col_idx] == 'O':
                ans += (rows - row_idx) + col_idx
    return ans 

def tilt_north(rocks):
    rows = len(rocks)
    cols = len(rocks[0])

    # O(n^2)
    for row_idx in range(rows):
        for col_idx in range(cols):
            if rocks[row_idx][col_idx] == 'O':
                new_row_idx = row_idx
                while new_row_idx - 1 >= 0 and rocks[new_row_idx - 1][col_idx] == '.':
                    new_row_idx -= 1

                if new_row_idx >= 0 and rocks[new_row_idx][col_idx] == '.':
                    rocks[row_idx][col_idx] = '.'
                    rocks[new_row_idx][col_idx] = 'O'


def tilt_south(rocks):
    rows = len(rocks)
    cols = len(rocks[0])

    # O(n^2)
    for row_idx in reversed(range(rows)):
        for col_idx in range(cols):
            if rocks[row_idx][col_idx] == 'O':
                new_row_idx = row_idx
                while new_row_idx + 1 < rows and rocks[new_row_idx + 1][col_idx] == '.':
                    new_row_idx += 1

                if new_row_idx < rows and rocks[new_row_idx][col_idx] == '.':
                    rocks[row_idx][col_idx] = '.'
                    rocks[new_row_idx][col_idx] = 'O'


def tilt_west(rocks):
    rows = len(rocks)
    cols = len(rocks[0])

    # O(n^2)
    for col_idx in range(cols):
        for row_idx in range(rows):
            if rocks[row_idx][col_idx] == 'O':
                # print(f"moving: {row_idx + 1} {col_idx + 2}")
                new_col_idx = col_idx
                while new_col_idx - 1 >= 0 and rocks[row_idx][new_col_idx - 1] == '.':
                    new_col_idx -= 1

                # print(f"new_col_idx = {new_col_idx + 2}")
                if new_col_idx >= 0 and rocks[row_idx][new_col_idx] == '.':
                    rocks[row_idx][col_idx] = '.'
                    rocks[row_idx][new_col_idx] = 'O'

def tilt_east(rocks):
    rows = len(rocks)
    cols = len(rocks[0])

    # O(n^2)
    for col_idx in reversed(range(cols)):
        for row_idx in range(rows):
            if rocks[row_idx][col_idx] == 'O':
                # print(f"moving: {row_idx + 1} {col_idx + 2}")
                new_col_idx = col_idx
                while new_col_idx + 1 < cols and rocks[row_idx][new_col_idx + 1] == '.':
                    new_col_idx += 1

                # print(f"new_col_idx = {new_col_idx + 2}")
                if new_col_idx < cols and rocks[row_idx][new_col_idx] == '.':
                    rocks[row_idx][col_idx] = '.'
                    rocks[row_idx][new_col_idx] = 'O'

print_rocks(rocks)


def cycle(rocks):
    tilt_north(rocks)
    tilt_west(rocks)
    tilt_south(rocks)
    tilt_east(rocks)


last_cycle = {}
last_cycle_difference = {}


S = None
P = None
for i in range(1000):
    # if i % 100000 == 0:
    #     print(i)
    cycle(rocks)
    value = load_hash(rocks)
    cycle_id = i + 1

    print(cycle_id, " ", value)
    
    if value in last_cycle:
        if value in last_cycle_difference:
            if last_cycle_difference[value] == cycle_id - last_cycle[value]:
                P = cycle_id - last_cycle[value]
                S = last_cycle[value]
                print("FOUND CYCLE ", value, last_cycle_difference[value])
                break
        last_cycle_difference[value] = cycle_id - last_cycle[value]
    last_cycle[value] = cycle_id

    
X = 1000000000
a = (X - S) % P
print("Position in cycle ", (X - S) % P)


for i in range(a):
    cycle(rocks)
    print(load(rocks))




# print(load(rocks))