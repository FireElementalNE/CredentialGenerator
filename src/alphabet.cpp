#include <iostream>
#include <stdlib.h>
#include <string.h>
// #include <openssl/rand.h>
#include <sstream>
#include <vector>
#include <bits/stdc++.h>
#include "alphabet.h"
#include "constants.h"

using std::vector;

extern int rand_int();

Alphabet::Alphabet(int l) {
	length = l;
}

char Alphabet::get_random_char() {
    int x = rand_int();
    if(x < 0) {
    	x = x * -1;
    }
    vector<int> valid_chars{ 48, 49, 50, 51, 52, 53, 54, 55, 56, 
					      	 57, 33, 34, 35, 36, 37, 38, 40, 41, 
						  	 42, 43, 44, 45, 46, 58, 59, 60, 61, 
						  	 62, 63, 64, 65, 66, 67, 68, 69, 70, 
						  	 71, 72, 73, 74, 75, 76, 77, 78, 79, 
					      	 80, 81, 82, 83, 84, 85, 86, 87, 88, 
					      	 89, 90, 97, 98, 99, 100, 101, 102, 103, 
					      	 104, 105, 106, 107, 108, 109, 110, 111, 
					      	 112, 113, 114, 115, 116, 117, 118, 119, 
					      	 120, 121, 122};

    return (char)valid_chars[x % valid_chars.size()];
}

std::string Alphabet::get_password() {
	std::string out_str;
	out_str = "";
	for(int i = 0; i < length; i++) {
		out_str += get_random_char();
	}
	return out_str;
}
