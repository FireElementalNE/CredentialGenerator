#include <iostream>
#include <stdlib.h>
#include <string.h>
#include <openssl/rand.h>
#include <sstream>
#include "alphabet.h"
#include "constants.h"

extern int rand_int();

Alphabet::Alphabet(int l) {
	length = l;
}

char Alphabet::get_random_char() {
    int x = rand_int();
    if(x < 0) {
    	x = x * -1;
    }
    return (char)((x % (MAX_CHAR-MIN_CHAR + 1)) + MIN_CHAR);
}

std::string Alphabet::get_password() {
	std::string out_str;
	out_str = "";
	for(int i = 0; i < length; i++) {
		out_str += get_random_char();
	}
	return out_str;
}
