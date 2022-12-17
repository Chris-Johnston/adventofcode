import re
from dataclasses import dataclass
import itertools

input = """
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
""".strip()

input = open("input.txt").read().strip()

re_valves = re.compile(r'([A-Z]{2})')
re_rate = re.compile(r'(\d+);')

def parse_input(input: str):
    valve_next = {}
    valve_rate = {}

    for line in input.splitlines():
        valves = re_valves.findall(line)
        rate = re_rate.findall(line)

        # print(valves, rate)
        left = valves[0]
        right = valves[1:]

        rate = int(rate[0])

        valve_next[left] = right
        valve_rate[left] = rate
    return valve_next, valve_rate

valve_next, valve_rate = parse_input(input)
print(valve_next)
print(valve_rate)

valves = set(valve_rate.keys())

# each key is the sum of the possible pressure released by everything on that path
# and the value is the path to follow
# example solution is:

# 'AA' -> 'DD' -> 'CC' -> 'BB' -> 'AA' -> 'II -> JJ -> II -> AA -> DD -> EE
# -> FF -> GG -> HH -> GG -> FF -> EE ...
# we visit EE, but because HH has a higher value we go there first
# so take each of the values

# def find_shortest_path(current_node: str, recurse_path: list, adj):
#     print('finding shortest path from', current_node, 'neighbors are', valve_next[current_node])
#     # immediately adjacent ones are just added to the map
#     for adjacent in valve_next[current_node]:
#         p = recurse_path.copy()
#         p.append(adjacent)
#         adj[adjacent] = p

#         #v = adj[k][-1]
#         v = adjacent
#         # look into the path from the next nodes
#         path = p
#         # print('finding shortest path from neighbor', k)
#         neighbor_adj = find_shortest_path(v, path, adj)

#         for neighbor_k, neighbor_v in neighbor_adj:
#             if neighbor_k not in adj:
#                 adj[neighbor_k] = neighbor_v
#             else:
#                 if len(adj[neighbor_k]) > len(neighbor_v):
#                     adj[neighbor_k] = neighbor_v
#                 else:
#                     # ignore, neighboring path was a longer route
#                     pass
#     return adj

@dataclass
class Node():
    # the name of the node
    name: str
    # the rate of the valve
    rate: int
    # current path from start
    path: list

    def get_adjacent_node_names(self):
        return valve_next[self.name]

def make_nodes(valve_rate):
    nodes = {}
    for k, v in valve_rate:
        n = Node(k, v, [])
        nodes[k] = n
    
    return nodes

def find_paths(start_node: str):
    next_nodes = valves.copy()
    next_nodes.remove(start_node)

    # create Nodes from the known data
    # nodes = make_nodes(valve_rate)

    paths = {}

    for next in next_nodes:
        p = find_path(start_node, next)
        # print("path from", start_node, "to end", next, ":", p)
        paths[next] = p
    # print(paths)
    return paths

def find_path(start_node: str, end_node: str):
    # copied from my own solve in day12 part2

    # key is the dest node from the start point
    adjacency_lists = {}
    evalutaed_adjacencies = set()

    nodes_to_consider = []
    visited_nodes = set()

    start_node = Node(start_node, valve_rate[start_node], [])
    nodes_to_consider.append(start_node)

    while len(nodes_to_consider) > 0:
        n = nodes_to_consider.pop()
        visited_nodes.add(n.name)

        if n.name not in adjacency_lists:
            adjacency_lists[n.name] = n.path
        else:
            if len(adjacency_lists[n.name]) > len(n.path):
                adjacency_lists[n.name] = n.path
        
        for next in n.get_adjacent_node_names():
            if (n.name, next) in evalutaed_adjacencies:
                continue

            next_path = n.path.copy()
            next_path.append(next)
            next_node = Node(next, valve_rate[next], next_path)
            nodes_to_consider.insert(0, next_node)

            evalutaed_adjacencies.add((n.name, next))
    
    if end_node not in adjacency_lists:
        print("no possible route to end", end_node, "from", start_node)
    
    return adjacency_lists[end_node]

