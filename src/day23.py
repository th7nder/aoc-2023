
grid = open("input/23.txt").read().splitlines()

start = (0, grid[0].index("."))
end = (len(grid) - 1, grid[-1].index("."))


print(start, end)
points = [start, end]
for r, row in enumerate(grid):
    for c, ch in enumerate(row):
        if grid[r][c] == '#':
            continue

        nbhs = 0
        for nr, nc in [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)]:
            if 0 <= nr < len(grid) and 0 <= nc < len(grid[0]) and grid[nr][nc] != '#':
                nbhs += 1
        if nbhs >= 3:
            points.append((r, c)) 

graph = {pt: {} for pt in points}


def nbhs(r, c, symbol):
    s = {
        '>': [(r, c + 1)],
        '<': [(r, c - 1)],
        '^': [(r - 1, c)],
        'v': [(r + 1, c)],
        '.': [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)]
    }

    return [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)]

def dfs(sr, sc, r, c, grid, graph, visited, path):
    if (r, c) != (sr, sc) and (r, c) in graph:
        graph[(sr, sc)][(r, c)] = path
        return

    for nr, nc in nbhs(r, c, grid[r][c]):
        if 0 <= nr < len(grid) and 0 <= nc < len(grid[0]) and grid[nr][nc] != '#' and (nr, nc) not in visited:
            visited.add((nr, nc))
            dfs(sr, sc, nr, nc, grid, graph, visited, path + 1)



dirs = {
    '>': [(0, 1)],
    '<': [(0, -1)],
    '^': [(-1, 0)],
    'v': [(1, 0)],
    '.': [(1, 0), (-1, 0), (0, 1), (0, -1)]
}


for sr, sc in points:
    stack = [(0, sr, sc)]
    seen = {(sr, sc)}
    while stack:
        n, r, c = stack.pop()

        if n != 0 and (r, c) in graph:
            graph[(sr, sc)][(r, c)] = n
            continue

        for nr, nc in nbhs(r, c, grid[r][c]):
            if 0 <= nr < len(grid) and 0 <= nc < len(grid[0]) and grid[nr][nc] != '#' and (nr, nc) not in seen:
                seen.add((nr, nc))
                stack.append((n + 1, nr, nc))
            


print(graph)

seen = set()

def bruteforce(curr, end, graph, path):
    if curr == end:
        return path
    
    ans = 0
    seen.add(curr)
    for nbh in graph[curr]:
        if nbh not in seen:
         ans = max(ans, bruteforce(nbh, end, graph, path + graph[curr][nbh]))
    seen.remove(curr)
    return ans


print(bruteforce(start, end, graph, 0))
