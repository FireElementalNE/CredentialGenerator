CC=g++
CFLAGS=-g -Wall 
ifneq ($(OS),Windows_NT)
	CFLAGS += -std=c++14 -lssl -lcrypto
endif
OBJ=utils.o alphabet.o username.o
all: main
main: main.cpp $(OBJ)
	$(CC) -o main main.cpp $(OBJ) $(CFLAGS)
alphabet.o: alphabet.cpp
	$(CC) -c -o alphabet.o alphabet.cpp $(CFLAGS)
username.o: username.cpp
	$(CC) -c -o username.o username.cpp $(CFLAGS)
utils.o: utils.cpp
	$(CC) -c -o utils.o utils.cpp $(CFLAGS)
clean:
	rm -f main *.o