# given the shortest paths, find the next valve with highest opportunity
def get_best_opportunity(start_node, shortest_paths, remaining_time, visited_set):

    # this is flawed, because it goes for the max for the current step, which may
    # set up a bad next step

    potentials = []

    for node in shortest_paths.keys():
        path_cost = len(shortest_paths[node])
        rate = valve_rate[node]

        if node in visited_set:
            # don't bother with already visited
            rate = 0
        # path cost is number of moves required to get there
        # 1 move to account for moving the valve
        # this seems correct given the cost to directly adjacent things
        # but seems to suggest we go to II JJ first
        potential = (remaining_time - (path_cost + 1)) * rate

        # include the cost of going to the point and back
        # so II JJ would be (2 * 2 - 1) = 3, II JJ II
        # but DD is (1 * 2 - 1) = 1, DD
        # potential = (remaining_time - 1 - ((path_cost * 2) - 1)) * rate
        # hack?
        # potential = (remaining_time - 1 - ((path_cost * path_cost) - 1)) * rate
        # print('potential of going to', node, 'is', potential, "from path", shortest_paths[node])
        potentials.append((node, potential, path_cost))

    potentials = sorted(potentials, key=lambda x: x[1], reverse=True)
    print("sorted potentials", potentials)
    return potentials[0]

def get_potentials(shortest_paths, remaining_time, visited_set):

    # this is flawed, because it goes for the max for the current step, which may
    # set up a bad next step

    potentials = []

    for node in shortest_paths.keys():
        path_cost = len(shortest_paths[node])
        rate = valve_rate[node]

        if node in visited_set:
            # don't bother with already visited
            rate = 0
        # path cost is number of moves required to get there
        # 1 move to account for moving the valve
        # this seems correct given the cost to directly adjacent things
        # but seems to suggest we go to II JJ first
        potential = (remaining_time - (path_cost + 1)) * rate

        # include the cost of going to the point and back
        # so II JJ would be (2 * 2 - 1) = 3, II JJ II
        # but DD is (1 * 2 - 1) = 1, DD
        # potential = (remaining_time - 1 - ((path_cost * 2) - 1)) * rate
        # hack?
        # potential = (remaining_time - 1 - ((path_cost * path_cost) - 1)) * rate
        # print('potential of going to', node, 'is', potential, "from path", shortest_paths[node])
        potentials.append((node, potential, path_cost))

    potentials = sorted(potentials, key=lambda x: x[1], reverse=True)
    return potentials


def get_valve_order(valve_rate):
    closed = set()
    current = 'AA'
    opportunity = 9999
    time_left = 30
    # makes assumption that AA has no cost, it does not
    visit_order = [ ]

    result = 0

    # while opportunity > 0:
    while time_left > 0 and opportunity > 0:
        # print("closed set", current)
        p = find_paths(current)

        # idea, what if instead of always picking the best opportunity at each hop,
        # we enumerate all possible orderings, then calculate the answer
        # and pick the max
        # this was done because I kept calculating D J H ..  instead of D B J ..
        next, opportunity, path_cost = get_best_opportunity(current, p, time_left, closed)
        result += opportunity
        print("ADDED POTENTIAL", opportunity)
        closed.add(next)
        time_left -= path_cost + 1
        print('next from', current, 'is', next, 'with', time_left, 'time left')
        if opportunity > 0:
            visit_order.append(next)
            current = next
    
    print('visited all required nodes in order', visit_order)
    print('opportunity result', result)
    return visit_order

@dataclass
class Path():
    # gets the current path to closed valves
    path : list
    # gets the possible branches from this step
    child_branches : list
    current_location : str
    score = 0
    time_left = 0

def get_path_tree_start():
    # get a tree of paths and the total answer for each path
    # closed = set()
    interested_paths = get_interested_valves()
    current = 'AA'
    time_left = 30
    current_path = Path([], [], 'AA')
    current_path.time_left = 30

    end_paths = []

    get_path_tree_recurse(current_path, end_paths, interested_paths)
    # print(current_path)
    # print('end paths', end_paths)
    max_path = 0
    max_path_v = None
    for e in end_paths:
        print("end path", e.score, "path", e.path)
        if e.score > max_path:
            max_path = e.score
            max_path_v = e

    # 1651
    # D B J
    print("answer", max_path, max_path_v)

# does not consider current state
shortest_path_cache = {}

def get_shortest_path(current):
    if current in shortest_path_cache:
        return shortest_path_cache[current]
    else:
        shortest_path_cache[current] = find_paths(current)
        return shortest_path_cache[current]


