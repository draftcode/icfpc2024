import sys

basecode = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!"#$%&'()*+,-./:;<=>?@[]^_`|~ \n"

decoded
for c in sys.stdin:
    decoded += basecode[ord(c) - 33]
print(decoded)
