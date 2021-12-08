#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <search.h>

#define INPUT_FILE "i1.txt"

// util functions/macros

FILE *input_file;

inline void check_null(void *ptr) {
    if (!ptr) __builtin_trap();
}

inline size_t safe_st_add(size_t a, size_t b) {
    size_t ret;
    if (__builtin_add_overflow(a, b, &ret)) {
        __builtin_trap();
    }
    return ret;
}

inline size_t safe_st_inc(size_t *a) {
    if (__builtin_add_overflow(*a, 1, a)) {
        __builtin_trap();
    }
}

inline size_t next_pow2(size_t a) {
    if (a) a--;
    for (int i = 1; i < (sizeof(size_t) * 8); i <<= 1) {
        a |= (a >> i);
    }
    safe_st_inc(&a);
    return a;
}

void _close_input() {
    fclose(input_file);
}

void init_input() {
    input_file = fopen(INPUT_FILE, "r");
    if (!input_file) {
        perror("[ERROR] Failed to open input file");
        exit(-1);
    }
    atexit(_close_input);
}

int gcd(int a, int b) {
    if (b > a) {
        int tmp = a;
        a = b;
        b = tmp;
    }
    while (1) {
        int tmp = a % b;
        if (tmp) {
            a = b;
            b = tmp;
        } else {
            return b;
        }
    }
}

struct array_list {
    char *elms;
    size_t size;
    size_t alloc_size;
};

struct array_list *list_create() {
    struct array_list *ret = malloc(sizeof(*ret));
    memset(ret, 0, sizeof(*ret));
    return ret;
}

void list_append(struct array_list *l, void *a, size_t size) {
    if (!size) return;
    size_t new_size = safe_st_add(l->size, size);
    if (new_size > l->alloc_size) {
        l->alloc_size = next_pow2(new_size);
        check_null(l->elms = realloc(l->elms, l->alloc_size));
    }
    memcpy(l->elms+l->size, a, size);
    l->size = new_size;
}

void *list_remove_if(struct array_list *l, int (*call)(void *ent, void *data), void *data, size_t elm_size) {
    size_t w_idx = 0;
    size_t r_idx = 0;
    while (r_idx < l->size) {
        if (!call(l->elms + r_idx, data)) {
            if (w_idx != r_idx) {
                memcpy(l->elms + w_idx, l->elms + r_idx, elm_size);
            }
            w_idx += elm_size;
        }
        r_idx += elm_size;
    }
    l->size = w_idx;
}

void *list_into_vec(struct array_list *l, size_t *len) {
    *len = l->size;
    return realloc(l->elms, l->size);
}

char *fetch_line(size_t *len) {
    char *s = NULL;
    if (getline(&s, len, input_file) == -1) {
        free(s);
        return NULL;
    } else {
        return s;
    }
}

// program specific code

struct draw_line {
    int x1;
    int y1;
    int x2;
    int y2;
};

struct draw_line parse_draw_line(char *s) {
    struct draw_line ret;
    if (sscanf(s, "%d,%d -> %d,%d ", &ret.x1, &ret.y1, &ret.x2, &ret.y2) != 4) {
        memset(&ret, 0, sizeof(struct draw_line));
    }
    return ret;
}

int is_bish(void *ptr) {
    struct draw_line *line = ptr;
    return (line->x1 != line->x2) && (line->y1 != line->y2);
}

struct draw_line *parse_draw_lines(size_t *len) {
    struct array_list *ls = list_create();
    size_t line_len;
    for (char *line; line = fetch_line(&line_len); free(line)) {
        struct draw_line tmp = parse_draw_line(line);
        int ti;
        if ((tmp.y1 > tmp.y2) || ((tmp.y1 == tmp.y2) && (tmp.x1 > tmp.x2))) {
            ti = tmp.x2;
            tmp.x2 = tmp.x1;
            tmp.x1 = ti;
            ti = tmp.y2;
            tmp.y2 = tmp.y1;
            tmp.y1 = ti;
        }
        list_append(ls, &tmp, sizeof(struct draw_line));
    }
    list_remove_if(ls, is_bish, sizeof(struct draw_line));
    return list_into_vec(ls, len);
}

#define EVENT_START 0
#define EVENT_END 1
#define EVENT_H 0
#define EVENT_V 2

struct event {
    int type;
    int x;
    int y;
};

int _sort_int(const void *a_v, const void *b_v) {
    const int *a = a_v, *b = b_v;
    return *a - *b;
}

int _sort_event(const void *a_v, const void *b_v) {
    const struct event *a = a_v, *b = b_v;
    if (a->x < b->x) {
        return -1;
    } else if (a->x > b->x) {
        return 1;
    } else if (a->y < b->y) {
        return -1;
    } else if (a->y > b->y) {
        return 1;
    } else if (a->type & 1) {
        return b->type & 1;
    } else if (b->type & 1) {
        return -1;
    } else {
        return 0;
    }
}

int _sort_line(const void *a_v, const void *b_v) {
    const struct draw_line *a = a_v, *b = b_v;
    if (a->x1 > b->x1) {
        return 1;
    } else if (a->x1 < b->x1) {
        return -1;
    } else {
        return a->y1 - b->y1;
    }
}

struct _calc_event_data {
    void **h_lines_p;
    int cur_v;
    int v_cnt;
    int last_y;
};

struct _overlap_data {
    struct draw_line *first_line;
    int new_len;
    int cnt;
};

int _overlap_remove_h(void *ent_v, void *data_v) {
    struct draw_line *ent = env_v;
    struct _overlap_data *data = data_v;
    if (ent->y1 != ent->y2) return 0;
    if (data->first_line) {
        if (data->first_line->y1 != ent->y1) {
            

void calc_h_overlap(struct array_list *lines, size_t line_cnt)
    qsort(lines->elms, lines->size / sizeof(struct draw_line), sizeof(struct draw_line), _sort_line);
    

void _calc_event_loop_clo(const void *e_v, VISIT vis, void *d_v) {
    if (vis != postorder) return;
    const struct event *e = e_v;
    struct _calc_event_data *d = d_v;
    switch (e->type) {
        case EVENT_START | EVENT_H:
            tsearch
    if (e->type & EVENT_V) {
        if (v_cnt++) {
            last_y = e->y;
        }
    }
            
}

void calc(struct draw_line *lines, size_t line_cnt, void **tree_intersect) {
    void *h_lines = NULL;
    void *events = NULL;
    // build event list
    for (size_t i = 0; i < line_cnt; i++) {
        int flag_base = (lines[i].x1 == lines[i].x2) ? EVENT_V : EVENT_H;
        struct event new_event = {flag_base, lines[i].x1, lines[i].y1};
        check_null(tsearch(&new_event, &events, _sort_event));
        new_event.x = lines[i].x2;
        new_event.y = lines[i].y2;
        new_event.type |= EVENT_END;
        check_null(tsearch(&new_event, &events, _sort_event));
    }
    // search for intersections
}

int main() {
    // input
    init_input();
    size_t line_cnt;
    struct draw_line *lines = parse_draw_lines(&line_cnt);
    // try calc
    void *inter;
    calc(lines, line_cnt, &inter);
    return 0;
}
