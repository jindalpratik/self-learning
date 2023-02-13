#!/bin/python3

import math
import os
import random
import re
import sys



#
# Complete the 'reverse_words_order_and_swap_cases' function below.
#
# The function is expected to return a STRING.
# The function accepts STRING sentence as parameter.
#

def reverse_words_order_and_swap_cases(sentence):
    li = sentence.split()
    new_str = ''
    for k in li[::-1]:
        for i in k:
            if ord(i) > 96 and  ord(i) < 123:
                new_str += chr(ord(i)-32)
            elif ord(i) > (96-32) and  ord(i) < (123-32):
                new_str += chr(ord(i)+32)
            else:
                new_str += i
        new_str += " "
    return new_str

if __name__ == '__main__':
    fptr = open(os.environ['OUTPUT_PATH'], 'w')

    sentence = input()

    result = reverse_words_order_and_swap_cases(sentence)

    fptr.write(result + '\n')

    fptr.close()
