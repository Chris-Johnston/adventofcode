example_input = """
A Y
B X
C Z
"""

input = example_input.strip()

moves = [ 'rock', 'paper', 'scissors' ]

input = open("input.txt", 'r').read()

def to_plain_english(action):
    if action == 'A': return 'rock'
    if action == 'B': return 'paper'
    if action == 'C': return 'scissors'

    if action == 'X': return 'lose'
    if action == 'Y': return 'draw'
    if action == 'Z': return 'win'

def determine_my_action(opponent, result):
    if result == 'draw':
        return opponent
    opp_idx = moves.index(opponent)
    if result == 'win':
        return moves[(opp_idx + 1) % 3]
    return moves[(opp_idx - 1) % 3]

def get_turn_points(action):
    if action == 'rock': return 1
    if action == 'paper': return 2
    if action == 'scissors': return 3

    print('dunno what', action)

def is_draw(opponent, mine):
    return opponent == mine

def is_win(opponent, mine):
    # smarter way to do this is to compare indexes into the 
    # moves list, whatever
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
    # my_action will be one of 'win' 'lose' 'draw' here

    my_action = determine_my_action(opponent_action, my_action)

    score += get_turn_points(my_action)

    score += eval_round(opponent_action, my_action)

print(score)
