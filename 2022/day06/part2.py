input = """
mjqjpqmgbljsphdztnvjfqwrcgsmlb
"""

def solve(input) -> int:
    """
    for a string, find the first index of the
    substring of length 4 which is unique
    """

    for idx in range(14, len(input)):
        substring = input[idx - 13:idx]
        chars = set(substring)

        # print(idx, substring, chars)

        if len(chars) < 13:
            # set contains duplicates already
            continue

        # print(input[idx])

        if input[idx] in chars:
            # current already in set
            continue
        
        print('idx', idx)
        return idx + 1

    print('no result')
    return None

assert(solve("mjqjpqmgbljsphdztnvjfqwrcgsmlb") == 19)
assert(solve("bvwbjplbgvbhsrlpgdmjqwftvncz") == 23)
assert(solve("nppdvjthqldpwncqszvftbrmjlhg") == 23)
assert(solve("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg") == 29)
assert(solve("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw") == 26)

input = open("input.txt").read().strip()
print("answer")
print(solve(input))