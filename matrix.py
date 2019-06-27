import random
import sys

def doMatrix():
    chars="abcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()_+"
    for i in range(1, 10000000):
        for i in range(1, 1000):
            chosen = chars[random.randint(0, len(chars)-1)]
            sys.stdout.write(chosen)
        sys.stdout.write("\n")

doMatrix()
