CC=gcc
CFLAGS=-DDEBUG -I ../uvwasi/include/ -L../uvwasi/ -L../uvwasi/_deps/libuv-build/ -luvwasi_a -luv_a
# for larger NUM_THREADS we would have to malloc bigger structures to account for the interleave!
# with N=1, we can save space since we're just testing a single thread here
CFLAGS+=-D NUM_THREADS=1
all:
	$(CC) test.c $(CFLAGS) -g -o test
clean:
	rm test