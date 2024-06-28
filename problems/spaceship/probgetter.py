import urllib
import urllib.request
import os
import time

token = os.environ["API_TOKEN"]

_ENDPOINT = "https://boundvariable.space/communicate"
_ENCODE_MAP = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\]^_`|~ \n"


def encode_str(s: str) -> str:
    return "S" + "".join(chr(_ENCODE_MAP.index(c) + 33) for c in s)


def decode_str(s: str) -> str:
    assert s.startswith("S")
    return "".join(_ENCODE_MAP[ord(c) - 33] for c in s[1:])


for i in [4] + list(range(6, 25)):
    text = f"get spaceship{i}"
    exp = encode_str(text)

    req = urllib.request.Request(_ENDPOINT, data=exp.encode(), method="POST")
    req.add_header("Authorization", f"Bearer {token}")
    with urllib.request.urlopen(req) as response:
        res = response.read().decode()

    with open(f"spaceship{i}.txt", "w") as ofh:
        if res.startswith("S") and " " not in res:
            print(decode_str(res), file=ofh)
        else:
            print(res, file=ofh)
    time.sleep(10)
