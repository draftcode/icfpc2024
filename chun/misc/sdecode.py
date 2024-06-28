from dataclasses import dataclass
import sys
from typing import List, Tuple, Union


_ENCODE_MAP = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n"


def decode(s):
    decoded = ""
    for c in s:
        decoded += _ENCODE_MAP[ord(c) - 33]
    return decoded


def encode(s):
    encoded = ""
    for c in s:
        encoded += chr(_ENCODE_MAP.index(c) + 33)
    return encoded


def decode_num(s):
    ret = 0
    for i, c in enumerate(s):
        ret += ord(c) - 33
        if i != len(s) - 1:
            ret *= 94
    return ret


assert decode_num("!") == 0
assert decode_num("/6") == 1337


@dataclass
class ImmString:
    s: str

    def __str__(self):
        return '"' + self.s + '"'


@dataclass
class Sexpr:
    vars: List[Union["Sexpr", ImmString, str, int, bool]]

    def __str__(self):
        return "(" + " ".join(map(str, self.vars)) + ")"


def decode_tree(remain_token) -> Tuple[Union["Sexpr", str, int, bool], int]:
    head = remain_token[0]
    assert len(head) > 0
    if head[0] == "S":
        return (ImmString(s=decode(head[1:])), 1)
    elif head[0] == "T":
        return (True, 1)
    elif head[0] == "F":
        return (False, 1)
    elif head[0] == "I":
        return (decode_num(head[1:]), 1)
    elif head[0] == "U":
        op = {"-": "-", "!": "!", "#": "stoi", "$": "itos"}[head[1]]
        arg1, consumed = decode_tree(remain_token[1:])
        return (Sexpr(vars=[op, arg1]), 1 + consumed)
    elif head[0] == "B":
        op = {
            "+": "+",
            "-": "-",
            "*": "*",
            "/": "/",
            "%": "%",
            "<": "<",
            ">": ">",
            "=": "=",
            "|": "|",
            "&": "&",
            ".": "concat",
            "T": "takefirst",
            "D": "dropfirst",
            "$": "apply",
        }[head[1]]
        arg1, consumed1 = decode_tree(remain_token[1:])
        arg2, consumed2 = decode_tree(remain_token[1 + consumed1 :])
        return Sexpr(vars=[op, arg1, arg2]), (1 + consumed1 + consumed2)
    elif head[0] == "?":
        op = "if"
        arg1, consumed1 = decode_tree(remain_token[1:])
        arg2, consumed2 = decode_tree(remain_token[1 + consumed1 :])
        arg3, consumed3 = decode_tree(remain_token[1 + consumed1 + consumed2 :])
        return Sexpr(vars=[op, arg1, arg2, arg3]), 1 + consumed1 + consumed2 + consumed3
    elif head[0] == "L":
        varnum = decode_num(head[1:])
        arg1, consumed1 = decode_tree(remain_token[1:])
        return Sexpr(vars=[f"\\v{varnum} ->", arg1]), 1 + consumed1
    elif head[0] == "v":
        varnum = decode_num(head[1:])
        return Sexpr(vars=[f"\\v{varnum} ->", arg1]), 1
    raise RuntimeError(f"head = {head}")


op = "get language_test"
e = encode(op)
assert decode(e) == op


def main():
    tokenized = '? B= U# S4%34 I4%34 ? U! F ? B= U- I$ B- I# I& ? B= I$ B- I& I# ? B= S4%34 S4%34 ? B= F F ? B= I$ I$ ? T B. B. SM%,&k#(%#+}IEj}3%.$}z3/,6%},!.\'5!\'%y4%34} U$ B+ I# B* I$> I1~s:U@ Sz}4/}#,!)-}0/).43}&/2})4 S)&})3}./4}#/22%#4 S").!29}q})3}./4}#/22%#4 S").!29}q})3}./4}#/22%#4 S").!29}q})3}./4}#/22%#4 S").!29}k})3}./4}#/22%#4 S5.!29}k})3}./4}#/22%#4 S5.!29}_})3}./4}#/22%#4 S5.!29}a})3}./4}#/22%#4 S5.!29}b})3}./4}#/22%#4 S").!29}i})3}./4}#/22%#4 S").!29}h})3}./4}#/22%#4 S").!29}m})3}./4}#/22%#4 S").!29}m})3}./4}#/22%#4 S").!29}c})3}./4}#/22%#4 S").!29}c})3}./4}#/22%#4 S").!29}r})3}./4}#/22%#4 S").!29}p})3}./4}#/22%#4 S").!29}{})3}./4}#/22%#4 S").!29}{})3}./4}#/22%#4 S").!29}d})3}./4}#/22%#4 S").!29}d})3}./4}#/22%#4 S").!29}l})3}./4}#/22%#4 S").!29}N})3}./4}#/22%#4 S").!29}>})3}./4}#/22%#4 S!00,)#!4)/.})3}./4}#/22%#4 S!00,)#!4)/.})3}./4}#/22%#4'.split()
    decoded, _ = decode_tree(tokenized)
    print(str(decoded))
    # for l in sys.stdin:
    #    tokenized = l.split("\n")


##
#        print(decode_tree(tokenized))

if __name__ == "__main__":
    main()
