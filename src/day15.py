

sequence = []

with open("input/15.txt") as f:
    for line in f:
        if line:
            sequence = [step for step in line.strip().split(',')]

print(sequence)

def HASH(step):
    val = 0

    for c in step:
        val = (val + ord(c)) % 256
        val = (val * 17) % 256

    return val

ans = 0
for step in sequence:
    ans += HASH(step)

print(f"Part1: {ans}")


parsed_sequence = []
for step in sequence:
    if step.find("-") != -1:
        pos = step.find("-")
        lens_name = step[:pos]
        parsed_sequence.append(('-', lens_name))
    elif step.find("=") != -1:
        pos = step.find("=")
        lens_name = step[:pos]
        focal_length = step[pos + 1:]
        parsed_sequence.append(('=', lens_name, int(focal_length)))

sequence = parsed_sequence

from collections import deque

buckets = [deque() for _ in range(256)]
for step in sequence:
    op = step[0]
    lens_name = step[1]
    target_bucket = HASH(lens_name)
    bucket = buckets[target_bucket]
    if op == '=':
        new_value = step[2]
        replaced = False
        for idx in range(len(bucket)):
            lens, value = bucket[idx]
            if lens == lens_name:
                bucket[idx] = (lens_name, new_value)
                replaced = True
                break
        if not replaced:
            bucket.append((lens_name, new_value))
    elif op == '-':
        # print("Deleting ", op, lens_name)
        for idx in range(len(bucket)):
            lens, value = bucket[idx]
            if lens == lens_name:
                if len(bucket) > 0:
                    del bucket[idx]
                else:
                    bucket.clear()
                break    


# print(buckets[:4])

ans = 0
for (bucket_idx, bucket) in enumerate(buckets):
    for lens_idx, (lens, focal_length) in enumerate(bucket):
        # print(f"Lens: {lens} ={(bucket_idx + 1) * (lens_idx + 1) * focal_length}")
        ans += (bucket_idx + 1) * (lens_idx + 1) * focal_length


print(f"Part2: {ans}")