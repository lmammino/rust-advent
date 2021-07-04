import sys

first = open(sys.argv[1])
second = open(sys.argv[2])

data = {}
delta = {}
k = None

for line in second:
    if line.startswith('##'):
        k = line[:k.rfind('-')]
    elif line.startswith('=='):
        _, _, _, counter = line.split()
        counter = int(counter.replace(',',''))
        data[k] = counter

for line in first:
    if line.startswith('##'):
        k = line[:k.rfind('-')]
    elif line.startswith('==') and k in data:
        _, _, _, counter = line.split()
        counter = int(counter.replace(',',''))
        delta[k] = (data[k] - counter) * 100 / counter

print("Negative deltas means faster code\n\n")
for k in data:
    print("{} {} ({} %)\n".format(k, data[k], '{:+.02f}'.format(delta[k]) if k in delta else 'N/A'))