def get_path_tree_recurse(current: Path, end_paths: list, interested_paths):
    print("looking at path", current.score, current.time_left, current.path)

    closed = set(current.path)

    # p = find_paths(current.current_location)
    p = get_shortest_path(current.current_location)
    potentials = get_potentials(p, current.time_left, closed)

    for next, vent, time_cost in potentials:
        if (time_cost + 1) > current.time_left:
            # not enough time for this
            # print("moving to", next, "uses more time", time_cost, "than avail", current.time_left)
            continue
        if vent == 0:
            # no point
            continue
        if next in set(current.path):
            # already closed
            continue

        child_path = Path(current.path.copy(), [], next)
        child_path.path.append(next)
        child_path.score = current.score + vent
        # include time to open valve
        child_path.time_left = current.time_left - time_cost - 1
        child_path.current_location = next

        # print("child path", child_path)

        current.child_branches.append(child_path)

        # need a different end condition
        if len(child_path.path) == len(interested_paths):
            # print("found an end point at", child_path)
            end_paths.append(child_path)
    
    for child in current.child_branches:
        get_path_tree_recurse(child, end_paths, interested_paths)

        end_paths.append(child)



def get_interested_valves():
    interested_set = set()
    for k in valve_rate.keys():
        v = valve_rate[k]
        if v > 0:
            interested_set.add(k)
    return interested_set

def determine_release(path):
    remaining_time = 30
    # closed_set = set()
    closed_increment = 0
    pressure_released = 0
    current_node = 'AA'

    walked_nodes = []

    for p in path:
        walked_nodes.append(p)
        # walk the path to the node p from the current node
        shortest_path = find_path(current_node, p)
        path_cost = len(shortest_path)
        time_delta = path_cost + 1
        if (remaining_time - time_delta) <= 0:
            break

        current_node = p

        # for the time spent moving and opening, update the presure amount released
        remaining_time -= time_delta
        pressure_released += time_delta * closed_increment
        closed_increment += valve_rate[current_node]

    if remaining_time >= 0:
        pressure_released += remaining_time * closed_increment
    else:
        # not enough remaining time
        return None

    return pressure_released, walked_nodes

def enumerate_possible_paths():
    i = 0
    s = get_interested_valves()

    # hack, take off the first 3
    # s = sorted(s)[3:]

    print("getting permutations of", s)
    max_score = 0
    max_path = None
    last_walked_nodes = None
    # this doesn't work for the input, because 56! or whatever is a lot
    # and guessing randomly wil lresult in a lot of paths which are impossible
    for p in itertools.permutations(s, min(10, len(s))):
        i += 1
        if i % 10000 == 0:
            # 14718 is too high
            # 6855 too high
            # 1789 is too low
            # 1985 not it
            print("permutation #", i, p, "current max is", max_score)

        # if p matches the last_walked_nodes, skip
        # prefix_match = False
        # if not last_walked_nodes is None:
        #     prefix_match = True
        #     for wi in range(len(last_walked_nodes) - 2):
        #         if prefix_match:
        #             prefix_match = p[wi] == last_walked_nodes[wi]
            
        # if prefix_match:
        #     continue

        score, walked = determine_release(p)
        if score is None:
            continue
        if score > max_score:
            max_path = p
            max_score = score
        else:
            last_walked_nodes = walked
            # print(last_walked_nodes)
            # print(len(last_walked_nodes))
            # print(last_walked_nodes)

    print("Answer", max_score, max_path)


print("getting path tree")
get_path_tree_start()


# valve_order = get_valve_order(valve_rate)
# assert valve_order[0] == 'DD'
# assert valve_order[1] == 'BB'

# example answer is 1651

# answer = determine_release(valve_order)
# print(valve_order)
# print("answer", answer) # 6855 is too high

# enumerate_possible_paths()

# DD on turn 2 = (20 * 27) = 540
# BB on turn 5 = (13 * 24) = 312
# JJ on turn 9 = (21 * 20) = 420
# HH on turn 17 = (22 * 12) = 264
# EE on turn 21 = (3 * 8) = 24
# CC on turn 24 = (2 * 5) = 10
# == ... 1570 and not 1651? diff by 81, which is off by one turn


# I sure hope I don't have to solve a puzzle, just going to get the max for each
# adjacent valve, if no paths then return back the stack
# edit: nevermind going to build out possible paths from the start first

# turn_stack = [ 'AA' ]
# currently_open = set()

# for turn in range(30):
#     print("turn", turn)

#     current_location = turn_stack[-1]

#     # move to an adjacent area

#     # adjacent areas
#     adjacent = re_valves[current_location]
#     adj_values = []

#     for a in adjacent:
#         if a in currently_open:
#             # do not move to currently open areas as a destination
#             continue
#         rate = valve_rate[a]
#         if rate == 0:
#             # don't bother going to 0 value valves as a destination
#             continue
#         adj_values.append((a, valve_rate[a]))
    
    