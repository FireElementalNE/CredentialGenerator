#include <iostream>
#include <string.h>

class Alphabet {
private:
	int length;
    char get_random_char();
public:
	Alphabet(int l);
	std::string get_password();
};