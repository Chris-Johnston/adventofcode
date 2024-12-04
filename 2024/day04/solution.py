# copy and pasting my boilerplate until I try changing this

from abc import ABC, abstractmethod
import os.path

# sample framework for advent of code setup
class SolutionBase(ABC):

    def __init__(self):

        # list of tuples which have example input and the expected answer
        self.example_inputs_part1 = []
        self.example_inputs_part2 = []

        self.answer_part1 = None
        self.answer_part2 = None

        self.real_input = None
        pass

    def load_input(self, filename = "input.txt"):
        if os.path.isfile(filename):
            with open(filename) as f:
                self.real_input = f.read().strip()
        else:
            assert False, "file does not exist"

    # test orchestration stuff

    def run(self, filename = "input.txt"):
        print("Setting up")
        self.setup()

        try:
            print(f"Reading input from file {filename}")

            if os.path.isfile(filename):
                with open(filename) as f:
                    self.real_input = f.read().strip()
            else:
                print(f"file {filename} doesn't exist, skipping")
                self.real_input = None

            print("Running part 1...")
            self.run_part1()    
            
            print("Running part 2...")
            self.run_part2()
                
            print("Done")
        except Exception as e:
            print("Uncaught exception", e)
        finally:
            print(f"Part 1: {self.answer_part1}")
            print(f"Part 2: {self.answer_part2}")
        
        return self.answer_part1, self.answer_part2
    
    def test_part1(self): # true if part 1 should run, otherwise false
        print("test part 1", self.example_inputs_part1)
        return self.check_examples(self.example_inputs_part1, self.solve_part1)
    
    def test_part2(self):
        return self.check_examples(self.example_inputs_part2, self.solve_part2)
    
    def run_part1(self):
        if self.real_input is None:
            print("missing input file, skipping part 1")
            return
        self.answer_part1 = self.solve_part1(self.real_input)
        print("Answer Part 1:", self.answer_part1)

    def run_part2(self):
        if self.real_input is None:
            print("missing input file, skipping part 2")
            return
        self.answer_part2 = self.solve_part2(self.real_input)
        print("Answer Part 2:", self.answer_part2)
            
    def check_examples(self, example_inputs_list, solve_function):
        print(f"Checking examples... ({len(example_inputs_list)}))")
        count = 0
        passing = True
        for example_input, expected_answer in example_inputs_list:
            count += 1
            print(f"#{count} Check example #{count} (expecting answer of {expected_answer})")

            actual_answer = solve_function(example_input)
            
            if expected_answer != actual_answer:
                print(f"#{count} FAIL: Got {actual_answer} expected {expected_answer}")
                passing = False
            else:
                print(f"#{count} Passed, got expected answer of {actual_answer} == {expected_answer}")

        return passing
    
    # setup stuff

    @abstractmethod
    def setup(self):
        pass

    def add_example_part1(self, example_input: str, example_answer):
        self.example_inputs_part1.append((example_input, example_answer))

    def add_example_part2(self, example_input, example_answer):
        self.example_inputs_part2.append((example_input, example_answer))

    # implement this part

    @abstractmethod
    def solve_part1(self, input: str):
        pass

    @abstractmethod
    def solve_part2(self, input: str):
        pass

# actual implementation

