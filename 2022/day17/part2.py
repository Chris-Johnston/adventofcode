# input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"
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

# idx is the y coord, going to look for repeating sequences in this 
# map_but_as_bits = []
increase_in_height = []

# for turn in range(2022):
for turn in range(2022 + 10000):
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
    
    delta_highest_rock = highest_rock
    for c in moving:
        delta_highest_rock = max(c[1] + 1, delta_highest_rock)

    increase_in_height.append(delta_highest_rock - highest_rock)
    print('turn', turn, 'had increase of ', delta_highest_rock - highest_rock)
    highest_rock = delta_highest_rock

    # debug_example(play_area, moving)

# 1514285714288

print("answer", highest_rock)
# assert highest_rock == 3068
# assert highest_rock == 3175

# assert highest_rock != 3506, "too high"

# 3175
# 40 * 5
# 1514285714288
# 3175 % 1438

# >>> 1514285714288 % 40
# 8
# >>> 1514285714288 / 40
# 37857142857.2
# >>> 

# for y in range(highest_rock):
#     row = 0
#     for x in range(7):
#         if (x, y) in play_area:
#             row |= (7 - x) << 1
#     map_but_as_bits.append(row)

# print(map_but_as_bits)

# used for getting numbers
# for i in range(len(increase_in_height) // 2):
#     for k in range(i):
#         if i < 2:
#             continue
#         a = increase_in_height[k:k+i]
#         b = increase_in_height[k+i:k+i*2]
#         if a == b:
#             print('repeating sequence of len', i, 'offset by', k, (k, k+i), (k+i, k+i*2))
#             break

# example
# repeating sequence of len 35 offset by 15 (15, 50) (50, 85)

# offset = sum(increase_in_height[:15])
# repeating_sequence = sum(increase_in_height[15:15 + 35])

# # multiplier
# mul = 1000000000000 // 35
# # example input happens to be offset by 15, so don't need to worry
# answer = offset + mul * repeating_sequence
# print("answer 2", answer)

# repeating sequence of len 1760 offset by 582 (582, 2342) (2342, 4102)
offset = sum(increase_in_height[:582])
repeating_sequence_sum = sum(increase_in_height[582:582 + 1760])
mul = ( 1000000000000 - 582 ) // 1760

right_offset = ( 1000000000000 - 582 ) % 1760
repeating_sequence = increase_in_height[582:582 + 1760]
right_seq = repeating_sequence[:right_offset]

answer = offset + sum(right_seq) + mul * repeating_sequence_sum
print("answer", answer)
# answer 1555113636385