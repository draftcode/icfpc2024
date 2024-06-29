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

def main():
    vx = 0
    vy = 0
    px = 0
    py = 0
    solution = []
    for line in sys.stdin:
        if not line.strip():
            continue
        print(line, file=sys.stderr)
        (nx, ny) = map(int, line.split())
        (cx, cy) = (px + vx, py + vy)
        (kx, ky) = (nx - cx, ny - cy)
        solution.append(_MAP[(kx, ky)])
        vx += kx
        vy += ky
        px = nx
        py = ny
    print(''.join(str(i) for i in solution))


if __name__== '__main__':
    main()
