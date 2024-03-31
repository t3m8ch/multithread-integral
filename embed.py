from ctypes import cdll
import time

a1 = time.time()
lib = cdll.LoadLibrary("./target/release/libembed.so")
b1 = time.time()
print("Lib loading: " + str(b1 - a1))
print()

a2 = time.time()
lib.process()
b2 = time.time()
print("One thread: " + str(b2 - a2))
print()

for t in range(2, 33):
    a3 = time.time()
    lib.process_multi(8)
    b3 = time.time()

    print(f"{t} threads: " + str(b3 - a3))
    print()
