#include <stdlib.h>
#include <stdio.h>

#define A 0b0000001
#define B 0b0000010
#define C 0b0000100
#define D 0b0001000
#define E 0b0010000
#define F 0b0100000
#define G 0b1000000

static const int digits[] = {
    A | B | C | E | F | G,
    C | F,
    A | C | D | E | G,
    A | C | D | F | G,
    B | C | D | F,
    A | B | D | F | G,
    A | B | D | E | F | G,
    A | C | F,
    A | B | C | D | E | F | G,
    A | B | C | D | F | G
};

void print_digits(int combo) {
    int had_last = 0;
    for (char c = '0'; c <= '9'; c++) {
        if (combo & 1) {
            if (had_last) fputs(" ^ ", stdout);
            had_last = 1;
            putc(c, stdout);
        }
        combo >>= 1;
    }
}

int main(int argc, char **argv) {
    for (unsigned int dig_combo = 1; dig_combo & ((1 << 10) - 1); dig_combo++) {
        // skip single digits
        if (!(dig_combo & (dig_combo - 1))) continue;
        unsigned int tmp = dig_combo;
        unsigned int base = 0;
        // get xor sum
        for (int i = 0; i < 10; i++) {
            if (tmp & 1) base ^= digits[i];
            tmp >>= 1;
        }
        // check digits
        for (int i = 0; i < 10; i++) {
            if (base == digits[i]) {
                print_digits(dig_combo);
                fputs(" = ", stdout);
                putc('0' + i, stdout);
                putc('\n', stdout);
                break;
            }
        }
    }
    return 0;
}
