#include "parse.h"
#include <limits.h>
#include <string.h>

PARSE_FILE("i1.txt")

#define LINE_COUNT_MAX 1024
#define COL_COUNT 12

char *fetch_line(size_t *len) {
    char *s = NULL;
    int r = getline(&s, len, _parse_fh);
    if (r == -1) {
        free(s);
        return NULL;
    } else {
        return s;
    }
}

// returns 0 on success, -1 on error
int fetch_bin_int(int *out) {
    size_t n;
    char *s = fetch_line(&n);
    if (!s) return -1;
    *out = strtol(s, NULL, 2);
    free(s);
    return 0;
}

int search(unsigned int *ls_in, size_t len, int neg) {
    unsigned int *ls = calloc(len, sizeof(int));
    memcpy(ls, ls_in, len * sizeof(int));
    for (unsigned int mask = 1 << (sizeof(int) * 8 - 1); mask; mask >>= 1) {
        // get expected
        int cnt = 0;
        for (size_t i = 0; i < len; i++) {
            cnt += (!!(ls[i] & mask) << 1) - 1;
        }
        int not_expect = (cnt >= 0) ? 0 : ~0;
        if (neg) not_expect ^= (1 << COL_COUNT) - 1;
        // filter
        size_t w = 0;
        for (size_t i = 0; i < len; i++) {
            if ((ls[i] ^ not_expect) & mask) {
                ls[w++] = ls[i];
            }
        }
        if (!w) __builtin_trap();
        if (w == 1) {
            unsigned int r = *ls;
            free(ls);
            return r;
        }
        len = w;
    }
    __builtin_trap();
}

int main(int argc, char **argv) {
    CHECK_INPUT;
    // vars
    unsigned int entries[LINE_COUNT_MAX];
    int entry_cnt;
    // read lines
    for (entry_cnt = 0; (entry_cnt < LINE_COUNT_MAX) && (fetch_bin_int(entries+entry_cnt) != -1); entry_cnt++) {}
    // output
    printf("> %d\n", search(entries, entry_cnt, 0) * search(entries, entry_cnt, 1));
}
