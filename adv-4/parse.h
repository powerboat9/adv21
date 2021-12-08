#include <stdlib.h>
#include <stdio.h>

FILE *_parse_fh;

__attribute__((destructor)) void _parse_dest() {
    if (_parse_fh) {
        fclose(_parse_fh);
        _parse_fh = NULL;
    }
}

#define PARSE_FILE(name) __attribute__((constructor)) void _parse_const() {\
    _parse_fh = fopen(name, "r");\
}

#define CHECK_INPUT do {\
    if (!_parse_fh) {\
        perror("failed to open input file");\
        exit(-1);\
    }\
} while (0)

#define READ_LINE(ptr, n) getline(ptr, n, _parse_fh)

#define LINES for (struct {char *ptr; ssize_t len;} _ = {NULL, 0}; READ_LINE(&_.ptr, &_.len) != -1;)
#define _line (_.ptr)
#define _line_len (_.len)
