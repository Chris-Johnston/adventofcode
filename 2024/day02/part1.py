
def solve(input: str) -> int:

    answer = 0

    for report in input.splitlines():
        levels = [int(x) for x in report.split()]

        is_valid = check_valid_report(levels)
        if is_valid:
            answer += 1

    print("answer", answer)
    return answer

def check_valid_report(levels):
    prev_value = levels[0]
    previous_delta = None

    for idx in range(len(levels) - 1):
        val = levels[idx + 1]

        delta = prev_value - val

        # check bounds
        if delta == 0 or abs(delta) > 3:
            print("invalid", levels, "due to delta", delta)
            return False

        if previous_delta is not None:
            # check trend
            if previous_delta < 0 and delta > 0:
                print("invalid", levels, "due to increase when previous was decrease", delta)
                return False
            if previous_delta > 0 and delta < 0:
                print("invalid", levels, "due to decrease when previous was increase", delta)
                return False
        
        previous_delta = delta
        prev_value = val

    return True

example_input = """
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
""".strip()

assert solve(example_input) == 2

real_input = ""
with open('input.txt') as f:
    real_input = f.read()

solve(real_input)
# 314
