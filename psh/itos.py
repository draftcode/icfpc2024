import sys

_ENCODE_MAP = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n"

def main():
    for line in sys.stdin:
        i = int(line.strip())
        result = []
        while i:
            m = i % 94
            result.append(_ENCODE_MAP[m])
            i //= 94
        print(''.join(reversed(result)))

if __name__ == '__main__':
    main()
