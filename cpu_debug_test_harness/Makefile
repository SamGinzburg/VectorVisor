CC=gcc
CFLAGS=-DDEBUG -I ../uvwasi/include/ -L../uvwasi/ -L../uvwasi/_deps/libuv-build/ -luvwasi_a -luv_a
CFLAGS+=-D NUM_THREADS=16
all:
	$(CC) test.c $(CFLAGS) -g -o test
clean:
	rm test