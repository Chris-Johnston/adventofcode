input = """
mjqjpqmgbljsphdztnvjfqwrcgsmlb
"""

def solve(input) -> int:
    """
    for a string, find the first index of the
    substring of length 4 which is unique
    """

    for idx in range(4, len(input)):
        # print(idx, input[idx - 3:idx])
        chars = set(input[idx - 3:idx])

        # print(chars)

        if len(chars) < 3:
            # set contains duplicates already
            continue

        # print(input[idx])

        if input[idx] in chars:
            # current already in set
            continue
        
        # print('idx', idx)
        return idx + 1
    return None

assert(solve("mjqjpqmgbljsphdztnvjfqwrcgsmlb") == 7)
assert(solve("bvwbjplbgvbhsrlpgdmjqwftvncz") == 5)
assert(solve("nppdvjthqldpwncqszvftbrmjlhg") == 6)
assert(solve("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg") == 10)
assert(solve("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw") == 11)

input = open("input.txt").read().strip()
print("answer")
print(solve(input))