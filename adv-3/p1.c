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
    int buff[12];
    memset(buff, 0, sizeof(buff));
    for (char *line; line = fetch_line(&line_len); free(line)) {
        for (int i = 0; i < 12; i++) {
            switch (line[i]) {
                case '0':
                    buff[i]--;
                    break;
                case '1':
                    buff[i]++;
                    break;
                default:
                    exit(-1);
            }
        }
    }
    int gamma = 0;
    for (int i = 0; i < 12; i++) {
        gamma <<= 1;
        gamma |= buff[i] > 0;
    }
    int eps = (~gamma) & 4095;
    printf("> %d\n", gamma * eps);
}
