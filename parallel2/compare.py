from sympy import isprime

large_num = 10**500

for i in range(1, 1000):
    next_num = large_num + i
    print(next_num)
    print(isprime(next_num))