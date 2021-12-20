all:
	$(MAKE) p1 || true
	$(MAKE) p2 || true

p1: p1.c ../common/parse.h
	gcc -O2 p1.c -o p1

p2: p2.c ../common/parse.h
	gcc -O2 p2.c -o p2
