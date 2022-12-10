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

input = """
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20""".strip()

input = open('input.txt').read().strip()

# head = (0, 0)
rope = [(0, 0) for _ in range(10)]
print("rope", rope)

tail_positions = set()

def get_dir_deltas(direction):
    if direction == "R":
        return (1, 0)
    if direction == "U": return (0, 1)
    if direction == "L": return (-1, 0)
    if direction == "D": return (0, -1)

    assert False, f"bad direction {direction}"

def print_debug():
    for y in range(-20, 20):
        for x in range(-20, 20):
            x = x
            y = y
            if (x, y) in set(rope):
                i = rope.index((x, y))
                print(f'{i}', end='')
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

    # print('from', head, 'to', tail, 'is', (dtailx, dtaily))

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

def update_tail(head, tail):
    # returns the new tail

    # move the head
    # new_head = (head[0] + dx, head[1] + dy)
    new_tail = list(tail)

    # if head two steps away from up down left or right, tail must move one step in that direction
    dtailx = (head[0] - tail[0])
    dtaily = (head[1] - tail[1])

    # print('from', head, 'to', tail, 'is', (dtailx, dtaily))

    if abs(dtailx) <= 1 and abs(dtaily) <= 1:
        # tail does not move, because it is directly adjacent
        pass
    else:
        # move each dimension as requrired by 1
        if dtailx < 0:
            new_tail[0] -= 1
        elif dtailx > 0:
            new_tail[0] += 1

        if dtaily < 0:
            new_tail[1] -= 1
        elif dtaily > 0:
            new_tail[1] += 1

    return tuple(new_tail)

for step in input.splitlines():
    direction, distance = step.split()
    distance = int(distance)
    print('==', direction, distance, '==')

    dx, dy = get_dir_deltas(direction)

    print_debug()

    for _ in range(distance):
        step_dx = dx
        step_dy = dy

        # move the head of the rope
        new_head = (rope[0][0] + dx, rope[0][1] + dy)
        rope[0] = new_head

        for idx in range(len(rope) - 1):
            head_idx = idx
            tail_idx = idx + 1

            new_tail = update_tail(rope[head_idx], rope[tail_idx])

            # rope[head_idx] = new_head
            rope[tail_idx] = new_tail
        # print_debug()
        # head, tail = update_positions(head, tail, dx, dy)
        tail_positions.add(rope[len(rope) - 1])

print('answer', len(tail_positions))