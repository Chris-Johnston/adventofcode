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

from functools import lru_cache

def get_neighbors(cube: tuple, cubes: set):
    check_dimensions = [
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ]
    for dimension in check_dimensions:
        to_check = (
            dimension[0] + cube[0],
            dimension[1] + cube[1],
            dimension[2] + cube[2]
        )
        # bounds check
        if abs(to_check[0]) > 30:
            continue
        if abs(to_check[1]) > 30:
            continue
        if abs(to_check[2]) > 30:
            continue

        if to_check in cubes:
            # wall
            continue
        yield to_check


def is_node_enclosed(cube: tuple, cubes: set, known_enclosed_nodes: set, known_exposed: set, limit = 10000):
    print('checking if node', cube, 'is enclosed')
    checked_neighbors = set()
    checked_neighbors.add(cube)
    neighbors_to_check = list(get_neighbors(cube, cubes))

    while len(neighbors_to_check) > 0:
        n = neighbors_to_check.pop()
        checked_neighbors.add(n)

        if n in known_enclosed_nodes:
            print('hit a known enclosed node')
            return True, checked_neighbors

        if n in known_exposed:
            print('hit a known exposed')
            return False, set()

        if len(checked_neighbors) > limit:
            print(cube, 'hit limit')
            # print(checked_neighbors)
            return False, checked_neighbors

        for next in get_neighbors(n, cubes):
            if next in checked_neighbors:
                continue
            neighbors_to_check.append(next)

    print(cube, 'had', len(checked_neighbors), 'neighbors')
    return len(checked_neighbors) < limit, checked_neighbors

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
    exterior_faces = 0

    exterior_checks = {}

    for cube in cubes:
        #for dimension in check_dimensions:
        for idx in range(len(check_dimensions)):
            dimension = check_dimensions[idx]
            to_check = (
                dimension[0] + cube[0],
                dimension[1] + cube[1],
                dimension[2] + cube[2]
            )

            if to_check in cubes:
                connected_count += 1
            else:
                if to_check in exterior_checks:
                    # exterior_checks.remove(to_check)
                    exterior_checks[to_check] += 1
                else:
                    # exterior_checks.add((to_check))
                    exterior_checks[to_check] = 1

                not_connected_count += 1

    # assumption, steam pockets are just a single point in size?
    # A: nope, this does not work
    to_remove = set()
    known_exterior = set()

    for k in exterior_checks.keys():
        # print('checking point', k, 'if its exterior')
        # walk each dimension 50 times, if all 6 dimensions collide
        # then remove them from the set
        # and can also remove all of the walked positions
        # walked_points = set()
        cube = k

        if k in to_remove:
            # going to remove this one already
            pass

        is_enclosed, walked_points = is_node_enclosed(cube, cubes, to_remove, known_exterior)
        if is_enclosed:
            to_remove = to_remove.union(walked_points)
        else:
            known_exterior.add(cube)


        # for idx in range(len(check_dimensions)):
        #     dimension = check_dimensions[idx]
        #     has_hit = False
        #     # this does not work, there could be cracks which are not
        #     # clearly visible
        #     to_check = (
        #         dimension[0] + cube[0],
        #         dimension[1] + cube[1],
        #         dimension[2] + cube[2]
        #     )

        #     #     if to_check in cubes:
        #     #         # hits a wall
        #     #         walls_hit += 1
        #     #         has_hit = True
        #     #         break
        #     #     else:
        #     #         walked_points.add(to_check)

        #     # if not has_hit:
        #     #     # walked a whole dimension and didn't have a hit, so skip it
        #     #     break
        
        # if walls_hit == 6:
        #     print("point", cube, "is entirely closed, as well as", len(walked_points), "points walked along the way")
        #     # point is completely enclosed, so mark it and all walked points
        #     # to the set to remove
        #     to_remove = to_remove.union(walked_points)
    
    ext_check_test = sum(exterior_checks.values())
    assert ext_check_test == not_connected_count

    to_remove_count = 0
    for k in to_remove:
        # if a walked point didn't add to the surface area, don't bother
        if k not in exterior_checks:
            continue
        print("removing surface area of", exterior_checks[k], "around point", k)
        to_remove_count += exterior_checks[k]

    


    # for k in exterior_checks.keys():
    #     v = exterior_checks[k]
    #     if v == 6:
    #         to_remove.add(k)

    # print("exterior", len(exterior_checks), exterior_checks)
    print("not connected count was", not_connected_count, "removed", to_remove_count)
    answer = not_connected_count - to_remove_count
    # answer = len(exterior_checks.keys())
    print(answer)
    return answer

def solve(input: str) -> int:
    c = parse_cubes(input)
    return get_surface_area(c)

# assert solve(ex_1_input) == 10
assert solve(ex_2_input) == 58

input = open('input.txt').read().strip()
answer = solve(input)

print("answer", answer)
assert answer != 4298, "too high"
assert answer != 4208, "too high"
assert answer != 2587, "too low"
assert answer == 2604