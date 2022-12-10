input = """
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
""".strip()

input = open('input.txt').read().strip()

head = (0, 0)
tail = (0, 0)

tail_positions = set()

def get_dir_deltas(direction):
    if direction == "R":
        return (1, 0)
    if direction == "U": return (0, 1)
    if direction == "L": return (-1, 0)
    if direction == "D": return (0, -1)

    assert False, f"bad direction {direction}"

def print_debug():
    for y in range(5):
        for x in range(5):
            if head == (x, y):
                print('H', end='')
            elif tail == (x, y):
                print('T', end='')
            elif (x, y) in tail_positions:
                print('#', end='')
            else:
                print('.', end='')
        print()
    print()

def update_positions(head, tail, dx, dy):
    # returns the new head and tail

    # move the head
    new_head = (head[0] + dx, head[1] + dy)
    new_tail = list(tail)

    # if head two steps away from up down left or right, tail must move one step in that direction
    dtailx = abs(new_head[0] - tail[0])
    dtaily = abs(new_head[1] - tail[1])

    if dtailx <= 1 and dtaily <= 1:
        # tail does not move
        pass
    # elif (dtailx == 2 and dtaily != 2) or (dtailx != 2 and dtaily == 2):
    #     new_tail = (tail[0] + dx, tail[1] + dy)
    else:
        # diag, move once in the direction of dx dy
        # but also move once to match the other dimension
        new_tail = [tail[0] + dx, tail[1] + dy]

        if abs(dx) == 1:
            # moving in x direction, converge in y direction
            new_tail[1] = new_head[1]
        else:
            new_tail[0] = new_head[0]
    return new_head, tuple(new_tail)

for step in input.splitlines():
    direction, distance = step.split()
    distance = int(distance)
    print(direction, distance)

    dx, dy = get_dir_deltas(direction)

    for _ in range(distance):
        print_debug()
        head, tail = update_positions(head, tail, dx, dy)
        tail_positions.add(tail)

print('answer', len(tail_positions))