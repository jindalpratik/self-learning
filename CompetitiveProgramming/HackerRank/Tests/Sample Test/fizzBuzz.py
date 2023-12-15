#!/bin/python3

import math
import os
import random
import re
import sys



#
# Complete the 'fizzBuzz' function below.
#
# The function accepts INTEGER n as parameter.
#

def fizzBuzz(k):
    for n in range(1,k+1):
        if n%3 == 0 and n%5 == 0:
            print("FizzBuzz")
        elif n%3 == 0:
            print("Fizz")
        elif n%5 == 0:
            print("Buzz")
        else:
            print(n)
if __name__ == '__main__':
    n = int(input().strip())

    fizzBuzz(n)
