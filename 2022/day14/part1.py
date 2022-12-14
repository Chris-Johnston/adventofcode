input = """
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
""".strip()

input = open("input.txt").read().strip()

# key is the coordinate, value is the character indicating the thing 
# blocking it
points = {}

def to_tuple(coords):
    x, y = coords.split(",")
    x = int(x)
    y = int(y)
    return (x, y)

def fill_single_line(points, start, end):
    if start[0] == end[0]:
        # changes in the y direction
        y_min = min(start[1], end[1])
        y_max = max(start[1], end[1])

        for y in range(y_min, y_max + 1):
            points[(start[0], y)] = '#'
    else:
        # changes in the x direction
        x_min = min(start[0], end[0])
        x_max = max(start[0], end[0])

        for x in range(x_min, x_max + 1):
            points[(x, start[1])] = '#'

def fill_lines(points: dict, input: str):
    for line in input.splitlines():
        coords = line.split(" -> ")
        for coordidx in range(len(coords) - 1):
            start_coord = coords[coordidx]
            end_coord = coords[coordidx + 1]
            start_coord = to_tuple(start_coord)
            end_coord = to_tuple(end_coord)

            fill_single_line(points, start_coord, end_coord)

def debug_example(points):
    for y in range(10):
        for x in range(493, 504):
            if (x, y) == (500, 0):
                print('+', end='')
                continue
            p = (x, y)
            if p in points:
                print(points[p], end='')
            else:
                print('.', end='')
        print()

fill_lines(points, input)
debug_example(points)

def fill_sand(points):
    # the path the sand travels while falling, if it hits void, these are all marked
    # with void
    path = []
    sand_point = [500, 0]

    while sand_point[1] < 1000: # inf limit
        moved = False
        # if can fall down
        candidates = [
            # straight down
            (sand_point[0], sand_point[1] + 1),
            # down to the left
            (sand_point[0] - 1, sand_point[1] + 1),
            # down to the right
            (sand_point[0] + 1, sand_point[1] + 1)
        ]
        for candidate in candidates:
            if candidate not in points:
                # print("updating point from", sand_point, "to", candidate)
                sand_point = list(candidate)
                path.append(sand_point)
                moved = True
                break
            else:
                continue

        # candidate list exhausted, fill in the spot
        if not moved:
            points[tuple(sand_point)] = 'o'
            break
    
    if sand_point[1] == 1000:
        print("hit oob")
        for p in path:
            p = tuple(p)
            points[p] = '~'
        return False
    return True


for iterations in range(9000):
    print("filling sand #", iterations)
    is_inbounds = fill_sand(points)
    if not is_inbounds:
        debug_example(points)
        print("went out of bounds after", iterations, "iterations")
        break

