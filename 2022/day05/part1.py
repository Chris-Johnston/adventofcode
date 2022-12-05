input = """
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"""# .strip()

input = open("input.txt").read()

lines = input.splitlines()
idx = 0

stacks = {}
end_of_crates = False

# read the stacks from the input
while True:
    # read 4 chars at a time, either contains whitespace or a crate
    current_line = lines[idx]

    if current_line == '':
        idx += 1
        continue

    for char_idx in range(0, len(current_line), 4):
        crate = current_line[char_idx:char_idx + 4].strip()
        if crate == '':
            # print('no crate')
            pass
        elif '[' in crate:
            col = int(char_idx / 4) + 1 # 1 index
            print('found crate in col', crate, col)

            if not col in stacks:
                stacks[col] = []

            # stacks[col].append(crate)
            stacks[col].insert(0, crate)
        else:
            # print('end of crates')
            end_of_crates = True
            pass

    idx += 1

    if end_of_crates:
        break

print('inital state:')
print(stacks)

import re
# operation regex
oper_regex = re.compile("move (\d+) from (\d+) to (\d+)")

# read the operations
while True:
    if idx >= len(lines):
        print('end')
        break
    current_line = lines[idx]
    m = oper_regex.match(current_line)
    if m is None:
        idx += 1
        continue

    mcount, mfrom, mto = m.groups()
    mcount = int(mcount)
    mfrom = int(mfrom)
    mto = int(mto)

    print("moving", mcount, "from", mfrom, "to", mto)
    print(stacks)

    for c in range(mcount):
        in_transit = stacks[mfrom].pop()
        stacks[mto].append(in_transit)

    print(stacks)

    idx += 1

print(stacks)

print('solution')

keys = sorted(stacks.keys())

for key in keys:
    print(key, stacks[key].pop())
    