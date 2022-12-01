example_input = """
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"""

input = example_input

input = open("input.txt", 'r').read()

calories = 0
counts = []

for line in input.splitlines():
    if line == "":
        if calories != 0:
            print("adding ", calories)
            counts.append(calories)
        calories = 0
    else:
        calories += int(line)

counts.append(calories)

counts.sort()

top_three = counts[-1] + counts[-2] + counts[-3]
print(top_three)

