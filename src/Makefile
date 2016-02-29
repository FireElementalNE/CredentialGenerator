CC=g++
CFLAGS=-g -Wall -std=c++11
CRYPTO_FLAGS=-lssl -lcrypto 
OBJ=alphabet.o
all: main
main: main.cpp $(OBJ)
	$(CC) -o main main.cpp $(OBJ) $(CRYPTO_FLAGS)
alphabet.o: alphabet.cpp
	$(CC) -c -o alphabet.o alphabet.cpp $(CRYPTO_FLAGS)
clean:
	rm -f main *.o
