#include <iostream>
#include <vector>
using namespace std;
class Username {
private:
	int get_random_number(int modulus);
	vector <string> nouns;
	vector <string> adjectives;
public:
	Username();
	std::string get_username_str();
};