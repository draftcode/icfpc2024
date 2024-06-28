

import sys

basecode = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!"#$%&'()*+,-./:;<=>?@[\]^_`|~ \n"

encoded = ""
for c in sys.stdin.read():
    print(c)
    encoded += chr(basecode.index(c) + 33)
print(encoded)
