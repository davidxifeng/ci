#include <stdio.h>

int c, d, f() {
    printf("f\n");
};

int main()
{
    int *i;
    c = 2;
    i = &c;
    f();
    printf("%d \n", *i);
    return 0;
}
