#include "test.h"
#include <stdlib.h>

triple_t* allocRandomTriple() {
    triple_t *triple = (triple_t*) malloc(sizeof(triple_t));
    triple->subject.id = 1;
    triple->predicate.id = 2;
    triple->object.id = 3;
    return triple;
}