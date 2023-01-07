

def eval(expr):
    match expr.split(" "):
        case [i]: return int(i)
        case [x, '+', y]: return eval(variables[x]) + eval(variables[y])
        case [x, '-', y]: return eval(variables[x]) - eval(variables[y])
        case [x, '*', y]: return eval(variables[x]) * eval(variables[y])
        case [x, '/', y]: return eval(variables[x]) / eval(variables[y])

def eval_expr(expr, res):
    s = expr.split(" ")
    if s[0] == "X":
        return res

    x, op, y = s
    try:
        l = eval(variables[x])
        match op:
            case '+': return eval_expr(variables[y], res - l)
            case '*': return eval_expr(variables[y], res / l)
            case '-': return eval_expr(variables[y], l - res)
            case '/': return eval_expr(variables[y], l / res)

    except:
        r = eval(variables[y])
        match op:
            case '+': return eval_expr(variables[x], res - r)
            case '*': return eval_expr(variables[x], res / r)
            case '-': return eval_expr(variables[x], r + res)
            case '/': return eval_expr(variables[x], r * res)

variables = {}

with open("input.txt") as input:
    for line in filter(lambda l: l, input.read().split("\n")):
        name, value = [l.strip() for l in line.split(":")]
        variables[name] = value

###### Part 1 #######
print(eval(variables['root']))

###### Part 2 #######
left, _, right = variables['root'].split(" ")
variables["humn"] = "X"
try:
    print(eval_expr(variables[left], eval(variables[right])))
except:
    print(eval_expr(variables[right], eval(variables[left])))
    



