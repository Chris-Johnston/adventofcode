# part 2

# fuel required to launch module based on mass
# fuel = floor(mass / 3) - 2

import math

def fuel(mass: int) -> int:
    return max((mass // 3) - 2, 0)

assert fuel(12) == 2
assert fuel(14) == 2
assert fuel(1969) == 654
assert fuel(100756) == 33583

def additional_fuel(mass: int) -> int:
    """
    adds the additional fuel
    """
    sum = 0
    fuel_added = fuel(mass)
    while fuel_added > 0:
        sum += fuel_added
        fuel_added = fuel(fuel_added)
    return sum

assert additional_fuel(14) == 2
assert additional_fuel(1969) == 966
assert additional_fuel(100756) == 50346

if __name__ == "__main__":
    sum = 0
    with open("input.txt") as f:
        contents = f.readlines()
        for line in contents:
            if line:
                mass = int(line)
                sum += additional_fuel(mass)
    print(sum)
