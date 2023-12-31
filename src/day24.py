
projectiles = [[int(num.strip()) for num in line.replace("@", ",").split(",")] for line in open("input/24.txt").read().splitlines()]

# print(projectiles)

cast_distance = 400000000000000
bound_low = 200000000000000
bound_high = 400000000000000

lines = {}

for i, projectile in enumerate(projectiles):
    px, py, _, vx, vy, _ = projectile

    xx, yy = px + (cast_distance * vx), py + (cast_distance * vy)

    a = (yy - py) / (xx - px)
    b = (xx * py - px * yy) / (xx - px)
    print(px, py, xx, yy, a, b)
    lines[i] = (a, b, xx, yy)





ans = set()
for i, p1 in enumerate(projectiles):
    for j, p2 in enumerate(projectiles):
        if i == j:
            continue
        
        # print("XXXXXXXX", p1, p2)
        a1, c1 = lines[i][0], lines[i][1]
        a2, c2 = lines[j][0], lines[j][1]

        if a1 == a2:
            # print("parallel", p1, p2)
            continue
        # print(a1, c1, "xx", a2, c2)

        xa, ya = p1[0], p1[1]
        xx, yy = lines[i][2], lines[i][3]

        xs, ys = p2[0], p2[1]
        xt, yt = lines[j][2], lines[j][3] 

        x = (c2 - c1) / (a1 - a2)
        y = -(c1 * a2 - c2 * a1) / (a1 - a2)
        if bound_low <= x <= bound_high and bound_low <= y <= bound_high:
            if min(xa, xx) <= x <= max(xa, xx) and min(xs, xt) <= x <= max(xs, xt):
                print(x, y)
                ans.add((min(i, j), max(i, j)))
            else:
                # print("past!!!", x, y)
                pass
        else:
            # print("outside test area", x, y)
            pass
            
        # break
    # break 

print(len(ans))