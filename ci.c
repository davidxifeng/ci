// ci.c - a little C interpreter

#include <stdio.h>
#include <stdlib.h>
#include <memory.h>

#include "ci.h"


char *p, *lp, // current position in source code
    *data;    // data/bss pointer

char * bd;

int *be;      // base address of text segment
int *e, *le,  // current position in emitted code
    *id,      // currently parsed indentifier
    *sym,     // symbol table (simple list of identifiers)
    ival,     // current token value
    ty,       // current expression type
    loc,      // local variable offset
    line,     // current line number
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

int main(int argc, char **argv) {
    int poolsz;
    int i; // temps

    int debug = 0;    // print executed instructions

    --argc; ++argv;
    if (argc > 0 && **argv == '-' && (*argv)[1] == 's') { src = 1; --argc; ++argv; }
    if (argc > 0 && **argv == '-' && (*argv)[1] == 'd') { debug = 1; --argc; ++argv; }
    if (argc < 1) { printf("usage: c4 [-s] [-d] file ...\n"); return 0; }

    poolsz = 256*1024; // arbitrary size
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

    memset(sym,  0, poolsz);
    memset(e,    0, poolsz);
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
    e[0] = -20250934; // ? magic number meaning

    if ((i = parse()) == 0) {
        //if (src) return 0;
        struct Process * p = create_process(e, be, data, bd, argc, argv);
        save_process("process.bin", p);
        free_process(p);

        struct Process * p2 = load_process("process.bin");
        save_process("process2.bin", p2);
        free_process(p2);

        return run_c(argc, argv, debug);
    } else {
        return i;
    }
}

int save_process(const char * process_file, struct Process * p) {
    FILE * f = fopen(process_file, "wb");

    fwrite(&p->text_size, sizeof(p->text_size), 1, f);
    fwrite(&p->data_size, sizeof(p->data_size), 1, f);
    fwrite(p->be, 1, p->text_size, f);
    fwrite(p->bd, 1, p->data_size, f);

    fclose(f);
    return 0;
}

struct Process * load_process(const char * process_file) {
    FILE * f = fopen(process_file, "rb");

    struct Process * p = malloc(sizeof *p);

    fread(&p->text_size, sizeof(p->text_size), 1, f);
    fread(&p->data_size, sizeof(p->data_size), 1, f);
    p->be = malloc(p->text_size);
    fread(p->be, 1, p->text_size, f);

    p->bd = malloc(p->data_size);
    fread(p->bd, 1, p->data_size, f);

    fclose(f);
    return p;
}

struct Process * create_process(int * e, int * be, char * data, char * bd,
        int argc, char ** argv) {
    struct Process * p = malloc(sizeof *p);

    int ts = (e - be + 1) * sizeof (int);
    p->text_size = ts;
    p->be = malloc(ts);
    memcpy(p->be, be, ts);

    int ds = data - bd;
    p->data_size = ds;
    p->bd = malloc(ds);
    memcpy(p->bd, bd, ds);
    return p;
}

void free_process(struct Process * process) {
    if(process) {
        free(process->be);
        free(process->bd);
        free(process);
    }
}
