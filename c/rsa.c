#include <stdio.h>
#include <stdlib.h>
#include <inttypes.h>
#include "rsa.h"

int gcd (int a, int b) {
    while (b != 0) {
        int t = b;
        b = a % b;
        a = t;
    }
    return a;
}

int main(int argc, char **argv) {
    if (argc != 3) {
        fprintf(stderr, "%s: invalid number of arguments\n", argv[0]);
        return 1;
    }

    printf("%d\n", gcd(atoi(argv[1]), atoi(argv[2])));

    return 0;
}
