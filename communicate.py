#!/usr/bin/env python3

import os
import urllib.request
import urllib.parse

_ENDPOINT = "https://boundvariable.space/communicate"
_ENCODE_MAP = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\]^_`|~ \n"


def encode_str(s: str) -> str:
    return "S" + "".join(chr(_ENCODE_MAP.index(c) + 33) for c in s)


def decode_str(s: str) -> str:
    assert s.startswith("S")
    return "".join(_ENCODE_MAP[ord(c) - 33] for c in s[1:])


def main():
    token = os.environ["API_TOKEN"]

    while True:
        text = input("> ")
        exp = encode_str(text)

        req = urllib.request.Request(_ENDPOINT, data=exp.encode(), method='POST')
        req.add_header("Authorization", f"Bearer {token}")
        with urllib.request.urlopen(req) as response:
            res = response.read().decode()

        if res.startswith("S") and " " not in res:
            print(decode_str(res))
        else:
            print(res)


if __name__ == '__main__':
    main()
