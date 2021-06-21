# i = '13,x,x,x,x,x,x,37,x,x,x,x,x,401,x,x,x,x,x,x,x,x,x,x,x,x,x,17,x,x,x,x,19,x,x,x,23,x,x,x,x,x,29,x,613,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,41'
i = '7,13,x,x,59,x,31,19'

buses = dict(x[::-1] for x in enumerate(i.split(',')))

if 'x' in buses:
    del buses['x']

# id, offset, satisfied
buses = [[int(k),v,False] for k,v in buses.items()]

print(buses)

id_max, off_max, _ = max(buses)

step = 1
candidate = id_max-off_max-step

# while not all buses are 'satisfied'
while not all(x[2] for x in buses):
    candidate += step
    for bus in (x for x in buses if not x[2]):
        _id, offset, satisfied = bus
        if (candidate+offset) % _id == 0:
            step *= _id
            bus[2] = True

    print(candidate, [x[0] for x in buses if not x[2]], step)

print(candidate)

"""
We take the biggest bus id (this is not mandatory, we could take any bus, but taking the biggest one speed up a little bit)
We can compute the first timestamp to satisfy this bus by:

```
candidate =  id-offset
```

so that

```
(candidate+offset) % id == 0
```

is True

Now, to keep satisfing this bus, we can only move of a `step` equal to the bus id.

Let's start with a smaller example.
'3,5,7,11'

We take 11 with offset 3.

```
candidate = 11 - 3 = 8
step = 11
```

We check all the buses against the candidate. We satisfy only 11.
The next candidate will be `8+11` = `19`
`19` is also satisfing 5 (offset 1) and (7 offset 2) because:

```
19+1 = 20 and 20 is multiple of 5
19+2 = 21 and 21 is multiple of 7
```

Now we just have to satisfy 3, BUT we can start moving at a faster speed, incrementing the `step`. WHY?

We are on position (19) that is good for 5,7, and 11. So we need to step by 11 to satisfy the bus with id 11, but also by 5 and by 7.
The only step that satisfy all the 3 of them is `5*7*11` = `385`

Let's visualize it with smaller numbers, 2,3, and 5.
  0                            30
2 | | | | | | | | | | | | | | | | | | |
3 |  |  |  |  |  |  |  |  |  |  |  |  |
5 |    |    |    |    |    |    |    |

the first step that align them all (except `0`) is `30` (`2*3*5`)

Now that we have candidate `19` and step `385` we go on until we satisfy all the buses left (in this case just `3`)

We get `404`, but is not multiple of `3` (offset 0).
Then we get `789` that is multple of `3` (offset 0). We have no more buses to satisfy, so this is our solution.


Taking the example from the README we have this progression:

candidate   buses still to satisfy    step
            [7, 13, 31, 19, 59]
55          [7, 13, 31, 19]             59
114         [7, 13, 31, 19]             59
173         [7, 13, 31, 19]             59
232         [7, 13, 31, 19]             59
291         [7, 13, 31, 19]             59
350         [31, 19]                  5369 (59 * 7 * 13)
5719        [31, 19]                  5369
11088       [31, 19]                  5369
16457       [31, 19]                  5369
21826       [31, 19]                  5369
27195       [31, 19]                  5369
32564       [31, 19]                  5369
37933       [31, 19]                  5369
43302       [31, 19]                  5369
48671       [31]                    102011 (59 * 7 * 13 * 19)
150682      [31]                    102011
252693      [31]                    102011
354704      [31]                    102011
456715      [31]                    102011
558726      [31]                    102011
660737      [31]                    102011
762748      [31]                    102011
864759      [31]                    102011
966770      [31]                    102011
1068781     []                     3162341 (59 * 7 * 13 * 19 * 31) but we don't need this anymore.
Solution is : 1068781
"""

