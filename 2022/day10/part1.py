from dataclasses import dataclass

input = """
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
""".strip()

# input = open('input.txt').read().strip()

@dataclass
class Cpu():
    cycles = 0
    x_register = 1

    # for each cycle, log the value of the register
    x_register_log = [1]

    def noop(self):
        # self.cycles += 1
        self.increment_cycle()

    def addx(self, val):
        # self.cycles += 2
        self.increment_cycle()
        self.increment_cycle()
        self.x_register += val

    def increment_cycle(self):
        self.cycles += 1
        self.x_register_log.append(self.x_register)

    def get_signal_strength(self, cycle):
        return cycle * self.x_register_log[cycle]
    
    def get_pixel_x(self, cycle):
        return self.x_register_log[cycle]
    
    def get_pixel_active(self, x):
        x_reg = self.get_pixel_x(x)
        return x - 1 <= x_reg <= x + 1

cpu = Cpu()

for op in input.splitlines():
    if op == "noop":
        cpu.noop()
    elif op.startswith("addx"):
        _, num = op.split()
        num = int(num)
        cpu.addx(num)

print("finished running through", cpu.cycles, "cycles")

# target = [20, 60, 100, 140, 180, 220]
# sum = 0
# for t in target:
#     sum += cpu.get_signal_strength(t)

# print("answer", sum)

for x in range(240):
    if cpu.get_pixel_active(x):
        print('#', end='')
    else:
        print('.', end='')
    if x % 40 == 0:
        print()
