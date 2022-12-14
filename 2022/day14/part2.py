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
    y_coords = []

    for line in input.splitlines():
        coords = line.split(" -> ")
        for coordidx in range(len(coords) - 1):
            start_coord = coords[coordidx]
            end_coord = coords[coordidx + 1]
            start_coord = to_tuple(start_coord)
            end_coord = to_tuple(end_coord)
            # track y max
            y_coords.append(start_coord[1])
            y_coords.append(end_coord[1])


            fill_single_line(points, start_coord, end_coord)
    return max(y_coords)

y_max = fill_lines(points, input)
y_max += 2 # offset
print('y max is', y_max)

def debug_example(points):
    for y in range(12):
        for x in range(490, 512):
            if (y) == y_max:
                print('#', end='')
                continue
            if (x, y) == (500, 0):
                print('+', end='')
                continue
            p = (x, y)
            if p in points:
                print(points[p], end='')
            else:
                print('.', end='')
        print()

debug_example(points)

def fill_sand(points):
    # the path the sand travels while falling, if it hits void, these are all marked
    # with void
    path = []
    sand_point = [500, 0]

    # while sand_point[1] < 1000: # inf limit
    while True:
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
            if candidate[1] == y_max:
                # floor
                break
            
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
    
    # returns true when the point hasn't moved
    return len(path) == 0


answer = 0
for iterations in range(9999999999999):
    print("filling sand #", iterations)
    is_settled = fill_sand(points)
    if is_settled:
        debug_example(points)
        print("settled after", iterations, "iterations")
        answer = iterations + 1 # off by one
        break

print('answer', answer)
