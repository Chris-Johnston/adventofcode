input = """
30373
25512
65332
33549
35390
""".strip()

def parse_grid(input: str):
    rows = []
    for line in input.splitlines():
        row = []
        for c in line.strip():
            row.append(int(c))
        if len(row) > 0:
            rows.append(row)
    return rows

grid = parse_grid(input)
print(grid)


def get_view(current_tree, view):
    if len(view) == 0:
        return 0
    for i, x in enumerate(view):
        # print('get_view', (x, i), '>=', current_tree)
        if x >= current_tree:
            return i + 1
    return len(view)

def get_view_distance(input):
    scenic_scores = [] # just get all of them
    grid_size = len(input)
    for y in range(grid_size):
        for x in range(grid_size):
            current_val = input[y][x] # this was reversed which is why part 1 took so long
            row = input[y]
            col = [input[yc][x] for yc in range(grid_size)]

            # swap west and north so traversal is from the point
            to_west = row[:x][::-1]
            to_east = row[x + 1:]
            to_north = col[:y][::-1]
            to_south = col[y + 1:]

            # print((x, y), 'is', current_val)
            # print('n', to_north)
            # print('e', to_east)
            # print('s', to_south)
            # print('w', to_west)

            e = get_view(current_val, to_east)
            w = get_view(current_val, to_west)
            n = get_view(current_val, to_north)
            s = get_view(current_val, to_south)
            score = n * e * s * w
            # print('n', n, 'e', e, 's', s, 'w', w, 'score =', score)

            scenic_scores.append(score)
    return max(scenic_scores)

visible = get_view_distance(grid)

print('test answer', visible)

input_f = open('input.txt').read().strip()
grid_f = parse_grid(input_f)
visible_f = get_view_distance(grid_f)
print('answer', visible_f)