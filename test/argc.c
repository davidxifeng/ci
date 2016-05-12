#include <stdio.h>

// 函数参数列表这块解析不严谨，没有分隔符','也不会报错
int main(int argc, char **argv) {
  while (argc-- > 0) {
    printf("%s\n", *argv++);
  }
  return 0;
}
