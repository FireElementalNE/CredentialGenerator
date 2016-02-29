#include <iostream>
#include <stdlib.h>
#include "alphabet.h"
using namespace std;
int main(int argc, char ** argv) {
	if(argc != 2) {
		cerr << "Usage: " << argv[0] << " <int>" << endl;
		exit(EXIT_FAILURE);
	}
	Alphabet a = Alphabet();
	int length = atoi(argv[1]);
	cout << a.get_password(length) << endl;
	exit(EXIT_SUCCESS);
}