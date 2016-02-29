#include <iostream>
#include <string.h>
using namespace std;
class Alphabet {
private:
	string alpha_str;
	int get_random_number();
	bool has_alpha_upper(string result);
	bool has_alpha_lower(string result);
	bool has_number(string result);
	bool has_symbol(string result);
public:
	Alphabet();
	string get_password(int length);
};