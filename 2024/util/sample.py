import utils

class SomeSolution(utils.SolutionBase):
    def setup(self):
        self.add_example_part1('999', 333)
        self.add_example_part2('999', 111)

    def solve_part1(self, input):
        return int(input) // 3
    
    def solve_part2(self, input):
        return int(input) // 9

SomeSolution().run('example.txt')
