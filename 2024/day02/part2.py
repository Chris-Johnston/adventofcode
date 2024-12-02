# part 2

def solve(input: str) -> int:

    answer = 0

    for report in input.splitlines():
        levels = [int(x) for x in report.split()]

        is_valid = check_valid_report(levels)
        if is_valid:
            answer += 1
            # print(levels, "was VALID")
        else:
            # print(levels, "was invalid")
            removal_valid = check_if_removal_makes_valid(levels)
            if removal_valid:
                answer += 1

    print("answer", answer)
    return answer

def check_if_removal_makes_valid(levels):
    valid = 0
    for x in range(len(levels)):
        copy = levels.copy()
        del copy[x]

        is_valid = check_valid_report(copy)

        # if any one removal causes valid, then exit early
        if is_valid:
            return True
    
    return False

    
# not sure if this is working, so trying a brute force option where I just opt
# to remove an index in case the issue is with the start of the list

# turns out that the reason this did not work is that it did not consider the effect
# of removing the first index
# and so maybe a combination of these two would have a test using this, and if this still
# returned false, checking that if the list with the first index removed was also false
# this would prevent the n^2 loop

# def check_valid_report(levels):
#     prev_value = levels[0]
#     previous_delta = None

#     has_one_failure = False

#     for idx in range(len(levels) - 1):
        
#         val = levels[idx + 1]
#         delta = prev_value - val

#         # check bounds
#         if delta == 0 or abs(delta) > 3:
#             print("invalid", levels, "due to delta", delta)

#             if has_one_failure:
#                 return False
#             else:
#                 print("first failure")
#                 has_one_failure = True
#                 continue

#         if previous_delta is not None:
#             # check trend
#             if previous_delta < 0 and delta > 0:

#                 print("invalid", levels, "due to increase when previous was decrease", delta)
#                 if has_one_failure:
#                     return False
#                 else:
#                     print("first failure")
#                     has_one_failure = True
#                     continue

#             if previous_delta > 0 and delta < 0:

#                 print("invalid", levels, "due to decrease when previous was increase", delta)
#                 if has_one_failure:
#                     return False
#                 else:
#                     print("first failure")
#                     has_one_failure = True
#                     continue
        
#         previous_delta = delta
#         prev_value = val

#     return True

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

assert solve(example_input) == 4

real_input = ""
with open('input.txt') as f:
    real_input = f.read().strip()

answer = solve(real_input)

assert answer > 354 # too low

# 373

