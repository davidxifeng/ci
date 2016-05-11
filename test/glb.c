#include <stdio.h>

int x;
int y;

int p(const char * fmt, int x, int y, int sum) {
  printf(fmt, x, y, sum);
  return 0;
}

int main(int argc, char const **argv) {
  x = 1;
  y = 2;
  return p("x is %d, y is %d, sum is %d\n", x, y, x + y);
}
