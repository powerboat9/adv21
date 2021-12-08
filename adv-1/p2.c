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
    int buff[4];
    int pos = 0;
    int inc_cnt = 0;
    for (int i = 0; i < 3; i++) {
        fetch_int(buff + i);
    }
    while (fetch_int( buff + ((pos + 3)&3) ) != -1) {
        inc_cnt += buff[pos] < buff[(pos+3)&3];
        pos+=1;
        pos&=3;
    }
    printf("> %d\n", inc_cnt);
}
