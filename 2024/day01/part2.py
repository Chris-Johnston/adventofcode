# part 2

def solve(in_str: str):
    left_list = list()

    # key is the number, value is the count of occurances
    right_dict = {}

    # parse input, add to lists

    for line in in_str.splitlines():
        print("line", line)
        nums = line.split()
        left, right = nums
        left = int(left)
        right = int(right)

        left_list.append(left)
        # add or update the value
        right_dict[right] = right_dict.get(right, 0) + 1

    # sort lists small to large
    left_list = sorted(left_list)

    # answer is sum of the differences

    answer = 0

    for x in range(len(left_list)):
        # similarity score
        score = abs(left_list[x] * right_dict.get(left_list[x], 0))

        print("score", left_list[x], right_dict.get(left_list[x], 0), "was", score)
        answer += score
    
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

assert solve(example) == 31

input_text = ""
with open("input.txt") as f:
    input_text = f.read()

print(solve(input_text))

# 2066446