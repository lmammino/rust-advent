import sys

first = open(sys.argv[1])
second = open(sys.argv[2])

data = {}
delta = {}
k = None

for line in second:
    if line.startswith("##"):
        k = line[2 : line.rfind("-")]
    elif line.startswith("=="):
        _, _, _, counter = line.split()
        counter = int(counter.replace(",", ""))
        data[k] = counter

for line in first:
    if line.startswith("##"):
        k = line[2 : line.rfind("-")]
    elif line.startswith("==") and k in data:
        _, _, _, counter = line.split()
        counter = int(counter.replace(",", ""))
        delta[k] = (data[k] - counter) * 100 / counter

print(
    """## Valgrind Benchmarks

<details>
    <summary>Negative deltas means faster code</summary>  
    <pre>                      
"""
)
for k in data:
    if k in delta:
        print("{} {} ({} %)\n".format(k, data[k], "{:+.02f}".format(delta[k])))
print("</pre></details>")
