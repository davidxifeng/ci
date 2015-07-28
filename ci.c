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

int parse_c() {
    int bt, ty;
    int i;
    // parse declarations
    line = 1;
    next();
    while (tk) {
        bt = INT; // basetype
        if (tk == Int) {
            next();
        } else if (tk == Char) {
            next();
            bt = CHAR;
        } else if (tk == Enum) {
            next();
            if (tk != '{') next();
            if (tk == '{') {
                next();
                i = 0;
                while (tk != '}') {
                    if (tk != Id) {
                        printf("%d: bad enum identifier %d\n", line, tk);
                        return -1;
                    }
                    next();
                    if (tk == Assign) {
                        next();
                        if (tk != Num) {
                            printf("%d: bad enum initializer\n", line);
                            return -1;
                        }
                        i = ival;
                        next();
                    }
                    id[Class] = Num; id[Type] = INT; id[Val] = i++;
                    if (tk == ',') next();
                }
                next();
            }
        }

        while (tk != ';' && tk != '}') {
            ty = bt;
            while (tk == Mul) {
                next();
                ty = ty + PTR;
            }
            if (tk != Id) {
                printf("%d: bad global declaration\n", line);
                return -1;
            }
            if (id[Class]) {
                printf("%d: duplicate global definition\n", line);
                return -1;
            }
            next();
            id[Type] = ty;
            if (tk == '(') { // function
                id[Class] = Fun;
                id[Val] = (int)(e + 1 - be); //(int)(e + 1);
                next(); i = 0;
                while (tk != ')') {
                    ty = INT;
                    if (tk == Int) {
                        next();
                    } else if (tk == Char) {
                        next();
                        ty = CHAR;
                    }
                    while (tk == Mul) {
                        next();
                        ty = ty + PTR;
                    }
                    if (tk != Id) {
                        printf("%d: bad parameter declaration\n", line);
                        return -1;
                    }
                    if (id[Class] == Loc) {
                        printf("%d: duplicate parameter definition\n", line);
                        return -1;
                    }
                    id[HClass] = id[Class];
                    id[Class]  = Loc;
                    id[HType]  = id[Type];
                    id[Type]   = ty;
                    id[HVal]   = id[Val];
                    id[Val]    = i++;
                    next();
                    if (tk == ',') next();
                }
                next();
                if (tk != '{') {
                    printf("%d: bad function definition\n", line);
                    return -1;
                }
                loc = ++i;
                next();
                while (tk == Int || tk == Char) {
                    bt = (tk == Int) ? INT : CHAR;
                    next();
                    while (tk != ';') {
                        ty = bt;
                        while (tk == Mul) {
                            next();
                            ty = ty + PTR;
                        }
                        if (tk != Id) {
                            printf("%d: bad local declaration\n", line);
                            return -1;
                        }
                        if (id[Class] == Loc) {
                            printf("%d: duplicate local definition\n", line);
                            return -1;
                        }
                        id[HClass] = id[Class];
                        id[Class]  = Loc;
                        id[HType]  = id[Type];
                        id[Type]   = ty;
                        id[HVal]   = id[Val];
                        id[Val]    = ++i;
                        next();
                        if (tk == ',') next();
                    }
                    next();
                }
                *++e = ENT; *++e = i - loc;
                while (tk != '}') stmt();
                if (*e != LEV) {
                    *++e = LEV;
                }
                id = sym; // unwind symbol table locals
                while (id[Tk]) {
                    if (id[Class] == Loc) {
                        id[Class] = id[HClass];
                        id[Type] = id[HType];
                        id[Val] = id[HVal];
                    }
                    id = id + Idsz;
                }
            } else {
                id[Class] = Glo;
                id[Val] = data - bd;
                data = data + 4;
                if (tk == ',') next();
            }
        }
        next();
    }
    return 0;
}

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

    if ((i = parse_c()) == 0) {
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
