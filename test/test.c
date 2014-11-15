#include <stdio.h>

int test_hex() {
    int i;
    i = 0xf;
    printf("i is %d\n", i);
    i = 0xFF;
    printf("i is %d\n", i);
    i = 0x01F3c;
    printf("i is %d\n", i);

    return 0;
}

int main() {
    test_hex();
    return 0;
}

