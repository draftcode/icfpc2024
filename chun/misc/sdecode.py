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
        if len(head) == 1:
            return (ImmString(s=""), 1)
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
        return Sexpr(vars=[f"v{varnum}"]), 1
    raise RuntimeError(f"head = {head}")


op = "get language_test"
e = encode(op)
assert decode(e) == op


def decode_expr(s):
    decoded, _ = decode_tree(s.split(" "))
    return decoded


def main():
    sys.set_int_max_str_digits(50000)
    # tokenized = '? B= U# S4%34 I4%34 ? U! F ? B= U- I$ B- I# I& ? B= I$ B- I& I# ? B= S4%34 S4%34 ? B= F F ? B= I$ I$ ? T B. B. SM%,&k#(%#+}IEj}3%.$}z3/,6%},!.\'5!\'%y4%34} U$ B+ I# B* I$> I1~s:U@ Sz}4/}#,!)-}0/).43}&/2})4 S)&})3}./4}#/22%#4 S").!29}q})3}./4}#/22%#4 S").!29}q})3}./4}#/22%#4 S").!29}q})3}./4}#/22%#4 S").!29}k})3}./4}#/22%#4 S5.!29}k})3}./4}#/22%#4 S5.!29}_})3}./4}#/22%#4 S5.!29}a})3}./4}#/22%#4 S5.!29}b})3}./4}#/22%#4 S").!29}i})3}./4}#/22%#4 S").!29}h})3}./4}#/22%#4 S").!29}m})3}./4}#/22%#4 S").!29}m})3}./4}#/22%#4 S").!29}c})3}./4}#/22%#4 S").!29}c})3}./4}#/22%#4 S").!29}r})3}./4}#/22%#4 S").!29}p})3}./4}#/22%#4 S").!29}{})3}./4}#/22%#4 S").!29}{})3}./4}#/22%#4 S").!29}d})3}./4}#/22%#4 S").!29}d})3}./4}#/22%#4 S").!29}l})3}./4}#/22%#4 S").!29}N})3}./4}#/22%#4 S").!29}>})3}./4}#/22%#4 S!00,)#!4)/.})3}./4}#/22%#4 S!00,)#!4)/.})3}./4}#/22%#4'.split()
    # seq = r"""B. SYU]}V^X~YWV}WV^~WV[}V]Z~W\U}W^Y~WZ[}ZUX~WZY}ZW]~X] B. B$ L" S Sum%4#m0!337$w SX}W\\~VWY}Y]~V^Y}V[Y~XW^}YUZ~V[Y}W\~YWU}VV^~V]U}ZXY~XZW}WZY~X^W}VZ^~YZ\}^Z~W]\}ZX]~VY]}\^~V\]}VUY~W\[}XUY~XWV}Y^]~YVX}VXW~V[]}VX\~WXX}Z[]~W]X}YXW~WXX}VY^~X\\}WZY~V][}V][~WZ[}ZY[~WUZ}ZXV~X[[}W\Y~W\W}X\]~W^Y}XXU~Y\]}]Z~YV[}VYU~V]U}]\~X[X}X]Z~V]V}ZZU~W\Y}ZYU~Y]X}X\~XZ^}XZW~V][}Z]X~WU\}VUY~X^]}WZ[~W\\}YZY~WWZ}V\Z~V\\}^X~Y[W}VU\~WUV}VW[~XUX}YZ[~XZ^}XYV~W\^}Z[Y~W]X}YX[~XVX}YW]~X\Z}V^]~Y[X}\]~YZZ}VYV~X\X}XZ]~WY^}ZXV~WYX}W[V~YYU}VU\~WX[}ZX]~X\Z}W[V~XYY}X[X~XVZ}ZU[~WXY}WVU~XUV}ZXW~VX]}\X~WUW}VX\~Y[W}YX~X\V}XVU~XVV}X][~X\U}W\W~XUX}YYZ~Y[V}[U~W^X}Y][~XY\}XW\~VZ]}VWU~YWY}VYU~W\V}X[]~YXV}VV^~YZY}ZV~W[Z}ZYV~V[Z}VU\~VXW}ZZ~W^V}Y[\~WVX}WWZ~YU^}WYZ~V^V}ZV~W^^}XUX~Y[[}VWU~XVX}XVW~XW^}XXZ~V[U}\X~X[V}XXU~WWY}V]U~W^[}Y\\~X\Z}WV\~WYX}Z\^~XY]}W^Y~WXW}ZZZ~WZ[}Z\V~XUU}ZWY~Y\^}[U~VX^}[Z~W[[}WZU~YV\}V^V~XWU}XZW~X]\}WWY~X\U}W^\~XU[}YV]~XWY}X]Y~VZ]}^Y~WY\}ZZ^~XUV}X\Y~V\]}ZY~YX\}YZ~WVX}V[\~VX[}ZZ~X^U}VY]~XV^}YWV~VWU}XW~YUW}VYV~WUY}]Z~W^X}XU^~XUU}Y^U~WWY}WUZ~YU[}VXW~VY[}^Z~XU[}XZV~WUY}VZZ~WX^}WYY~W^[}ZX]~XVV}Y^V~VV]}W^~X\U}X\V~W^V}XZ^~YXV}V[X~WZV}V]V~X\V}W^Y~WY^}XVZ~XX^}XZY~W^X}YXW~XY]}WZW~Y\V}YU~W[]}ZYZ~X]X}XW\~YUU}W^U~XUU}X\\~XUW}ZWZ~WZX}WX]~XWU}YZX~XZ\}W[[~WUZ}WVX~WZV}XW\~YUV}WX[~YWX}VV^~W[W}ZZZ~Y\X}ZW~XUX}Y[[~XWY}YU]~WVZ}WY\~X[[}X[[~V]\}VUU~YXX}VUW~WXZ}V\X~V\\}X[~W\Z}ZY]~X[V}X[Z~X^[}VXV~VYU}[]~W\^}WZ[~XZV}YU]~XYV}XY[~X^Z}XUV~WX\}WZX~X\U}W^U~YUU}V]\~YVZ}V]V~XW[}XZV~YVY}VUU~YU]}VWU~W[\}WVV~WV\}V^\~YU]}VWY~WW^}VYX~VZY}^W~X[V}X[]~YVX}V][~WWZ}Z]U~W[Y}WYZ~XZX}X^Y~W]X}XYW~XZZ}W\Z~YV[}WZZ~WZZ}XXW~XY^}W]^~V^V}ZXY~W^[}Y[]~V]^}VW]~V]U}V[W~V]^}V[\~Y]U}X]~W]W}ZW^~Y[]}Z^~XV\}YXV~X]]}WZ]~XW^}YWZ~WUV}Z[^~W^U}ZZ\~Y\\}[Y~XY[}W]]~XUV}ZWY~W[X}ZWX~WWW}WVZ~YX]}[Y~W^[}YWW~WXZ}V[X~V^U}ZYZ~V]Z}Z\Z~W[\}WYW~W\\}W\Y~X[]}XX^~XYX}YVV~V]V}VYZ~XZ\}XV^~W\W}W^Y~X]^}W]V~WXY}WV]~WZ\}V^W~XYW}W^X~X]W}WW[~X]V}WX^~XZX}WYX~W\\}XV[~X^^}W[V~XV\}Y]Z~XYU}YX]~W[Z}WYX~W]Y}YX[~X^V}WYY~V[Z}]X~W[U}Z[Y~YZU}V[Y~WZ[}ZW\~YUV}WYY~V]\}Z\Z~WX[}ZYY~V][}V][~Y[[}VV]~Y[W}[V~W[]}W\V~WXY}VYU~WV\}WU]~W\Y}W]W~X^Z}V[U~WW[}VZ^~WVV}VYV~X\]}V\W~WZX}XUU~WWZ}Z]W~WVX}VU[~X^^}VYY~W\X}Y^]~WY^}WU]~W[^}W\^~W]Z}X[W~WY]}Z\X~XU]}XY^~W[U}ZW^~XZ\}W[Y~XWZ}Y[W~V]W}VW\~Y[\}\^~WUY}WU]~WWV}V^Y~VZY}^Z~V]W}X]~XYW}XXY~YXV}WVZ~XY[}XZ\~X[W}X]W~XXX}Y[V~V^U}VXZ~WZ\}Z[X~W^Y}X\Y~XUY}Y^[~W\W}XV\~YX\}\X~VYV}W[~XYY}YX[~VY]}\Z~WZ\}ZVX~V]V}^V~YV\}WWV~YVY}WY]~WYZ}V]U~W[V}WW^~V]W}VVV~WU]}VYW~"""
    seq = sys.stdin.read()
    seq = seq.rstrip("\n")
    decoded = decode_expr(seq)
    print(str(decoded))
    # for l in sys.stdin:
    #    tokenized = l.split("\n")


##
#        print(decode_tree(tokenized))

if __name__ == "__main__":
    main()
