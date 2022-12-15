import re

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

        x_max = (manhattan_distance - y_delta) + 1

        print("point", s, "with manhat dist", manhattan_distance, "at y_target has x_max of", x_max)

        print("num points before", len(impossible))
        for x in range(x_max):
            impossible.add((s[0] - x, y_target))
            impossible.add((s[0] + x, y_target))
        print("num points after", len(impossible))

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

impossible_spots = fill_impossible_spots(sensors, 10)

def find_spots_with_y(impossible_spots, sensors, beacons, y_val):
    matches = set()
    for x, y in impossible_spots:
        if (x, y) in sensors:
            continue
        if (x, y) in beacons:
            continue
        if y == y_val:
            matches.add((x, y))
    
    return len(matches)

matches = find_spots_with_y(impossible_spots, sensors, beacons, 10)
print("answer", matches)
assert matches == 26, "wrong example answer"

# 

input = open("input.txt").read().strip()
sensors, beacons = parse_input(input)
print('filling impossible spots')
impossible_spots = fill_impossible_spots(sensors, 2000000)
print('finding spots with y')
matches = find_spots_with_y(impossible_spots, sensors, beacons, 2000000)
print("final answer", matches)