#include <iostream>
#include <string.h>
using namespace std;
class Alphabet {
private:
	string choice_string;
	int get_random_number();
	bool check_memebership(string alphabet_constant, string result);
	bool check_all_alphabets(string result);
public:
	Alphabet();
	string get_password(int length);
};