#include <iostream>
#include <stdlib.h>
#include <string.h>
#include <openssl/rand.h>
#include <sstream>
#include "alphabet.h"
#include "constants.h"

Alphabet::Alphabet(int l) {
	length = l;
}

char Alphabet::get_random_char() {
    char * rand_msg = new char[4];
	int rc = RAND_bytes((unsigned char *)rand_msg, 4);
	if(rc == 0) {
		std::cerr << "crypto: failed." << std::endl;
		exit(EXIT_FAILURE);
    }
    int x = (rand_msg[3] << 24) | (rand_msg[2] << 16) | (rand_msg[1] << 8) | (rand_msg[0]);
    if(x < 0) {
    	x = x * -1;
    }
	delete rand_msg;
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
