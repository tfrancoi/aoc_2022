import pprint

from functools import cmp_to_key

def check_pair(p1, p2):
    ordered = -1
    for i in range(0, len(p1)):
        if isinstance(p1[i], list):
            if len(p2) > i and isinstance(p2[i], list):
                ordered = check_pair(p1[i], p2[i])
            elif len(p2) > i:
                ordered = check_pair(p1[i], [p2[i]])
            else:
                ordered = 1
        elif len(p2) > i and isinstance(p2[i], list):
            ordered = check_pair([p1[i]], p2[i])
        elif len(p2) > i:
            ordered = -1 if (p1[i] < p2[i]) else (1 if (p1[i] > p2[i]) else 0)
        else:
            ordered = 1
        if ordered:
            return ordered

    if not ordered and len(p2) > len(p1):
        return -1
    return ordered


def aoc1():
    with open('input.txt', 'r') as input:
        pairs = [p.split("\n") for p in input.read().split("\n\n")]
        pairs = [(eval(p[0]), eval(p[1])) for p in pairs]
    f = cmp_to_key(check_pair)
    sum = 0
    for i, pair in enumerate(pairs):
        print(i+1, pair, check_pair(pair[0], pair[1]))
        if check_pair(pair[0], pair[1]) == -1:
            sum += i+1
    print(sum)

def aoc2():
    print("=================")
    with open('input.txt', 'r') as input:
        lines = list(filter(lambda l: l, input.read().split("\n")))
        lines = [eval(p) for p in lines]
        lines.extend([[[2]], [[6]]])
        mul = 1
        for i, l in enumerate(sorted(list(lines), key=cmp_to_key(check_pair))):
            if l == [[2]] or l == [[6]]:
                mul *= (i + 1)
        print(mul)
aoc1()
aoc2()