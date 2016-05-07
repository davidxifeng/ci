#include <stdio.h>

int f(int x) {
    return x % 2 ? ++x : 2 + x;
}

int test_brak() {
    int * pi;
    int y;
    pi[y = 2];
    return 0;
}

int main() {
    printf("f x is %d\n", f(2));
    test_brak();
    return 0;
}