class Day4(SolutionBase):

    def setup(self):

        example_text = """
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"""

        self.add_example_part1(example_text, 18)

        self.add_example_part2(example_text, 9)
        
    def parse_input(self, input):
        self.width = 0
        self.height = 0

        # coordinates start from top left and increase x going to right
        # and y going down

        # and indexed using coordinates[y][x]

        self.coordinates = []

        for line in input.strip().splitlines():
            line = line.strip()
            self.width = len(line)

            self.coordinates.append(line)
            self.height += 1
        
        print("width", self.width, "height", self.height)
        
        # debug
        for y in range(self.height):
            for x in range(self.width):
                print(self.coordinates[y][x], end='')
            print()

    def solve_part1(self, input):
        self.parse_input(input)

        # contains set of tuples
        # where contains (x, y, dx, dy)
        matches = set()

        # find all of the candidates (all instances of the letter X)
        # contains tuple of (x, y)
        candidates = set()


        for y in range(self.height):
            for x in range(self.width):
                letter = self.coordinates[y][x]
                if letter == 'X':
                    candidates.add((x, y))

        print(f"found {len(candidates)} candidates to start from")

        # for each candidate, walk each of the possible directions to find the rest of the string 'MAS'
        directions = [
            # cardinal directions
            (-1, 0), # left
            (1, 0), # right
            (0, 1), # down
            (0, -1), # up
            # diagonal directions
            (-1, -1),
            (-1, 1),
            (1, -1),
            (1, 1),
        ]

        # used for debugging
        visited_set = set()

        for candidate in candidates:
            start_x, start_y = candidate
            print("checking candidate", candidate)

            for direction in directions:
                print("\t checking direction", direction)
                dx, dy = direction

                current_x = start_x
                current_y = start_y

                current_breadcrumbs = [
                    (current_x, current_y)
                ]

                # visited_set.add((current_x, current_y))

                # walk in the direction, and match the next char in the string
                for c in 'MAS':
                    current_x += dx
                    current_y += dy

                    # check bounds first
                    if not (0 <= current_x < self.width):
                        print("\t out of bounds x", current_x)
                        break
                    if not (0 <= current_y < self.height):
                        print("\t out of bounds y", current_y)
                        break

                    # compare char
                    if c != self.coordinates[current_y][current_x]:
                        print("\t wrong char", self.coordinates[current_y][current_x])
                        break

                    print("\t matched character", c, (current_x, current_y))
                    # visited_set.add((current_x, current_y))
                    current_breadcrumbs.append((current_x, current_y))

                    # reached the end of the string, so this is a match
                    if c == 'S':
                        print("\t matched:", (start_x, start_y, dx, dy))
                        matches.add((start_x, start_y, dx, dy))

                        # for debugging, only show the visited set for the matches
                        for breadcrumb in current_breadcrumbs:
                            visited_set.add(breadcrumb)

        # debug all of the visited spots

        for y in range(self.height):
            for x in range(self.width):
                # if (x, y) in candidates:
                #     print('x', end='')
                if not (x, y) in visited_set:
                    print('.', end='')
                else:
                    print(self.coordinates[y][x], end='')
            print()
        
        answer = len(matches)
        return answer

    def solve_part2(self, input):
        self.parse_input(input)

        # contains set of tuples
        # where contains (x, y)
        matches = set()

        # find all of the candidates (all instances of the letter X)
        # contains tuple of (x, y)
        candidates = set()

        for y in range(self.height):
            for x in range(self.width):
                letter = self.coordinates[y][x]
                if letter == 'A':
                    candidates.add((x, y))

        print(f"found {len(candidates)} candidates to start from")

        # for each candidate, walk each of the directions
        # this is ordered such that index 0 should have the opposite value of index 1
        # (if left up is 'm', right down should be 's')
        # and same with the others
        directions = [
            # cardinal directions
            # (-1, 0), # left
            # (1, 0), # right
            # (0, 1), # down
            # (0, -1), # up
            # diagonal directions
            (-1, -1), # left up
            (1, 1), # right down
            (1, -1), # right up
            (-1, 1), # left down
            
        ]

        # used for debugging
        visited_set = set()

        for candidate in candidates:
            start_x, start_y = candidate
            print("checking candidate", candidate)

            current_x = start_x
            current_y = start_y

            current_breadcrumbs = [
                (current_x, current_y)
            ]

            pairings = [
                set(),
                set()
            ]

            direction_count = 0

            for direction in directions:
                print("\t checking direction", direction)
                dx, dy = direction

                current_x = start_x + dx
                current_y = start_y + dy

                # check bounds first
                if not (0 <= current_x < self.width):
                    print("\t out of bounds x", current_x)
                    break
                if not (0 <= current_y < self.height):
                    print("\t out of bounds y", current_y)
                    break

                current_breadcrumbs.append((current_x, current_y))

                current_char = self.coordinates[current_y][current_x]
                pairings[direction_count // 2].add(current_char)

                direction_count += 1
            
            # verify that both pairing sets contain "MS"
            valid = True
            for pair in pairings:
                # print('pair', pair)
                if not ('M' in pair and 'S' in pair and len(pair) == 2):
                    valid = False
            
            if valid:
                print('\t match! ', (current_x, current_y))
                matches.add((start_x, start_y))

                for breadcrumb in current_breadcrumbs:
                    visited_set.add(breadcrumb)

        # debug all of the visited spots

        for y in range(self.height):
            for x in range(self.width):
                # if (x, y) in candidates:
                #     print('x', end='')
                if not (x, y) in visited_set:
                    print('.', end='')
                else:
                    print(self.coordinates[y][x], end='')
            print()
        
        answer = len(matches)
        return answer

        
solution = Day4()
solution.setup()

assert solution.test_part1()

solution.load_input()
solution.run_part1()

assert solution.test_part2()

solution.run_part2()