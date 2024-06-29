import sys

_MAP = {
    (1, 1): 9,
    (0, 1): 8,
    (-1, 1): 7,
    (1, 0): 6,
    (0, 0): 5,
    (-1, 0): 4,
    (1, -1): 3,
    (0, -1): 2,
    (-1, -1): 1,
}

def sign(v):
    if v == 0:
        return 0
    elif v < 0:
        return -1
    else:
        return 1


def plan(px, nx):
    s = sign(nx - px)
    d = abs(nx - px)

    if d == 0:
        return []

    k = 1
    while True:
        if k % 2:
            x = ((k + 1) // 2) * ((k + 1) // 2)
        else:
            x = (k // 2) * (k // 2 + 1)
        if d <= x:
            break
        k += 1

    if k % 2:
        p = ([1] * ((k + 1)// 2)) + ([-1] * ((k + 1) // 2))
    else:
        p = ([1] * (k // 2)) + [0] + ([-1] * (k // 2))

    offset = 0 if k % 2 else 1
    for i in range(x - d):
        if k % 2:
            p[(k + 1) // 2 - 1 - i] -= 1
            p[(k + 1) // 2 - i] += 1
        else:
            p[k // 2 - 1 - i] -= 1
            p[k // 2 - i] += 1

    if s < 0:
        p = [x * -1 for x in p]
    return p


def main():
    px, py = 0, 0
    solution = []
    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue
        (nx, ny) = map(int, line.split())
        plan_x = plan(px, nx)
        plan_y = plan(py, ny)

        length = max(len(plan_x), len(plan_y))
        plan_x += [0] * (length - len(plan_x))
        plan_y += [0] * (length - len(plan_y))

        for x, y in zip(plan_x, plan_y):
            solution.append(_MAP[(x, y)])
        (px, py) = (nx, ny)

    print(''.join(str(i) for i in solution))


if __name__ == '__main__':
    main()
