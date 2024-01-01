import sympy


hailstones = [tuple(map(int, line.replace("@", ",").split(","))) for line in open("input/24.txt").read().splitlines()]

# 1 2 3 4 5
# 

xr, yr, zr, vxr, vyr, vzr = sympy.symbols("xr yr zr vxr vyr vzr")

equations = []

for i, hailstone in enumerate(hailstones[:50]):
    xh, yh, zh, vxh, vyh, vzh = hailstone

    equations.append((xr - xh) * (vyh - vyr) - (yr - yh) * (vxh - vxr))
    equations.append((yr - yh) * (vzh - vzr) - (zr - zh) * (vyh - vyr))


# print(equations)
answers = sympy.solve(equations)    
print(answers)
answers = answers[0]
# print(answers[xr], answers[yr], answers[zr], answers[vxr], answers[vyr], answers[vzr])

ans = answers[xr] + answers[yr] + answers[zr]
print("P2: ", ans)