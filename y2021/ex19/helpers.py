def rotations(scanner):
    x,y,z = zip(*scanner)

    yield (tuple(zip(x,y,z)))
    yield (tuple(zip(x,z,y)))
    yield (tuple(zip(y,x,z)))
    yield (tuple(zip(y,z,x)))
    yield (tuple(zip(z,x,y)))
    yield (tuple(zip(z,y,x)))

    z = [-a for a in z]
    yield (tuple(zip(x,y,z)))
    yield (tuple(zip(x,z,y)))
    yield (tuple(zip(y,x,z)))
    yield (tuple(zip(y,z,x)))
    yield (tuple(zip(z,x,y)))
    yield (tuple(zip(z,y,x)))

    z = [-a for a in z]
    y = [-a for a in y]
    yield (tuple(zip(x,y,z)))
    yield (tuple(zip(x,z,y)))
    yield (tuple(zip(y,x,z)))
    yield (tuple(zip(y,z,x)))
    yield (tuple(zip(z,x,y)))
    yield (tuple(zip(z,y,x)))
        
    z = [-a for a in z]
    yield (tuple(zip(x,y,z)))
    yield (tuple(zip(x,z,y)))
    yield (tuple(zip(y,x,z)))
    yield (tuple(zip(y,z,x)))
    yield (tuple(zip(z,x,y)))
    yield (tuple(zip(z,y,x)))

    x = [-a for a in x]
    yield (tuple(zip(x,y,z)))
    yield (tuple(zip(x,z,y)))
    yield (tuple(zip(y,x,z)))
    yield (tuple(zip(y,z,x)))
    yield (tuple(zip(z,x,y)))
    yield (tuple(zip(z,y,x)))

    z = [-a for a in z]
    yield (tuple(zip(x,y,z)))
    yield (tuple(zip(x,z,y)))
    yield (tuple(zip(y,x,z)))
    yield (tuple(zip(y,z,x)))
    yield (tuple(zip(z,x,y)))
    yield (tuple(zip(z,y,x)))

    z = [-a for a in z]
    y = [-a for a in y]
    yield (tuple(zip(x,y,z)))
    yield (tuple(zip(x,z,y)))
    yield (tuple(zip(y,x,z)))
    yield (tuple(zip(y,z,x)))
    yield (tuple(zip(z,x,y)))
    yield (tuple(zip(z,y,x)))
        
    z = [-a for a in z]
    yield (tuple(zip(x,y,z)))
    yield (tuple(zip(x,z,y)))
    yield (tuple(zip(y,x,z)))
    yield (tuple(zip(y,z,x)))
    yield (tuple(zip(z,x,y)))
    yield (tuple(zip(z,y,x)))


def sum3(a,b):
    return a[0]+b[0], a[1]+b[1], a[2]+b[2]

def diff3(a,b):
    return a[0]-b[0], a[1]-b[1], a[2]-b[2]

def matches(scanner0, scanner1):
    for i1, s1 in enumerate(rotations(scanner1)):
        cnt = {}
        for p0 in scanner0:
            for p1 in s1:
                c = diff3(p0,p1)
                cnt[c] = cnt.get(c,0) + 1
        m = [k for k,v in cnt.items() if v>=12]
        if m: 
            return tuple(sum3(x, m[0]) for x in s1), m[0]
    return None, None