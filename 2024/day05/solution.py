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


class Day5(SolutionBase):

    def setup(self):

        example_input = """
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"""

        self.add_example_part1(example_input, 143)
        self.add_example_part2(example_input, 123)

    def parse_input(self, input):

        # print(input)

        # key is the number which must prcee other cards
        self.ordering_rules = {}
        # feels like I need to solve this using a binary tree, but maybe I can do it via a bad bubble sort

        self.page_number_sequences = []

        self.initially_invalid_sequences = []

        for line in input.splitlines():
            line = line.strip()

            if '|' in line:
                before, after = line.split('|')
                before = int(before)
                after = int(after)

                l = self.ordering_rules.get(before, set())
                l.add(after)
                self.ordering_rules[before] = l

            if ',' in line:
                self.page_number_sequences.append(
                    [int(x) for x in line.split(',')]
                )
        
        print('parsed ordering rules:', self.ordering_rules)
        print('page number sequences:', self.page_number_sequences)


    def solve_part1(self, input):

        self.parse_input(input)
        answer = 0

        # just validate each of the sequences
        for sequence in self.page_number_sequences:
            valid = True
            for index in range(len(sequence) - 1):
                current = sequence[index]
                next = sequence[index + 1]

                # if current in self.ordering_rules and next in self.ordering_rules[current]:
                #     pass
                # else:
                #     valid = False
                #     break

                if next in self.ordering_rules and current in self.ordering_rules[next]:
                    # found a counter example
                    print('not valid because next', next, 'was before current', current)
                    valid = False
                    break
            
            if valid:
                print('valid sequence', sequence)

                # find the middle element
                middle_element = sequence[len(sequence) // 2]
                print('middle element was', middle_element)
                answer += middle_element
            else:
                self.initially_invalid_sequences.append(sequence)

        return answer
    
    def solve_part2(self, input):
        # self.parse_input(input)
        answer = 0

        print('working on initially valid sequences', self.initially_invalid_sequences)

        # only need to fix the wrong sequences
        for sequence in self.initially_invalid_sequences:

            while True:

                valid = True

                for index in range(len(sequence) - 1):
                    current = sequence[index]
                    next = sequence[index + 1]

                    # going to just swap 
                    if next in self.ordering_rules and current in self.ordering_rules[next]:
                        # found a counter example
                        print('not valid because next', next, 'was before current', current)
                        valid = False

                        # swap the two and see what happens
                        sequence[index] = next
                        sequence[index + 1] = current

                        break

                if valid:
                    break
            
            if valid:
                print('valid sequence', sequence)

                # find the middle element
                middle_element = sequence[len(sequence) // 2]
                print('middle element was', middle_element)
                answer += middle_element
        return answer

solution = Day5()
solution.setup()

assert solution.test_part1()
assert solution.test_part2()

solution.load_input()
solution.run_part1()

solution.run_part2()
# 4480