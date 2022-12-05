example_input = """
A Y
B X
C Z
"""

input = example_input.strip()

input = open("input.txt", 'r').read()

def to_plain_english(action):
    if action == 'A': return 'rock'
    if action == 'B': return 'paper'
    if action == 'C': return 'scissors'

    if action == 'X': return 'rock'
    if action == 'Y': return 'paper'
    if action == 'Z': return 'scissors'

def get_turn_points(action):
    if action == 'rock': return 1
    if action == 'paper': return 2
    if action == 'scissors': return 3

def is_draw(opponent, mine):
    return opponent == mine

def is_win(opponent, mine):
    if opponent == 'rock' and mine == 'paper':
        return True
    if opponent == 'paper' and mine == 'scissors':
        return True
    if opponent == 'scissors' and mine == 'rock':
        return True
    return False

def eval_round(opponent, mine):
    if is_draw(opponent, mine):
        return 3
    if is_win(opponent, mine):
        return 6
    return 0 # lose

score = 0

for line in input.splitlines():
    actions = line.split()

    opponent_action, my_action = actions

    opponent_action = to_plain_english(opponent_action)
    my_action = to_plain_english(my_action)

    score += get_turn_points(my_action)

    score += eval_round(opponent_action, my_action)

print(score)
