#include <iostream>
#include <stdlib.h>
#include <string.h>
#include <openssl/rand.h>
#include "alphabet.h"
#include "constants.h"
using namespace std;

Alphabet::Alphabet() {
	choice_string = ALPHA_LOWER + ALPHA_UPPER + NUMBERS;
}

bool Alphabet::check_memebership(string alphabet_constant, string result) {
	for(int i = 0; i < result.size(); i++) {
		for(int j = 0; j < alphabet_constant.size(); j++) {
			if(result[i] == alphabet_constant[j]) {
				return true;
			}
		}
	}
	return false;
}
bool Alphabet::check_all_alphabets(string result) {
	bool upper_test = check_memebership(ALPHA_UPPER, result);
	bool lower_test = check_memebership(ALPHA_LOWER, result);
	bool num_test = check_memebership(NUMBERS, result);
	// need to think more on this.
	// return upper_test && lower_test && num_test;
	return true;
}
int Alphabet::get_random_number() {
	char * rand_msg = new char[1];
	int rc = RAND_bytes((unsigned char *)rand_msg, 1);
	if(rc == 0) {
		cerr << "crypto: failed." << endl;
		exit(EXIT_FAILURE);
  }
  int x = *rand_msg;
  if(x < 0) {
  	x = x * -1;
  }
  x = x % choice_string.size();
  return x;
}
string Alphabet::get_password(int length) {
	string out_str;
	do {
		out_str = "";
		for(int i = 0; i < length; i++) {
			out_str += choice_string[get_random_number()];
		}
	} while(!check_all_alphabets(out_str));
	return out_str;
}
