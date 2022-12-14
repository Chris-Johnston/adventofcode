import json

input = """
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
""".strip()

input = open('input.txt').read().strip()

lines_iter = iter(input.splitlines())

def is_packet_inorder(left, right, nest = 0):
    # print('evaluating', left, right)
    left_idx = 0
    right_idx = 0
    print(" " * nest, "Compare", left, "vs", right)

    while True:
        # both are oob
        if left_idx == len(left) and right_idx == len(right):
            # print("lists are same size")
            return None
        elif left_idx == len(left) and not right_idx == len(right):
            # if left oob but right is not
            return True
        elif not left_idx == len(left) and right_idx == len(right):
            # if left oob but right is not
            return False

        l = left[left_idx]
        r = right[right_idx]

        print(" " * nest, "Compare", l, "vs", r)

        left_idx += 1
        right_idx += 1

        if isinstance(l, int) and isinstance(r, int):
            if l == r:
                continue
            if l > r:
                # print("left", l, ">", "right", right)
                return False
            else:
                # print('right order')
                return True
        elif isinstance(l, list) and isinstance(r, list):
            print(" " * nest, "List compare", l, "vs", r)
            x = is_packet_inorder(l, r, nest + 1)
            if x is not None:
                return x
        else:
            if isinstance(l, int):
                print(" " * nest, "L Mixed list compare", l, "vs", r)
                x = is_packet_inorder([l], r, nest + 1)
                if x is not None:
                    return x
            else:
                print(" " * nest, "R Mixed list compare", l, "vs", r)
                x = is_packet_inorder(l, [r], nest + 1)
                if x is not None:
                    return x

# answers = []
# while True:
#     line = ""
#     try:
#         line = next(lines_iter)
#     except StopIteration:
#         break
#     if line == "":
#         continue
#     line2 = next(lines_iter)

#     left = json.loads(line)
#     right = json.loads(line2)

#     print("evaluting", left, "and", right)
#     a = is_packet_inorder(left, right)
#     print(a)
#     answers.append(a)

packets = []
packets.append([[2]])
packets.append([[6]])
while True:
    line = ""
    try:
        line = next(lines_iter)
    except StopIteration:
        break
    if line == "":
        continue
    packets.append(json.loads(line))

print("unsorted")
for p in packets:
    print("\t", p)

def packet_compare(item1, item2):
    x = is_packet_inorder(item1, item2)
    # if x is None:
    #     return 0
    if x:
        return -1
    else:
        return 1
from functools import cmp_to_key
# packets.sort(key=packet_compare)
packets = sorted(packets, key=cmp_to_key(lambda l, r: packet_compare(l, r)))

print("sorted")
for p in packets:
    print("\t", p)

decoder2 = packets.index([[2]]) + 1
decoder6 = packets.index([[6]]) + 1
answer = decoder2 * decoder6
print("answer", answer)