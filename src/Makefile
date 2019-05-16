CC=g++
CFLAGS=-g -Wall -std=c++14
CRYPTO_FLAGS=-lssl -lcrypto 
OBJ=alphabet.o username.o
all: main
main: main.cpp $(OBJ)
	$(CC) -o main main.cpp $(OBJ) $(CRYPTO_FLAGS)
alphabet.o: alphabet.cpp
	$(CC) -c -o alphabet.o alphabet.cpp $(CRYPTO_FLAGS)
username.o: username.cpp
	$(CC) -c -o username.o username.cpp $(CRYPTO_FLAGS)
clean:
	rm -f main *.o
