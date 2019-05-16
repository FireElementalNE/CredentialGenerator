#include <iostream>
#include <stdlib.h>
#include <stdio.h>
#include "alphabet.h"
#include "username.h"
#include "constants.h"
#include <assert.h>


using namespace std;

char * safe_copy(char * s) {
  char * cpy;
  if(!(cpy = new char[MAX_CPY])) {
    perror("Allocation Failure safe_copy()");
    exit(EXIT_FAILURE);
  }
  strncpy(cpy, s, MAX_CPY);
  return cpy;
}

int main(int argc, char ** argv) {
	if(argc != 2) {
		cerr << "Usage: " << argv[0] << " <int>" << endl;
		exit(EXIT_FAILURE);
	}
	char * safe_arg = safe_copy(argv[1]);
	int length = atoi(argv[1]);
	if(length < 8) {
		cerr << "This password is too small." << endl;
		exit(EXIT_FAILURE);
	}
	Username un = Username();
	Alphabet a = Alphabet(length);
	cout << "Username: " << un.get_username_str() << endl;
	cout << "Password: " << a.get_password() << endl;
	delete safe_arg;
	exit(EXIT_SUCCESS);
}