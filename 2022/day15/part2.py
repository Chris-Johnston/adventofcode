import re, math

input = """
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
""".strip()

input_re = re.compile(r"Sensor at x=(\-?\d+), y=(\-?\d+): closest beacon is at x=(\-?\d+), y=(\-?\d+)")

def parse_input(input: str):
    # key is sensor, value is beacon
    sensors = {}
    beacons = set()

    for line in input.splitlines():
        match = input_re.match(line)
        g = match.groups()
        sensor_x, sensor_y, beacon_x, beacon_y = g
        sensor_x = int(sensor_x)
        sensor_y = int(sensor_y)
        beacon_x = int(beacon_x)
        beacon_y = int(beacon_y)
        # print((sensor_x, sensor_y), (beacon_x, beacon_y))
        sensor = (sensor_x, sensor_y)
        beacon = (beacon_x, beacon_y)
        sensors[sensor] = beacon
        beacons.add(beacon)
    return sensors, beacons

sensors, beacons = parse_input(input)



def get_manhattan_distance(s, b):
    dx = abs(s[0] - b[0])
    dy = abs(s[1] - b[1])
    return dx + dy

def fill_impossible_spots(sensors, y_target):
    impossible = set()

    for s in sensors:
        b = sensors[s]
        manhattan_distance = get_manhattan_distance(s, b)
        y_delta = abs(s[1] - y_target)
        
        if y_delta > manhattan_distance:
            continue

        # print("filling with manhattan distance", manhattan_distance)

        x_max = (manhattan_distance - y_delta)

        # print("point", s, "with manhat dist", manhattan_distance, "at y_target has x_max of", x_max)

        # print("num points before", len(impossible))
        # for x in range(x_max):
        #     impossible.add((s[0] - x, y_target))
        #     impossible.add((s[0] + x, y_target))
        impossible.add((s[0] - x_max, s[0] + x_max))
        # print("num points after", len(impossible))

        # for x in range(manhattan_distance):
        #     for y in range(manhattan_distance - x):
        #         deltas = [
        #             [1, 1],
        #             [-1, 1],
        #             [1, -1],
        #             [-1, -1]
        #         ]
        #         for d in deltas:
        #             p = (x * d[0] + s[0], y * d[1] + s[1])
        #             impossible.add(p)
    
    return impossible

def is_in_range(r1, r2):
    # if r1 is within r2
    # if r1 min between
    return (r2[0] <= r1[0] <= r2[1] or r2[0] <= r1[1] <= r2[1]) or (r1[0] <= r2[0] <= r1[1] or r1[0] <= r2[1] <= r1[1])

def find_spots_with_y(impossible_spots, sensors, beacons, y_val, max_range):
    # matches = set()
    x_min = 400000
    x_max = 0
    # for x, y in impossible_spots:
    #     # if (x, y) in sensors:
    #     #     continue
    #     # if (x, y) in beacons:
    #     #     continue
    #     if y == y_val:
    #         matches.add((x, y))

    #         # track min max x
    #         x_min = min(x_min, x)
    #         x_max = max(x_max, x)

    if y_val == 11:
        print(sorted(impossible_spots))


    left_range_min = max_range
    left_range_max = -max_range
    right_range_min = max_range
    right_range_max = -max_range

    ranges = sorted(impossible_spots)

    # initial populate
    left_range_min, left_range_max = ranges[0]
    right_range_min, right_range_max = ranges[-1]

    for r in ranges:
        # print("range", r)
        if is_in_range(r, (left_range_min, left_range_max)):
            # print("left in range")
            left_range_min = min(r[0], left_range_min)
            left_range_max = max(r[1], left_range_max)
        
        if is_in_range(r, (right_range_min, right_range_max)):
            # print("right in range")
            right_range_min = min(r[0], right_range_min)
            right_range_max = max(r[1], right_range_max)
    
    if y_val == 11:
        print((left_range_min, left_range_max), (right_range_min, right_range_max))

    # if the two ranges intersect
    if is_in_range((left_range_min, left_range_max), (right_range_min, right_range_max)):
        return None
    elif left_range_max + 1 == right_range_min:
        # ranges next to each other
        return None
    else:
        print(ranges)
        print((left_range_min, left_range_max), (right_range_min, right_range_max))
        print("found a gap between", left_range_max , "and" , right_range_min)
        return (left_range_max + 1, y_val)

    # for i in range(len(ranges)):
    #     start, end = ranges[i]

    #     # for x in range(start, end):
    #     #     range_set.add(x)
    
    mask_set = set(range(min(range_set), max(range_set)))
    # for x in range(min(range_set), max(range_set)):
    #     mask_set.add(x)
    
    result = mask_set.difference(range_set)
    if len(result) == 1:
        # print("result" ,result)
        x = list(result)[0]
        return (x, y_val)
    else:
        return None

    if y_val == 11:
        print(sorted(range_set))
        # if start_range[1] + 2 == end_range[0]:
        #     # found it
        #     return (start_range[1] + 1, y_val)

    # for x in range(x_min, x_max):
    #     if (x - 1, y_val) in matches and (x, y_val) not in matches and (x + 1, y_val) in matches:
    #         print("Found one", (x, y_val))
    #         return (x, y_val)

    return None

found = False
for y in range(20):
    print('checking y', y)
    impossible_spots = fill_impossible_spots(sensors, y)
    matches = find_spots_with_y(impossible_spots, sensors, beacons, y, 20)
    if matches is not None:
        print("answer", matches)
        freq = matches[0] * 4000000 + matches[1]
        print("freq", freq)
        assert freq == 56000011, "wrong example answer"
        found = True
        break

assert found

# 

input = open("input.txt").read().strip()
sensors, beacons = parse_input(input)
# print('filling impossible spots')
# impossible_spots = fill_impossible_spots(sensors, 2000000)
# print('finding spots with y')
# matches = find_spots_with_y(impossible_spots, sensors, beacons, 2000000)
# print("final answer", matches)


for y in range(4000000):
    if y % 10000 == 0:
        print('checking y', y)
    impossible_spots = fill_impossible_spots(sensors, y)
    matches = find_spots_with_y(impossible_spots, sensors, beacons, y, 4000000)
    if matches is not None:
        print("answer", matches)
        freq = matches[0] * 4000000 + matches[1]
        print("freq", freq)
        # (2949122, 3041245)
        assert freq == 11796491041245, "wrong answer"
        found = True
        break

assert found