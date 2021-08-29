#include <stdio.h>
#include <stdlib.h>

#include "crsa.h"

static int basic_test(void);
static int check(BN a, BN b, int line);

int main() {
    printf("Testmodule %s", __FILE__);
    basic_test();
    return 0;
}

static int basic_test(void) {
    SET_ZERO(r1);
    SET_ZERO(r2);
    bn_add(r1, r2, r3);
    check(r1, r2, __LINE__);

    return 0;
}

static int check(BN a, BN b, int line) {
    if (check_value(a)) {
        fprintf(stderr, "Error in check_value(a) in line %d\n", line);
        fprintf(stderr, "check_value(a == %d\n)", check_value(a));
        display_err("a = ", a);
        exit(EXIT_FAILURE);
    }

    if (!equal(a, b)) {
        fprintf(stderr, "Error in add() in line %d\n", line);
        display_err("a = ", a);
        display_err("b = ", b);
        exit(EXIT_FAILURE);
    }
    return 0;
}
