import argparse
import sys

from alphabet import Alphabet
from username import Username

def main(length: int):
	if length < 0:
		print("This password is too small")
		sys.exit(0)
	uname: Username = Username()
	abet: Alphabet = Alphabet(length)
	print(f"Username: {uname.get_username_str()}")
	print(f"Password: {abet.get_password()}")		

if __name__ == '__main__':
	parser = argparse.ArgumentParser("Generate a username and a password")
	parser.add_argument("length", help="number of characters for the password", type=int)
	args = parser.parse_args()
	main(args.length)
	