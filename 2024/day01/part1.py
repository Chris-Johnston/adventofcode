def solve(in_str: str):
    left_list = list()
    right_list = list()

    # parse input, add to lists

    for line in in_str.splitlines():
        print("line", line)
        nums = line.split()
        left, right = nums
        left = int(left)
        right = int(right)

        left_list.append(left)
        right_list.append(right)

    # sort lists small to large
    
    left_list = sorted(left_list)
    right_list = sorted(right_list)

    # answer is sum of the differences

    answer = 0

    for x in range(len(left_list)):
        diff = abs(left_list[x] - right_list[x])

        print("diff", left_list[x], right_list[x], "was", diff)
        answer += diff
    
    print("Answer", answer)
    return answer

example = """
3   4
4   3
2   5
1   3
3   9
3   3
""".strip()

assert solve(example) == 11

input_text = ""
with open("input.txt") as f:
    input_text = f.read()

print(solve(input_text))

# 2066446