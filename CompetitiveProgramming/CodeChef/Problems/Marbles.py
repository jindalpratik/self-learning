import math

# Number of test cases
t = int(input())

for i in range(t):
    # Number of marbles, Number of unique marbles
    [n, k] = [int(x) for x in input().split()]
    # As we have to atleast one type of marble total number of marbles we have to devide goes down to n-k
    # Also as the marbles can be in whatever order this becomes a case of combination with repetition
    s = n - k
    # The number of ways of making s selections from among r distinguishable possibilities, where the order does not matter and repetitions are allowed is (k+s-1)!/((k-1)!*(s)!) which is equal to (n-1)!/(k-1)!(n-k)! or (n-1)c(k-1) in this case as s is equal to n - k.
    # More information at https://math.stackexchange.com/questions/58753/distinct-ways-to-keep-n-balls-into-k-boxes , https://en.wikipedia.org/wiki/Stars_and_bars_(combinatorics)

    print(math.comb(n-1, k-1))
