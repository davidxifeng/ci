#include <stdio.h>

int c;

int main()
{
    int *i;
    c = 2;
    i = &c;
    printf("%d \n", *i);
    return 0;
}
