#include <stdio.h>

int f(int x) {
    return x % 2 ? ++x : 2 + x;
}

int main() {
    printf("f x is %d\n", f(2));
    printf("f x is %d\n", f(1));
    return 0;
}
