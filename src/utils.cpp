#include <iostream>
#include <openssl/rand.h>

int rand_int() {
	char * rand_msg = new char[4];
	int rc = RAND_bytes((unsigned char *)rand_msg, 4);
	if(rc == 0) {
		std::cerr << "crypto: failed." << std::endl;
		exit(EXIT_FAILURE);
    }
    int x = (rand_msg[3] << 24) | (rand_msg[2] << 16) | (rand_msg[1] << 8) | (rand_msg[0]);	
    delete rand_msg;
    return x;
}

