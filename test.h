typedef struct value_t {
    int type;
    long id;
} value_t;

typedef struct triple_t {
    value_t subject;
    value_t predicate;
    value_t object;
} triple_t;