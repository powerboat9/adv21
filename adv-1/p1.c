#include "parse.h"
#include <limits.h>

PARSE_FILE("i1.txt")

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
int fetch_int(int *out) {
    size_t n;
    char *s = fetch_line(&n);
    if (!s) return -1;
    *out = atoi(s);
    free(s);
    return 0;
}

int main(int argc, char **argv) {
    CHECK_INPUT;
    int last = INT_MAX;
    int inc_cnt = 0;
    for (int num; fetch_int(&num) != -1;) {
        if (num > last) inc_cnt++;
        last = num;
    }
    printf("> %d\n", inc_cnt);
}
