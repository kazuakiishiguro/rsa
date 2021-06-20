#include "rsa.h"

#ifndef EXP
#define EXP 65537
#endif


int int_sqrt(double x) {
    return floor(sqrt(x));
}

bool is_prime(int n) {
    if (n <= 1)
        return false;

    int a = int_sqrt((double) n);

    while (a > 1) {
        if ((n % a) == 0)
            break;
        a--;
    }

    return a == 1;
}

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

    printf("%d\n", is_prime(atoi(argv[1])));

    return 0;
}
