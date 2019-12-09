# part 2
# figured it might be more straightforward to just
# call the rust program as a child process

import subprocess
import itertools
import time

intcode = "/home/chris/Git/adventofcode/2019/7/day7/target/release/day7"

input = [5,6,7,8,9]
# input = [0, 1, 2, 3, 4]

for p in itertools.permutations(input):
    print(p)

def call_intcode(val1, val2):
    """
    calls the intcode with the given input
    and reads the output
    """
    print(f'in {val1}\n{val2}\n')
    # proc = subprocess.run([intcode],
    #     input=f'{val1}\n{val2}\n',
    #     encoding='ascii',
    #     stdout=subprocess.PIPE)
    
    if proc.returncode == 123:
        return None
    return int(proc.stdout)

# print(call_intcode(0, 4))


def check_setting(settings):
    processes = []
    signal = 0
    counter = 0
    for x in range(5):
        processes.append(
            subprocess.Popen([intcode], stdin=subprocess.PIPE, stdout=subprocess.PIPE,shell=False)
        )

        proc = processes[x]

        # send the first code
        proc.stdin.write(f'{settings[x]}\n{signal}\n'.encode('utf-8'))
        proc.stdin.flush()

        line = proc.stdout.readline()
        if line:
            if isinstance(line, bytes):
                line = line.decode('utf-8')
            if line.strip() == "EXIT":
                print("EXIT ------------------")
                return signal
            signal = int(line)
    while True:
        for x in range(5):
            proc = processes[x]
            try:
                proc.stdin.write(f'{signal}\n'.encode('utf-8'))
            except BrokenPipeError:
                if x == 4:
                    return signal
                continue
            # send the first code
            try:
                proc.stdin.flush()
            except BrokenPipeError:
                if x == 4:
                    return signal
                continue

            line = proc.stdout.readline()
            if line:
                if isinstance(line, bytes):
                    line = line.decode('utf-8')
                if line.strip() == "EXIT":
                    print("EXIT ------------------")
                    if x == 4:
                        return signal
                    continue
                signal = int(line)
            else:
                if x == 4:
                    return signal
                continue
    

# def check_setting(settings):
#     signal = 0
#     first = True
#     # proc = subprocess.run([intcode],
#     #         stdin=subprocess.PIPE,
#     #         stdout=subprocess.PIPE)
#     proc = subprocess.Popen([intcode], stdin = subprocess.PIPE, stdout = subprocess.PIPE, shell = False)
#     while True:
#         for idx, amp in enumerate(settings):
#             # signal = call_intcode(amp, signal)

#             if idx == 0 and first:
#                 # print(f'sending {amp} {signal}')
#                 # proc.stdin.write(f'{amp}\n{signal}\n'.encode('ascii'))
#                 input = f'{amp}\n{signal}\n'.encode('utf-8')
#                 first = False
#             else:
#                 # print(f'sending {amp}')
#                 # proc.stdin.write(f'{amp}\n'.encode('ascii'))
#                 # TODOO
#                 input = f'{signal}\n'.encode('utf-8')
#             # result = proc.stdout.readlines()
#             print(f'input {input}')
#             proc.stdin.write(input)
#             try:
#                 proc.stdin.flush()
#             except BrokenPipeError:
#                 return signal
#             # time.sleep(0.01)
#             # print(f'reading... {proc.stdout.readable()}')
#             line = proc.stdout.readline()
#             if line:
#                 if isinstance(line, bytes):
#                     line = line.decode('utf-8')
#                 if line.strip() == "EXIT":
#                     print("EXIT ------------------")
#                     return signal
#                 # print(f"Signal {line}")
#                 signal = int(line)
#             else:
#                 return signal

#             # print(f"return {proc.poll()}")
#             # # out = proc.stdout.readline()
#             # out, _ = proc.communicate(timeout=0.5)
#             # print('read')
#             # if out is not None:
#             #     result = int(out.decode('utf-8'))
#             # else:
#             #     break
#             # print(f'result {result}')
#             # result = call_intcode(amp, signal)
#             # if result is None:
#             #     print(f'{settings} -> {signal}')
#             #     return signal
#             # else:
#             #     signal = result
#             # if proc.returncode == 0:
#             #     return signal
#             # print('aa')
#     proc.terminate()
#     return signal

highest_signal = 0
# assert(check_setting([4,3,2,1,0]) == 43210)

for x in itertools.permutations(input):
    result = check_setting(x)
    print(result)
    if result > highest_signal:
        print(f"new high of {result}")
        highest_signal = result

print(highest_signal)

# 9 - amp a
# 0 - start val
# 5 - input to b val
