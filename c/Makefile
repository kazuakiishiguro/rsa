CFLAGS=-Wall -std=c11

rsa: rsa.o
	${CC} -o $@ $? $(LDFLAGS)

test: rsa
	./test.sh

clean:
	rm -f *.o a.out rsa.o rsa
