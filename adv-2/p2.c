#include "parse.h"
#include <limits.h>
#include <string.h>

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
    size_t line_len;
    int x = 0;
    int y = 0;
    int aim = 0;
    for (char *line; line = fetch_line(&line_len); free(line)) {
        if (!memcmp(line, "forward ", 8)) {
            int v = atoi(line+8);
            x += v;
            y += aim*v;
        } else if (!memcmp(line, "down ", 5)) {
            aim += atoi(line+5);
        } else if (!memcmp(line, "up ", 3)) {
            aim -= atoi(line+3);
        }
    }
    printf("> %d\n", x*y);
}
