#include <iostream>
#include <stdlib.h>
#include <string.h>
#include <openssl/rand.h>
#include <fstream>
#include "username.h"
#include "constants.h"
using namespace std;
Username::Username() {
  ifstream nfile;
  ifstream afile;
  nfile.open(NOUN_FILE);
  afile.open(ADJ_FILE);
  if(!nfile.is_open()) {
    cerr << "File " << NOUN_FILE << " was not opened correctly." << endl;
    exit(EXIT_FAILURE);
  }
  if(!afile.is_open()) {
    cerr << "File " << ADJ_FILE << " was not opened correctly." << endl;
    exit(EXIT_FAILURE);
  }
  string tmp;
  while(nfile >> tmp) {
    nouns.push_back(tmp);
  }
  while(afile >> tmp) {
    adjectives.push_back(tmp);
  }
  afile.close();
  nfile.close();
}

string Username::get_username_str() {
  string adj = adjectives[get_random_number(adjectives.size())]; 
  string noun = nouns[get_random_number(nouns.size())];
  noun[0] = toupper(noun[0]);
  adj[0] = toupper(adj[0]);
  return adj + noun;
}

int Username::get_random_number(int modulus) {
	char * rand_msg = new char[1];
	int rc = RAND_bytes((unsigned char *)rand_msg, 1);
	if(rc == 0) {
		cerr << "crypto: failed." << endl;
		exit(EXIT_FAILURE);
  }
  int x = *rand_msg;
  if(x < 0) {
  	x = x * -1;
  }
  x = x % modulus;
  delete rand_msg;
  return x;
}
