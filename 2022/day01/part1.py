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
max_calories = 0

for line in input.splitlines():
    if line == "":
        calories = 0
    else:
        calories += int(line)

    if calories > max_calories:
        max_calories = calories

print(max_calories)