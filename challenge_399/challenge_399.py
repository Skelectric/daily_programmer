"""
https://www.reddit.com/r/dailyprogrammer/comments/onfehl/20210719_challenge_399_easy_letter_value_sum/
Challenge
Assign every lowercase letter a value, from 1 for a to 26 for z. Given a string of lowercase letters, find the sum of the values of the letters in the string.

lettersum("") => 0
lettersum("a") => 1
lettersum("z") => 26
lettersum("cab") => 6
lettersum("excellent") => 100
lettersum("microspectrophotometries") => 317"""


def lettersum(string: str) -> int:
    letter_list = list(string)
    if len(letter_list) == 0:
        return 0
    sum = 0
    for letter in letter_list:
        sum += ord(letter) - (ord('a') - 1)
    return sum


print(lettersum(""))
print(lettersum("a"))
print(lettersum("z"))
print(lettersum("cab"))
print(lettersum("excellent"))
print(lettersum("microspectrophotometries"))
