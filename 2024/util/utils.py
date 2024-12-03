from abc import ABC, abstractmethod
import os.path

# sample framework for advent of code setup
class SolutionBase(ABC):

    def __init__(self):

        # list of tuples which have example input and the expected answer
        self.example_inputs_part1 = []
        self.example_inputs_part2 = []

        # in case one of the solutions is really slow
        self.skip_part1 = False
        self.skip_part2 = False

        self.skip_tests_part1 = False
        self.skip_tests_part2 = False
        
        self.answer_part1 = None
        self.answer_part2 = None

        self.real_input = None
        pass

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
            if self.test_part1():
                self.run_part1()
            
            print("Running part 2...")
            if self.test_part2():
                self.run_part2()
            
            print("Done")
        except Exception as e:
            print("Uncaught exception", e)
        finally:
            print(f"Part 1: {self.answer_part1}")
            print(f"Part 2: {self.answer_part2}")
        
        return self.answer_part1, self.answer_part2
    
    def test_part1(self): # true if part 1 should run, otherwise false
        if not self.skip_part1:
            if self.skip_tests_part1:
                return True
            return self.check_examples(self.example_inputs_part1, self.solve_part1)
        print("Skipping part 1")
        return False
    
    def test_part2(self):
        if not self.skip_part2:
            if self.skip_tests_part2:
                return True
            return self.check_examples(self.example_inputs_part2, self.solve_part2)
        print("Skipping part 2")
        return True
    
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
        print("Checking examples...")
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
                print(f"#{count} Pass")

        return passing
    
    # setup stuff

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