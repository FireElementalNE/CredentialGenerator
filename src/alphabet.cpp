#include <iostream>
#include <stdlib.h>
#include <string.h>
#include <openssl/rand.h>
#include "alphabet.h"
using namespace std;
Alphabet::Alphabet() {bool has_alpha_upper(string result);
	bool has_alpha_lower(string result);
	bool has_number(string result);
	bool has_symbol(string result);
	alpha_str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNPQRSTUVWXYZ1234567890!@#$^&*()_+=-|?/\\";
}
bool Alphabet::has_alpha_upper(string result) {}
bool Alphabet::has_alpha_lower(string result) {}
bool Alphabet::has_number(string result) {}
bool Alphabet::has_symbol(string result) {}
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
  x = x % alpha_str.size();
  return x;
}
string Alphabet::get_password(int length) {
	string out_str;
	do {
		out_str = "";
		for(int i = 0; i < length; i++) {
			out_str += alpha_str[get_random_number()];
		}
	} while(!has_alpha_upper(out_str) 
				|| !has_alpha_lower(out_str) 
				|| !has_number(out_str) 
				|| !has_symbol(out_str));
	return out_str;
}