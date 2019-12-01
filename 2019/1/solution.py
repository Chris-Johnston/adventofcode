# fuel required to launch module based on mass
# fuel = floor(mass / 3) - 2

import math

def fuel(mass: int) -> int:
    return (mass // 3) - 2

assert fuel(12) == 2
assert fuel(14) == 2
assert fuel(1969) == 654
assert fuel(100756) == 33583


if __name__ == "__main__":
    sum = 0
    with open("input.txt") as f:
        contents = f.readlines()
        for line in contents:
            if line:
                mass = int(line)
                sum += fuel(mass)
    print(sum)
