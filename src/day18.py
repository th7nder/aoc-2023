import sys
sys.setrecursionlimit(9999999)

edges = []

with open("input/18.txt") as f:
    for line in f:
        line = line.strip()
        if line:
            dir, length, color = line.split(" ")
            edges.append((dir, int(length), color[1:-1]))

# print(edges)

def add(a, b):
    return (a[0] + b[0], a[1] + b[1])

def times(a, b):
    return (a[0] * b, a[1] * b)

dirs = {
    'R': (0, 1),
    'D': (1, 0),
    'L': (0, -1),
    'U': (-1, 0)
}

def build_map(edges):
    map = {}
    current = (0, 0)

    min_row = 0
    min_col = 0
    max_row = 0
    max_col = 0

    for (dir, length, _color) in edges:
        start = current
        end = add(start, times(dirs[dir], length))

        s = start
        while s != end:
            map[s] = '#'
            s = add(s, dirs[dir])

        max_row = max(max_row, end[0])
        max_col = max(max_col, end[1])
        min_row = min(min_row, end[0])
        min_col = min(min_col, end[1])

        current = end

    return map, min_row, max_row + 1, min_col, max_col + 1

map, min_row, rows, min_col, cols = build_map(edges)

def is_inside_polygon(map, min_col, point):
    # TODO: BUGGY!
    (row, col) = point

    count = 0
    while col >= min_col:
        if (row, col) in map:
            if (row, col + 1) not in map:
                count += 1 
            elif (row, col - 1) not in map:
                count += 1
        col -= 1

    return count % 2 != 0

def print_map(map, min_row, rows, min_col, cols):
    for row in range(min_row, rows):
        for col in range(min_col, cols):
            if (row, col) in map:
                print('#', end='')
            elif is_inside_polygon(map, min_col, (row, col)):
                print('0', end='')
            else:
                print('.', end='')
        print()

def flood_fill(map, min_row, rows, min_col, cols, pos):
    def valid(pos):
        row, col = pos
        if row < min_row or row >= rows or col < min_col or col >= cols:
            return False
        return True
    
    # print("Filling: ", pos[0] + 1, pos[1] + 2)

    map[pos] = 'X'
    # print_map(map, min_row, rows, min_col, cols)
    # print("-----------")

    for dir in ['R', 'L', 'U', 'D']:
        next_pos = add(pos, dirs[dir])
        if not valid(next_pos):
            continue
        if next_pos in map:
            continue

        # print("Calling: ", next_pos[0] + 1, next_pos[1] + 2, " from ", pos[0] + 1, pos[1] + 2)

        flood_fill(map, min_row, rows, min_col, cols, next_pos)
    

def build_coordinates(edges):
    current = (0, 0)

    coordinates = [(current)]
    border = 1
    for (dir, length, _color) in edges:
        start = current
        end = add(start, times(dirs[dir], length))
        border += length
        current = end
        coordinates.append(current)

    return coordinates, border


def build_coordinates_v2(edges):
    current = (0, 0)

    coordinates = [(current)]
    border = 1
    for (_, _, color) in edges:
        tr = ['R', 'D', 'L', 'U']
        print(color)
        dir = tr[int(color[-1])]
        length = int(color[1:-1], 16)

        start = current
        end = add(start, times(dirs[dir], length))
        border += length
        current = end
        coordinates.append(current)

    return coordinates, border

coords, border = build_coordinates(edges)

area = 0
for i in range(1, len(coords)):
    x1, y1 = coords[i - 1]
    x2, y2 = coords[i]
    area += (y1 + y2) * (x1 - x2)
area /= 2

# A -> shoelace formula

# pick's theorem 
# A = i + b/2 +1
# i = A - b/2 + 1
# 

print(abs(area) + (border // 2) + 1)

# print_map(map, min_row, rows, min_col, cols)

# print("FLOOOD FILL!")
# def start(map, min_row, rows, min_col, cols):
#     for row in range(min_row, rows):
#         for col in range(min_col, cols):
#             if (row, col) not in map and is_inside_polygon(map, min_col, (row, col)):
#                 flood_fill(map, min_row, rows, min_col, cols, (row, col))
#                 return


# start(map, min_row, rows, min_col, cols)

# print_map(map, min_row, rows, min_col, cols)

# print("Part 1", len(map))
        
