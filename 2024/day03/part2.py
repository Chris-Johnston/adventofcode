# part 2

import re

def try_index(input: str, substring: str):
    try:
        return input.index(substring)
    except ValueError:
        return -1

def solve(input: str):
    answer = 0

    mul_enabled = True

    # input will be substring'ed as we go along

    while len(input) > 0:

        # of the currently valid tokens, find the first which occurs next

        # do_index = input.index('do()') or -1
        do_index = try_index(input, 'do()')
        # dont_index = input.index('don\'t()') or -1
        dont_index = try_index(input, 'don\'t()')
        mul_match = re.search(r'mul\((\d+),(\d+)\)', input)

        mul_index = -1
        mul_end_index = -1

        if mul_match is not None:
            mul_index = mul_match.start(0)
            mul_end_index = mul_match.end(0)

        possible_matches = []

        if do_index != -1:
            possible_matches.append(('do', do_index, len('do()')))

        if dont_index != -1:
            possible_matches.append(('dont', dont_index, len('don\'t()')))

        if mul_index != -1:
            possible_matches.append(('mul', mul_index, len(mul_match.group())))

        print('found candidates:', possible_matches)
        
        if len(possible_matches) == 0:
            # no matches, all done
            print('no matches')
            break
        
        # pick the first match
        possible_matches.sort(key=lambda x: x[1])
        action, index, match_len = possible_matches[0]

        if action == 'do':
            mul_enabled = True
        if action == 'dont':
            mul_enabled = False
        if action == 'mul':
            if mul_enabled:
                operand_l = int(mul_match.groups()[0])
                operand_r = int(mul_match.groups()[1])
                mul = operand_l * operand_r
                print(f'found match {operand_l} x {operand_r} = {mul}')
                answer += mul
            else:
                print('found a mul but was not enabled')
        
        input = input[index + match_len:]

    print("answer", answer)
    return answer

example_input = """
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
""".strip()

assert solve(example_input) == 48

real_input = ""
with open('input.txt') as f:
    real_input = f.read().strip()

solve(real_input)
# 89798695