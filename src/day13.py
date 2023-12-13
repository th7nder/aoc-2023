

patterns = []

with open("input/13.txt") as f:
    pattern = []
    for line in f:
        l = line.strip()
        if not l and len(pattern) > 0:
            patterns.append(pattern)
            pattern = []
        elif l:
            pattern.append([c for c in l])

    if len(pattern) > 0:
        patterns.append(pattern)

def print_pattern(pattern):
    for row in pattern:
        for c in row:
            print(c, end='')
        print()
    print()

# for pattern in patterns:
#     print_pattern(pattern)


def find_reflection(pattern):
    rows = len(pattern)
    cols = len(pattern[0])

    row_sum = 0
    for row_idx in range(rows):
        # print("Checking ", row_idx, rows)

        all_columns = True
        for col_idx in range(cols):
            up_idx = row_idx - 1
            down_idx = row_idx
            equal = True
            while up_idx >= 0 and down_idx < rows:
                if pattern[up_idx][col_idx] != pattern[down_idx][col_idx]:
                    equal = False
                    break
                up_idx -= 1
                down_idx += 1

            if not equal:
                all_columns = False
                break

        # if all_columns and (up_idx == 0 and down_idx == rows) or (up_idx == -1 and down_idx == rows - 1):
        if all_columns:
            # print("FOUND A BREAKING POINT: ", row_idx, up_idx, down_idx, rows)
            row_sum += row_idx * 100
            # return row_idx * 100


    col_sum = 0
    for col_idx in range(cols):
        # print("Col checking ", col_idx, cols)

        all_rows = True
        for row_idx in range(rows):
            left_idx = col_idx - 1
            right_idx = col_idx
            equal = True 
            while left_idx >= 0 and right_idx < cols:
                if pattern[row_idx][left_idx] != pattern[row_idx][right_idx]:
                    equal = False
                    break
                left_idx -= 1
                right_idx += 1
            
            if not equal:
                all_rows = False
                break
        
        # if all_rows and (left_idx == 0 and right_idx == cols) or (left_idx == -1 and right_idx == cols - 1):
        if all_rows:
            # print("COLUMN BREAKING POINT: ", col_idx, left_idx, right_idx, cols)
            col_sum += col_idx


    # print(col_sum, row_sum)
    return col_sum + row_sum

ans = 0
for idx, pattern in enumerate(patterns):
    # print(idx)
    ans += find_reflection(pattern)

print("part1: ", ans)
