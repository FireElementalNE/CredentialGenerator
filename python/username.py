from constants import NOUN_FILE
from constants import ADJ_FILE

from typing import List, TextIO

from utils import rand_int


class Username:
	def __init__(self):
		self.nouns: List[str] = []
		self.adjectives: List[str] = []
		nfile: TextIO
		afile: TextIO
		with open(NOUN_FILE, 'r') as nfile:
			line: str
			for line in Username.clean_lst(nfile.readlines()):
				self.nouns.append(line.rstrip())
		with open(ADJ_FILE, 'r') as afile:	
			line: str
			for line in Username.clean_lst(afile.readlines()):
				self.adjectives.append(line.rstrip())

	@staticmethod
	def clean_lst(lst: List[str]) -> List[str]:
		return [el.rstrip().lstrip().replace(' ', '') for el in lst]

	def get_random_number(self, modulus: int) -> int:
		return rand_int() % modulus

	def get_username_str(self) -> str:
		adj: List[str] = list(self.adjectives[self.get_random_number(len(self.adjectives))])
		noun: List[str] = list(self.nouns[self.get_random_number(len(self.nouns))])
		adj[0] = adj[0].upper()
		noun[0] = noun[0].upper()
		return f"{''.join(adj)}{''.join(noun)}"
		