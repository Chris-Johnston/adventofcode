input = """
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
""".strip()

input = open('input.txt').read()

count = 0

for pair in input.splitlines():
    pair1, pair2 = pair.split(",")
    min1, max1 = pair1.split("-")
    min2, max2 = pair2.split("-")

    min1 = int(min1)
    max1 = int(max1)
    min2 = int(min2)
    max2 = int(max2)

    if min1 <= min2 and max1 >= max2:
        # min1 fully contains 2
        count += 1
        print("1 contains 2", pair)
    elif min2 <= min1 and max2 >= max1:
        count += 1
        print("2 contains 1", pair)

print("answer", count)
