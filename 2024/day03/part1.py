import re

def solve(input: str):
    answer = 0

    for match in re.finditer(r'mul\((\d+),(\d+)\)', input):
        
        operand_l = int(match.groups()[0])
        operand_r = int(match.groups()[1])

        mul = operand_l * operand_r

        print(f'found match {operand_l} x {operand_r} = {mul}')

        answer += mul

    print("answer", answer)
    return answer

example_input = """
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
""".strip()

assert solve(example_input) == 161

real_input = ""
with open('input.txt') as f:
    real_input = f.read().strip()

solve(real_input)
# 185797128
