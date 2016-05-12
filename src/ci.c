// ci.c - a little C interpreter

#include <stdio.h>
#include <stdlib.h>
#include <memory.h>

#include "ci.h"


char *p,
   *lp, // current position in source code
   *data;    // data/bss pointer

char * bd;

int *be;      // base address of text segment
int *e, *le,  // current position in emitted code
  *id,      // currently parsed indentifier
  *sym,    // symbol table (simple list of identifiers)
  ival,    // current token value
  ty,      // current expression type
  loc,      // local variable offset
  line,    // current line number
  src;      // print source and assembly flag

enum Token tk;       // current token

// map toEnum [0 .. 127] :: String
char ck_visible(char c) {
  if (c < 33 || c > 126) {
    return ' ';
  } else {
    return c;
  }
}

enum Action {
  RunC = 0,
  Compile,
  RunBinary,
};

int real_main(int, char **, int, enum Action);

int main(int argc, char **argv) {
  int debug = 0;    // print executed instructions
  enum Action action = RunC;

  --argc; ++argv;
  if (argc > 0 && **argv == '-' && (*argv)[1] == 's') {
    --argc; ++argv;
    src = 1;
  }
  if (argc > 0 && **argv == '-' && (*argv)[1] == 'b') {
    --argc; ++argv;
    action = RunBinary;
  }
  if (argc > 0 && **argv == '-' && (*argv)[1] == 'c') {
    --argc; ++argv;
    action = Compile;
  }
  if (argc > 0 && **argv == '-' && (*argv)[1] == 'd') {
    --argc; ++argv;
    debug = 1;
  }
  if (argc < 1) {
    printf("usage: ci [-s|b|c] [-d] file [...]\n");
    printf("example:\n");

    printf(COLOR_GREEN "\tci -s test.c\n" COLOR_RESET);
    printf("\t\tshow compile code\n\n");

    printf(COLOR_GREEN "\tci -b [-d] test.c.bin\n\n" COLOR_RESET);
    printf("\t\trun bytecode\n");

    printf(COLOR_GREEN "\tci [-c] test.c\n" COLOR_RESET);
    printf("\t\tcompile and save c code\n\n");

    printf(COLOR_GREEN "\tci [-d] test.c\n" COLOR_RESET);
    printf("\t\tcompile and run c code\n\n");
    return 0;
  }
  return real_main(argc, argv, debug, action);
}

int real_main(int argc, char **argv, int debug, enum Action action) {
  if (action == RunBinary) {
    return run_process(argc, argv, debug);
  }
  int poolsz = 256*1024; // arbitrary size

  int i; // temps

  if (!(sym = malloc(poolsz))) {
    printf("could not malloc(%d) symbol area\n", poolsz);
    return -1;
  }
  if (!(be = le = e = malloc(poolsz))) {
    printf("could not malloc(%d) text area\n", poolsz);
    return -1;
  }
  if (!(bd = data = malloc(poolsz))) {
    printf("could not malloc(%d) data area\n", poolsz);
    return -1;
  }

  memset(sym, 0, poolsz);
  memset(e,   0, poolsz);
  memset(data, 0, poolsz);

  p = "char else enum if int return while "
    "fopen fread fclose printf malloc memset memcmp exit";

  // add keywords to symbol table
  for (i = Char; i <= While; i++) {
    next();
    id[Tk] = i;
  }

  // add library to symbol table
  i = OPEN;
  while (i <= EXIT) {
    next();
    id[Class] = Sys;
    id[Type] = INT;
    id[Val] = i++;
  }

  if (!(lp = p = malloc(poolsz))) {
    printf("could not malloc(%d) source area\n", poolsz);
    return -1;
  }

  FILE *fd;
  if ((fd = fopen(*argv, "r")) == 0) {
    printf("could not fopen(%s)\n", *argv);
    return -1;
  }

  if ((i = fread(p, 1, poolsz-1, fd)) <= 0) {
    printf("fread() returned %d\n", i);
    return -1;
  }
  p[i] = 0;
  fclose(fd);
  e[0] = 0xfecafeca; // magic number cafecafe (little endian)
  // 0xfecafeca --32位补码表示的数值--> -20250934
  // (1 << 32) - 0xfecafeca == 20250934

  if ((i = parse()) == 0) {
    if (action == Compile) {
      char buf[128];
      snprintf(buf, 128, "%s.bin", *argv);
      save_process(buf, e, be, data, bd, sym);
      return 0;
    }
    if (src) return 0;
    return run_c(argc, argv, debug, -1);
  } else {
    return i;
  }
}

// vim: tabstop=2 shiftwidth=2 softtabstop=2
