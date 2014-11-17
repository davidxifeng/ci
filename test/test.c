#include <stdio.h>

char c;

// global/local imm load/store
int test_hex() {
    char *pc;
    char **ppc;
    pc = &c;
    ppc = &pc;
    printf("c is %c\n", **ppc);
    return 0;
}

int main() {
    c = 'X';
    test_hex();
    return 0;
}

