#include <iostream>
#ifndef _WIN32
#include <openssl/rand.h>
#else
#include <windows.h>
#include <Wincrypt.h>
#endif

#ifndef _WIN32
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
#else
int rand_int() {
    int x = -1;
    HCRYPTPROV hCryptProv;
    BYTE pbData[4];
    if (CryptAcquireContext(&hCryptProv, NULL, "Microsoft Base Cryptographic Provider v1.0", PROV_RSA_FULL, CRYPT_VERIFYCONTEXT)) {
        if (CryptGenRandom(hCryptProv,4,pbData)) {
            x = (pbData[3] << 24) | (pbData[2] << 16) | (pbData[1] << 8) | (pbData[0]); 
            // delete rand_msg;
        }
    }
    if(x == -1) {
        std::cerr << "crypto: failed." << std::endl;
        exit(EXIT_FAILURE);
    }
    return x;
}
#endif
