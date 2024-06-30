#!/usr/bin/env python3

positive_pattern = """. {} . . .
A = S . ."""

negative_pattern = """. {} . . .
A = . . .
. . . . .
-1 * S . ."""

min_pattern = """. -1 . . .
-99 + . . .
. . . -1 .
A = . * S"""

max_pattern = """. 99 . . .
1 + . . .
. . . . .
A = S . ."""

print("solve 3d2")
# print(min_pattern)
print(negative_pattern.format(-96))
print(negative_pattern.format(-84))
print(negative_pattern.format(-73))
print(negative_pattern.format(-70))
print(negative_pattern.format(-37))
print(negative_pattern.format(-21))
print(negative_pattern.format(-11))
print(negative_pattern.format(-10))
print(negative_pattern.format(-6))
print(positive_pattern.format(0))
print(positive_pattern.format(2))
print(positive_pattern.format(3))
print(positive_pattern.format(8))
print(positive_pattern.format(17))
print(positive_pattern.format(29))
print(positive_pattern.format(52))
print(positive_pattern.format(62))
print(positive_pattern.format(69))
print(positive_pattern.format(71))
print(positive_pattern.format(78))
print(positive_pattern.format(88))
print(max_pattern)
