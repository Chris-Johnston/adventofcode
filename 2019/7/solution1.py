# part 1
# figured it might be more straightforward to just
# call the rust program as a child process

import subprocess
import itertools

intcode = "/home/chris/Git/adventofcode/2019/7/day7/target/release/day7"

input = [0, 1, 2, 3, 4]

for p in itertools.permutations(input):
    print(p)

def call_intcode(val1, val2):
    """
    calls the intcode with the given input
    and reads the output
    """
    proc = subprocess.run([intcode],
        input=f'{val1}\n{val2}\n',
        encoding='ascii',
        stdout=subprocess.PIPE)
    return int(proc.stdout)

print(call_intcode(0, 4))

def check_setting(settings):
    signal = 0
    for amp in settings:
        signal = call_intcode(amp, signal)
    print(f'{settings} -> {signal}')
    return signal

highest_signal = 0
# assert(check_setting([4,3,2,1,0]) == 43210)

for x in itertools.permutations(input):
    result = check_setting(x)
    if result > highest_signal:
        highest_signal = result

print(highest_signal)

