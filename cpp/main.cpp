#include <iostream>
#include <stdlib.h>
#include <stdio.h>
#include "alphabet.h"
#include "username.h"
#include "constants.h"
#include <assert.h>
#include <memory>

using namespace std;


int main(int argc, char ** argv) {
	if(argc != 2) {
		cerr << "Usage: " << argv[0] << " <int>" <<endl;
		exit(EXIT_FAILURE);
	}
	std::string tmp(argv[1]);
	int length = stoi(tmp);
	if(length < 8) {
		cerr << "This password is too small." << endl;
		exit(EXIT_FAILURE);
	}
	Username un = Username();
	Alphabet a = Alphabet(length);
	cout << "Username: " << un.get_username_str() << endl;
	cout << "Password: " << a.get_password() << endl;
	exit(EXIT_SUCCESS);
}