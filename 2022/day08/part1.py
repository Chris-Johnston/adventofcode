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

def count_visible(input):
    visible = 0
    grid_size = len(input)
    interior = 0
    exterior = 0
    for y in range(grid_size):
        for x in range(grid_size):
            # print('looking at', (x,y))
            # edges
            if x == 0 or x == (grid_size - 1):
                # print((x,y), 'is an edge')
                exterior += 1
                # visible += 1
            elif y == 0 or y == (grid_size - 1):
                # print((x,y), 'is an edge')
                # visible += 1
                exterior += 1
            else:
                current_val = input[y][x] # this was reversed which is why part 1 took so long
                print((x, y), 'is', current_val)
                row = input[y]
                col = [input[yc][x] for yc in range(grid_size)]

                # to_east = (row[:x])
                # to_west = (row[x + 1:])
                # to_north = (col[:y])
                # to_south = (col[y + 1:])
                to_west = list(sorted(row[:x]))
                to_east = list(sorted(row[x + 1:]))
                to_north = list(sorted(col[:y]))
                to_south = list(sorted(col[y + 1:]))

                print('n', to_north)
                print('e', to_east)
                print('s', to_south)
                print('w', to_west)

                # combine, if current value
                # is less than directions, then good
                # neighbors = to_east.union(to_west).union(to_north).union(to_south)
                # ighbors = list(sorted(neighbors))

                if current_val > to_east[-1]:
                    print((x, y), 'visible from east')
                    interior += 1
                    continue
                if current_val > to_west[-1]:
                    print((x, y), 'visible from west')
                    interior += 1
                    continue
                if current_val > to_north[-1]:
                    print((x, y), 'visible from north')
                    interior += 1
                    continue
                if current_val > to_south[-1]:
                    print((x, y), 'visible from south')
                    interior += 1
                    continue

                print((x, y), 'is visible in interior')
    print(interior, 'interior,', exterior, 'exterior')
    return interior + exterior

visible = count_visible(grid)

print('test answer', visible)

input_f = open('input.txt').read().strip()
grid_f = parse_grid(input_f)
visible_f = count_visible(grid_f)
print('answer', visible_f)
assert visible_f != 1698, "not the answer"