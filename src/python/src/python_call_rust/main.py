# main.py
import ctypes

# Load the Rust library
lib = ctypes.CDLL("./lib.so")  # Change to lib.dll on Windows

# Define the argument and return types for the Rust function
lib.is_prime.argtypes = [ctypes.c_uint64]
lib.is_prime.restype = ctypes.c_bool


# Define a Python function to call the Rust function
def is_prime_rust(num):
    return bool(lib.is_prime(num))


# Test the Rust function
number_to_check = 17
if is_prime_rust(number_to_check):
    print(f"{number_to_check} is prime.")
else:
    print(f"{number_to_check} is not prime.")


lib.calculate_sum.argtypes = [ctypes.c_uint64]
lib.calculate_sum.restype = ctypes.c_uint64

# Call the Rust function from Python
n = 1000000
result = lib.calculate_sum(n)
print(f"Sum from 1 to {n}: {result}")
