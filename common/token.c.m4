struct _m_line_store {
    char *line;
    char *pos;
    size_t len_left;
};

int _m_parse_int(struct _m_line_store *l, int *n) {
    l->
}

m4_define(`OPEN', FILE *_fh_in = fopen("$1", "r") ? : ({perror("failed to open input"); exit(2); NULL}))dnl
m4_define(`LINE_INIT', struct _m_line_store _line = {NULL, NULL, 0})dnl
m4_define(`PARSE_INT', for (int $1; _m_parse_int(); break)
m4_define(`TOKEN', `dnl
#define TOKEN_$1

')m4_dnl
TOKEN(`FOO')
