from dataclasses import dataclass

input = """
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
""".strip()

input = open('input.txt').read().strip()

class Monkey():
    items = []
    # operation_expression = ""
    test_expression = 0
    true_monkey = 0
    false_monkey = 0

    operand = ''
    operand_lhs = ''
    operand_rhs = ''

    eval_count = 0

    def __str__(self) -> str:
        return f"Monkey(items: {self.items}, op: {self.operand})"

    def parse_operation(self, operation: str):
        self.operand_lhs, self.operand, self.operand_rhs = operation[6:].split() # trim off "new = "
    
    def evaluate_items(self):
        # returns list of tuples where the first value is the monkey to send to
        # and the value is the worry level
        result = []
        for item in self.items:
            self.eval_count += 1
            # evaluate the expression

            result_val = 0

            lhs_value = 0
            rhs_value = 0
            if self.operand_lhs == "old":
                lhs_value = item
            else:
                lhs_value = int(self.operand_rhs)

            if self.operand_rhs == "old":
                rhs_value = item
            else:
                rhs_value = int(self.operand_rhs)

            if self.operand == "+":
                result_val = lhs_value + rhs_value
            elif self.operand == "-":
                result_val = lhs_value - rhs_value
            elif self.operand == "*":
                result_val = lhs_value * rhs_value
            elif self.operand == "/":
                result_val = int(lhs_value // rhs_value)
            
            # divide result by 3 and floor
            result_val = result_val // 3

            # check
            if result_val % self.test_expression == 0:
                result.append((self.true_monkey, result_val))
            else:
                result.append((self.false_monkey, result_val))
        self.items = []
        return result



line_idx = 0
input_lines = iter(input.splitlines())

monkeys = []

while True:
    # line = input_lines[line_idx]
    # line_idx += 1
    try:
        line = next(input_lines)
    except StopIteration:
        break

    if line == "":
        continue
    elif line.startswith("Monkey"):
        # parse the monkey, ignore the index because it starts at 0
        m = Monkey()

        line = next(input_lines).strip()
        assert line.startswith("Starting items")
        _, items = line.split(":")
        m.items = list(map(lambda x: int(x), items.split(",")))

        line = next(input_lines).strip()
        assert line.startswith("Operation")
        _, operation = line.split(":")
        m.parse_operation(operation)

        line = next(input_lines).strip()
        assert line.startswith("Test")
        test = line[len("Test: divisible by "):]
        m.test_expression = int(test)

        line = next(input_lines).strip()
        assert line.startswith("If true")
        throw_to = line[len("If true: throw to monkey "):]
        m.true_monkey = int(throw_to)

        line = next(input_lines).strip()
        assert line.startswith("If false")
        throw_to = line[len("If false: throw to monkey "):]
        m.false_monkey = int(throw_to)

        monkeys.append(m)

print("monkeys", [str(m) for m in monkeys])

for round in range(20):
    print("Round", round)
    for m in monkeys:
        updates = m.evaluate_items()
        for to_monkey, worry in updates:
            # print("Adding item with worry level", worry, "to monkey", to_monkey)
            monkeys[to_monkey].items.append(worry)

    for i, m in enumerate(monkeys):
        print('Monkey', i, ":", m.items)

# find the top eval_count
eval_counts = []
for m in monkeys:
    eval_counts.append(m.eval_count)

eval_counts.sort()

print(eval_counts)
answer = eval_counts[-1] * eval_counts[-2]
print('answer', answer)