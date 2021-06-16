import os
import sys

def rand_int() -> int:
	r_int: int = int.from_bytes(os.urandom(4), sys.byteorder)
	return r_int * -1 if r_int < 0 else r_int
	