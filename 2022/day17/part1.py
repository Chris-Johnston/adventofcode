input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"
input = open('input.txt').read().strip()
print(input)


rocks = """
####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##
""".strip().split('\n\n')

pieces = [

]

for r in rocks:
    rock_set = set()
    lines = r.split('\n')
    y = len(lines) - 1
    for line in lines:
        for x in range(len(line)):
            if line[x] == '#':
                rock_set.add((x, y))
        y -= 1
    print(rock_set)
    pieces.append(rock_set)

# example


def translate_piece(pieces: set, x, y):
    result = set()
    for p in pieces:
        new_coord = (p[0] + x, p[1] + y)
        result.add(new_coord)
    return result

def check_in_bounds(piece: set, board: set):
    for c in piece:
        if c in board:
            return False
        if c[0] < 0:
            return False
        if c[0] > 6:
            return False
        if c[1] < 0:
            return False
    return True

def debug_example(play_area: set, moving: set):
    return
    for y in range(16):
        y = 15 - y
        print('|', end='')
        for x in range(7):
            p = (x, y)
            if p in play_area:
                print('#', end='')
            elif p in moving:
                print('@', end='')
            else:
                print('.', end='')
        print('|')
    print('---------')
    print()

def get_piece_height(piece: set):
    y_min = 999
    y_max = -1
    for p in piece:
        y_min = min(p[1], y_min)
        y_max = max(p[1], y_max)
    return abs(y_max - y_min)

def get_piece_width(piece: set):
    y_min = 999
    y_max = 0
    for p in piece:
        y_min = min(p[0], y_min)
        y_max = max(p[0], y_max)
    return abs(y_max - y_min)


play_area = set()
highest_rock = 0
input_idx = 0
for turn in range(2022):
# for turn in range(7):
    current_piece = pieces[turn % 5]
    # print("NEW PIECE idx", turn % 4)
    piece_height = get_piece_height(current_piece)
    # print('piece height is', piece_height, 'and highest rock', highest_rock)
    # create new piece

    # print("current", current_piece)
    piece_x = 2
    piece_y = highest_rock + 3

    moving = translate_piece(current_piece.copy(), piece_x, piece_y)
    assert check_in_bounds(moving, play_area), "start piece was not in bounds"

    debug_example(play_area, moving)

    while True: # while falling
        input_step = input[input_idx]
        if input_step == ">":
            # print("moved right")
            # right
            move_maybe = translate_piece(moving, 1, 0)
            if check_in_bounds(move_maybe, play_area):
                moving = move_maybe
        else:
            # print("moved left")
            # left
            move_maybe = translate_piece(moving, -1, 0)
            if check_in_bounds(move_maybe, play_area):
                moving = move_maybe
        
        input_idx += 1
        input_idx %= len(input)

        # debug_example(play_area, moving)
        
        # move down
        move_maybe = translate_piece(moving, 0, -1)
        if check_in_bounds(move_maybe, play_area):
            moving = move_maybe
            # print('moved down')
        else:
            # print('stopped moving')
            # hit something
            for c in moving:
                # highest_rock = max(c[1], highest_rock) - 1
                play_area.add(c)
            break
        
        # debug_example(play_area, moving)
    
    for c in moving:
        highest_rock = max(c[1] + 1, highest_rock)

    # debug_example(play_area, moving)

print("answer", highest_rock)
# assert highest_rock == 3068
assert highest_rock == 3175

assert highest_rock != 3506, "too high"