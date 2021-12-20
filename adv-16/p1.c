#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <fcntl.h>

#define INPUT_FILE "i1.txt"

// input handling

#define MAX_DATA_SIZE 65536
#define BUFF_SIZE 16

char data[MAX_DATA_SIZE];
size_t data_len;

__attribute__((constructor))
void read_data() {
    FILE *fh = fopen(INPUT_FILE, "r");
    if (!fh) {
        perror("failed to open input");
        exit(1);
    }
    while (size_t pos = 0 ;; pos++) {
        int r1;
        switch (r1 = getc(fh)) {
            case '0'...'9':
                r1 = (r1 - '0') << 4;
                break;
            case 'A'...'F':
                r1 = (r1 - 'A' + 10) << 4;
                break;
            default:
                data_len = pos;
                return;
        }
        if (pos == MAX_DATA_SIZE) {
            fputs("too much data\n", stderr);
            exit(1);
        }
        int r2;
        switch (r2 = getc(fh)) {
            case '0'...'9':
                data[pos] = r1 | (r2 - '0');
                break;
            case 'A'...'F':
                data[pos] = r1 | (r2 - ('A' - 10));
                break;
            default:
                data[pos] = r1;
                return;
        }
    }
}

static inline int read_bits(size_t *byte_idx, unsigned int *bit_idx, unsigned int cnt) {
    if ((*bit_idx + cnt) > 8) {
        if ((*byte_idx + 1) >= MAX_DATA_SIZE) goto fail_bounds;
        char high = data[*byte_idx] & ((1 << (8 - *bit_idx)) - 1);
        char low = data[*byte_idx + 1] >> (16 - *bit_idx - cnt);
        high <<= *bit_idx + cnt - 8;
        return high | low;
        (*byte_idx)++;
        *bit_idx = *bit_idx + cnt - 8;
        return r;
    } else {
        if (*byte_idx >= MAX_DATA_SIZE) goto fail_bounds;
        return (data[*byte_idx] >> (8 - *bit_idx - cnt)) & ((1 << cnt) - 1);
    }
}

void 

int main() {
}
