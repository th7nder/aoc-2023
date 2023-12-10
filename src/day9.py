

def predict(starting_sequence):
    matrix = [starting_sequence]
    
    sequence = starting_sequence
    while True:
        differences = []
        for i in range(1, len(sequence)):
            differences.append(sequence[i] - sequence[i - 1])

        sequence = differences
        matrix.append(sequence)

        good = True
        for num in sequence:
            if num != 0:
                good = False
        if good:
            break
    
    last_element = 0
    for i in reversed(range(len(matrix) - 1)):
        new_element = last_element + matrix[i][-1]
        last_element = new_element
        # print(last_element)


    # print("predicted: ", last_element)
    return last_element
    # print(matrix)

def predict2(starting_sequence):
    matrix = [starting_sequence]
    
    sequence = starting_sequence
    while True:
        differences = []
        for i in range(1, len(sequence)):
            differences.append(sequence[i] - sequence[i - 1])

        sequence = differences
        matrix.append(sequence)

        good = True
        for num in sequence:
            if num != 0:
                good = False
        if good:
            break
    
    bottom_element = 0
    for i in reversed(range(len(matrix) - 1)):
        new_element = matrix[i][0] - bottom_element
        bottom_element = new_element
        # print(bottom_element)


    # print("predicted: ", last_element)
    return bottom_element

sequences = []

with open("input/9.txt") as f:
    for line in f:
        sequences.append([int(k) for k in line.strip().split(" ")])

ans = 0
for sequence in sequences:
    # print(sequence)
    ans += predict2(sequence)

print("Part2: ", ans)