# couldn't figure out an issue with lifetime in rust
# so swtiching back to python for a day

input = ["COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
"K)YOU", "I)SAN"]

input = []
with open("input.txt") as f:
    input = f.readlines()

orbit_map = {
    # depth, name, parent
    "COM": (0, "COM", None),
}

for item in input:
    center, orbit = item.strip().split(')')
    print(f"center {center}, orbit {orbit}.")

    # if the key not in the map yet, re-add it to handle it later
    if center not in orbit_map:
        print(f"Center {center} is not in the list yet")
        input.append(item)
        continue

    parent = orbit_map[center]
    orbit_map[orbit] = (parent[0] + 1, orbit, parent[1])

print("Map:", orbit_map)

sum = 0
for key in orbit_map.keys():
    sum += orbit_map[key][0]

print("Sum:", sum)

# part 2

you = orbit_map["YOU"]
san = orbit_map["SAN"]

# get the set of hops from YOU -> COM
# get the set of hops from SAN -> COM
# remove the hops after the first one in common

def get_com_hops(hop):
    while True:
        parent_hop = hop[2]
        yield hop
        if parent_hop is None:
            break
        hop = orbit_map[parent_hop]

def to_nodes(hops):
    for hop in hops:
        yield hop[1]
        
you_hops = list(to_nodes(get_com_hops(you)))
san_hops = list(to_nodes(get_com_hops(san)))
print(you_hops)
print(san_hops)

# find the first common hop
common = None
for you_hop in you_hops:
    if you_hop in san_hops:
        common = you_hop
        break
print("common:", common)

distance = you_hops.index(common) + san_hops.index(common) - 2
print("dist:", distance)