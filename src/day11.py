

def read_map():
    galaxy_map = []
    to_expand_rows = set()
    
    galaxies = []
    row_idx = 0
    with open("input/11.txt") as f:
        for line in f:
            line = line.strip()
            if line:
                line = [c for c in line]
                galaxy_map.append(line)

                expand = True
                for (idx, c) in enumerate(line):
                    if c != '.':
                        expand = False 

                if expand:
                    # galaxy_map.append(line)
                    to_expand_rows.add(row_idx)
                row_idx += 1


    to_expand_cols = set()
    for c in range(len(galaxy_map[0])):
        expand = True
        for r in range(len(galaxy_map)):
            if galaxy_map[r][c] != '.':
                expand = False
        if expand:
            to_expand_cols.add(c) 

    # for col in reversed(to_expand_cols):
    #     for row in range(len(galaxy_map)):
    #         galaxy_map[row].insert(col + 1, '.')

    for row in range(len(galaxy_map)):
        for col in range(len(galaxy_map[0])):
            if galaxy_map[row][col] == '#':
                galaxies.append((row, col))

    return galaxy_map, galaxies, to_expand_rows, to_expand_cols

galaxy_map, galaxies, big_r, big_c = read_map()

for r in range(len(galaxy_map)):
    for c in range(len(galaxy_map[0])):
        print(f"{galaxy_map[r][c]}", end='')
    print("")



from collections import deque


distances = {}
ids = {}
gid = 1
for galaxy in galaxies:
    ids[galaxy] = gid
    gid += 1


MULTIPLIER = 1000000
# print(galaxies)

for start in galaxies:
    queue = deque([start])
    distance = 0
    visited = set(start)

    parent = {start: 0}

    # print(f"start {start[0] + 1} {start[1] + 2}")

    while queue:
        nodes = len(queue)
        for _ in range(nodes):
            (r, c) = queue.popleft()
            if galaxy_map[r][c] == '#':
                # print(f"distance to: {r + 1} {c + 2} = {distance} -> parent? {parent[(r, c)]}")
                a = ids[start]
                b = ids[(r, c)]
                if a < b:
                    distances[(a, b)] = parent[(r, c)]
                elif b > a:
                    distances[(b, a)] = parent[(r, c)]

            for (dr, dc) in [(0, 1), (1, 0), (0, -1), (-1, 0)]:
                new_r, new_c = (r + dr, c + dc)
                if 0 <= new_r < len(galaxy_map) and 0 <= new_c < len(galaxy_map[0]):
                    if (new_r, new_c) not in visited:
                        visited.add((new_r, new_c))
                        queue.append((new_r, new_c))
                        cost = 1
                        if new_r != r and new_r in big_r:  
                            cost += (MULTIPLIER - 1)
                        if new_c != c and new_c in big_c:
                            cost += (MULTIPLIER - 1)

                        parent[(new_r, new_c)] = parent[(r, c)] + cost




        distance += 1


ans = 0
for key in distances:
    ans += distances[key]

print(ans)

print(len(distances))