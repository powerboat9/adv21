#include "parse.h"
#include <limits.h>
#include <string.h>

PARSE_FILE("i1.txt")

#define MAX_CALLS 128
#define MAX_BOARDS 128

struct bingo_board {
    unsigned char cells[25];
    unsigned char is_select[25];
};

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

void skip_line() {
    size_t n;
    char *s = NULL;
    if (getline(&s, &n, _parse_fh)) {};
    free(s);
}

void fetch_csv_ints(int *ls, size_t *len, size_t max_len) {
    size_t n;
    char *line = fetch_line(&n);
    char *cur = line;
    size_t i;
    for (i = 0; *cur && (i < max_len); i++) {
        char *new_cur;
        ls[i] = (int) strtol(cur, &new_cur, 10);
        if (*new_cur == ',') new_cur++;
        cur = new_cur;
    }
    *len = i;
    free(line);
}

// 0 on success
// -1 on error
int fetch_bingo(struct bingo_board *board) {
    memset(board->is_select, 0, 25);
    for (int r = 0; r < 5;) {
        char *line;
        size_t line_len;
        if (!(line = fetch_line(&line_len))) return -1;
        if (*line == '\n') {
            free(line);
            continue;
        }
        char *cur = line;
        for (int c = 0; c < 5; c++) {
            char *new_cur;
            board->cells[r*5+c] = (unsigned char) strtol(cur, &new_cur, 10);
            cur = new_cur;
        }
        free(line);
        r++;
    }
    return 0;
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

int get_score(struct bingo_board *board, int last_call) {
    int acc = 0;
    for (int i = 0; i < 25; i++) {
        if (!board->is_select[i]) acc += board->cells[i];
    }
    return acc * last_call;
}

// returns 1 if has been marked
int mark_board(struct bingo_board *board, int call) {
    int ret = 0;
    for (int i = 0; i < 25; i++) {
        if (board->cells[i] == call) {
            board->is_select[i] = 1;
            ret = 1;
        }
    }
    return ret;
}

int is_winner(struct bingo_board *board) {
    for (int i = 0; i < 5; i++) {
        int has_row = 1;
        int has_col = 1;
        for (int j = 0; j < 5; j++) {
            has_row &= board->is_select[i*5+j];
            has_col &= board->is_select[j*5+i];
        }
        if (has_row | has_col) return 1;
    }
    return 0;
}

int main(int argc, char **argv) {
    CHECK_INPUT;
    int calls[MAX_CALLS];
    size_t call_cnt;
    struct bingo_board boards[MAX_BOARDS];
    size_t board_cnt;
    // read input
    fetch_csv_ints(calls, &call_cnt, MAX_CALLS);
    for (board_cnt = 0; (board_cnt < MAX_BOARDS) && !fetch_bingo(boards+board_cnt); board_cnt++) {}
    // play bingo
    for (int i = 0; i < call_cnt; i++) {
        for (int j = 0; j < board_cnt; j++) {
            if (mark_board(boards + j, calls[i]) && is_winner(boards + j)) {
                printf("> %d\n", get_score(boards + j, calls[i]));
                return 0;
            }
        }
    }
}
