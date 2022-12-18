ex_1_input = """
1,1,1
2,1,1
""".strip()

ex_2_input = """
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
""".strip()


def parse_cubes(input: str):
    cubes = set()
    for line in input.splitlines():
        if "," not in line:
            continue
        x, y, z = line.split(",")
        x = int(x)
        y = int(y)
        z = int(z)

        cubes.add((x, y, z))
    return cubes

def get_surface_area(cubes: set):
    check_dimensions = [
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ]

    connected_count = 0
    not_connected_count = 0

    for cube in cubes:
        for dimension in check_dimensions:
            to_check = (
                dimension[0] + cube[0],
                dimension[1] + cube[1],
                dimension[2] + cube[2]
            )

            if to_check in cubes:
                connected_count += 1
            else:
                not_connected_count += 1
    return not_connected_count

def solve(input: str) -> int:
    c = parse_cubes(input)
    return get_surface_area(c)

assert solve(ex_1_input) == 10
assert solve(ex_2_input) == 64

input = open('input.txt').read().strip()
answer = solve(input)

print("answer", answer)