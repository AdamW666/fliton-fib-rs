from time import time
from fliton_fib_rs import fibonacci_number
from numba import jit


def python_fib_number(num: int) -> int:
    if num < 0:
        raise ValueError('Fibonacci should take parameter above 0')
    elif num in [1, 2]:
        return 1
    else:
        return python_fib_number(num - 1) + python_fib_number(num - 2)


@jit(nopython=True)
def numba_fib_number(num: int) -> int:
    if num < 0:
        raise ValueError('Fibonacci should take parameter above 0')
    elif num in [1, 2]:
        return 1
    else:
        return numba_fib_number(num - 1) + numba_fib_number(num - 2)


# python
python_fib_number(35)
t0 = time()
for i in range(20):
    python_fib_number(35)
t1 = time()
print(f"the time taken for python is: {t1-t0}")

# numba
numba_fib_number(35)
t0 = time()
for i in range(20):
    numba_fib_number(35)
t1 = time()
print(f"the time taken for numba is: {t1-t0}")

# rust
t0 = time()
for i in range(20):
    fibonacci_number(35)
t1 = time()
print(f"the time taken for rust boosted python is: {t1-t0}")
