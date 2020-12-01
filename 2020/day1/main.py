import matplotlib.pyplot as plt

# show a histogram of data to see if this
# informs the results
# it kinda does?
# most data is > 1000
data = []
with open("input_part1.txt") as d:
    for line in d:
        data.append(int(line))

print("Data", data)

plt.hist(data, bins=100)
plt.show()