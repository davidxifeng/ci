
#include <stdio.h>
#include <stdlib.h>
#include <memory.h>
#include "ci.h"

extern int * sym;

int run_c(int argc, char **argv, int debug) {
    int *pc, *sp, *bp, a, cycle; // vm registers

    int *id = sym, *idmain;
    while (id[Tk]) {
        if (!memcmp((char *)id[Name], "main", 4)) {
            idmain = id;
            break;
        }
        id = id + Idsz;
    }

    if (!(pc = (int *)idmain[Val])) {
        printf("main() not defined\n");
        return -1;
    }

    int poolsz = 256*1024; // arbitrary size
    if (!(sp = malloc(poolsz))) {
        printf("could not malloc(%d) stack area\n", poolsz);
        return -1;
    }

    int i, *t; // temps
    // setup stack
    sp    = (int *)((int)sp + poolsz);
    *--sp = EXIT; // call exit if main returns
    *--sp = PSH;
    t     = sp;
    *--sp = argc;
    *--sp = (int)argv;
    *--sp = (int)t;

    // run...
    cycle = 0;
    while (1) {
        i = *pc++; ++cycle;
        if (debug) {
        printf("%d> %.4s", cycle,
            &"LEA ,IMM ,JMP ,JSR ,BZ  ,BNZ ,ENT ,ADJ ,LEV ,LI  ,LC  ,SI  ,SC  ,PSH ,"
            "OR  ,XOR ,AND ,EQ  ,NE  ,LT  ,GT  ,LE  ,GE  ,SHL ,SHR ,ADD ,SUB ,MUL ,DIV ,MOD ,"
            "OPEN,READ,CLOS,PRTF,MALC,MSET,MCMP,EXIT,"[i * 5]);
        if (i <= ADJ) printf(" %d\n", *pc); else printf("\n");
        }
        if      (i == LEA) a = (int)(bp + *pc++);                             // load local address
        else if (i == IMM) a = *pc++;                                         // load global address or immediate
        else if (i == JMP) pc = (int *)(*pc);                                 // jump
        else if (i == JSR) { *--sp = (int)(pc + 1); pc = (int *)*pc; }        // jump to subroutine
        else if (i == BZ)  pc = a ? pc + 1 : (int *)*pc;                      // branch if zero
        else if (i == BNZ) pc = a ? (int *)*pc : pc + 1;                      // branch if not zero
        else if (i == ENT) { *--sp = (int)bp; bp = sp; sp = sp - *pc++; }     // enter subroutine
        else if (i == ADJ) sp = sp + *pc++;                                   // stack adjust
        else if (i == LEV) { sp = bp; bp = (int *)*sp++; pc = (int *)*sp++; } // leave subroutine
        else if (i == LI)  a = *(int *)a;                                     // load int
        else if (i == LC)  a = *(char *)a;                                    // load char
        else if (i == SI)  *(int *)*sp++ = a;                                 // store int
        else if (i == SC)  a = *(char *)*sp++ = a;                            // store char
        else if (i == PSH) *--sp = a;                                         // push

        else if (i == OR)  a = *sp++ |  a;
        else if (i == XOR) a = *sp++ ^  a;
        else if (i == AND) a = *sp++ &  a;
        else if (i == EQ)  a = *sp++ == a;
        else if (i == NE)  a = *sp++ != a;
        else if (i == LT)  a = *sp++ <  a;
        else if (i == GT)  a = *sp++ >  a;
        else if (i == LE)  a = *sp++ <= a;
        else if (i == GE)  a = *sp++ >= a;
        else if (i == SHL) a = *sp++ << a;
        else if (i == SHR) a = *sp++ >> a;
        else if (i == ADD) a = *sp++ +  a;
        else if (i == SUB) a = *sp++ -  a;
        else if (i == MUL) a = *sp++ *  a;
        else if (i == DIV) a = *sp++ /  a;
        else if (i == MOD) a = *sp++ %  a;

        else if (i == OPEN) a = (int)fopen((const char *)sp[1], (const char *)*sp);
        else if (i == READ) a = fread((char *)sp[3], sp[2], sp[1], (FILE *)*sp);
        else if (i == CLOS) a = fclose((FILE *)*sp);
        else if (i == PRTF) { t = sp + pc[1]; a = printf((char *)t[-1], t[-2], t[-3], t[-4], t[-5], t[-6]); }
        else if (i == MALC) a = (int)malloc(*sp);
        else if (i == MSET) a = (int)memset((char *)sp[2], sp[1], *sp);
        else if (i == MCMP) a = memcmp((char *)sp[2], (char *)sp[1], *sp);
        else if (i == EXIT) { printf("exit(%d) cycle = %d\n", *sp, cycle); return *sp; }
        else { printf("unknown instruction = %d! cycle = %d\n", i, cycle); return -1; }
    }


}
