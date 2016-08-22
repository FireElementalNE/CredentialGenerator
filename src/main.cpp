#include <iostream>
#include <stdlib.h>
#include "alphabet.h"
#include "username.h"
using namespace std;
int main(int argc, char ** argv) {
	if(argc != 2) {
		cerr << "Usage: " << argv[0] << " <int>" << endl;
		exit(EXIT_FAILURE);
	}
	Username un = Username();
	cout << "Username: " << un.get_username_str() << endl;
	Alphabet a = Alphabet();
	int length = atoi(argv[1]);
	if(length < 8) {
		cerr << "This password is too small." << endl;
		exit(EXIT_FAILURE);
	}
	cout << "Password: " << a.get_password(length) << endl;
	exit(EXIT_SUCCESS);
}