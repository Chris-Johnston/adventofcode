from dataclasses import dataclass

input = """
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
""".strip()

input = open('input.txt').read().strip()

def parse_height_map(input: str):
    # going to see if I regret doing this in a map
    height_map = {}
    start_point = (-1, -1)
    end_point = (-1, -1)
    x = -1
    y = -1
    for line in input.splitlines():
        x = -1
        y += 1
        for c in line:
            x += 1
            point = (x, y)

            if c == "S":
                height_map[point] = ord('a')
                start_point = point
            elif c == "E":
                height_map[point] = ord('z')
                end_point = point
            else:
                height_map[point] = ord(c)
    return height_map, start_point, end_point

height_map, start_p, end_p = parse_height_map(input)

print("parsed map with", len(height_map), "points with start", start_p, "and end", end_p)
# print(height_map)

@dataclass
class Node():
    # the point that this node represents
    point: tuple
    # the height of the current node
    height: int
    # current path from start
    path: list

def find_next_hop_candidates(map: dict, current_point: tuple, current_height: int):
    # given a coordinate, check if the next hop can be visited
    deltas = [(-1, 0), (1, 0), (0, 1), (0, -1)]
    for d in deltas:
        delta_point = (current_point[0] + d[0], current_point[1] + d[1])

        if delta_point not in map:
            # print('point', delta_point, 'was oob')
            continue
        
        if map[delta_point] <= current_height + 1:
        # if map[delta_point] == current_height or map[delta_point] == current_height + 1:
            # print('next point', delta_point)
            yield delta_point

def find_path(map: dict, start_point: tuple, end_point: tuple):
    # key is the destination node from the start point
    # if there is a shorter path from start to key, then update the path with this
    adjacency_lists = {}
    evaluated_adjacencies = set()

    nodes_to_consider = []
    visited_nodes = set()

    start_node = Node(start_point, map[start_point], [])
    # print('start node', start_node)
    nodes_to_consider.append(start_node)

    # solve.. hopefully
    while len(nodes_to_consider) > 0:
        n = nodes_to_consider.pop()
        # print('checking node', n)
        visited_nodes.add(n.point)

        # update adjacency
        if n.point not in adjacency_lists:
            adjacency_lists[n.point] = n.path
        else:
            if len(adjacency_lists[n.point]) > len(n.path):
                print('found a shorter path to', n.point, "(", len(adjacency_lists[n.point]), len(n.path) ,")")
                adjacency_lists[n.point] = n.path

        # if n.point == end_point:
        #     print('end point', n)
        #     break

        # otherwise find all possible next hops and add them to the list
        for next in find_next_hop_candidates(map, n.point, n.height):
            if (n.point, next) in evaluated_adjacencies:
                continue
            # print('next hop from', n.point, 'is', next)
            # if next in visited_nodes:
            #     continue
            next_path = n.path.copy()
            next_path.append(next)
            next_node = Node(next, map[next], next_path)
            nodes_to_consider.insert(0, next_node)
            evaluated_adjacencies.add((n.point, next))
    
    if end_point not in adjacency_lists:
        return None

    # print('visited', len(visited_nodes))
    # get answer
    # print(adjacency_lists)
    return adjacency_lists[end_point]

start_points = []
for k in height_map.keys():
    v = height_map[k]
    if v == ord('a'):
        start_points.append(k)

distances = []
for point in start_points:
    dist = find_path(height_map, point, end_p)
    if dist is None:
        print('point', point, 'did not have valid path')
        continue
    print("found distance of", len(dist))
    distances.append(len(dist))

min_distance = min(distances)
print("answer", min_distance)

# path = find_path(height_map, start_p, end_p)

# def debug_path(path):
#     pathset = set(path)
#     for y in range(10):
#         for x in range(10):
#             if (x, y) in pathset:
#                 i = path.index((x, y)) % 10
#                 print(i, end='')
#             else:
#                 print('.', end='')
#         print()
# debug_path(path)

# print("answer", len(path))


