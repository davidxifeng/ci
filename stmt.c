#include <stdio.h>
#include <stdlib.h>

#include "ci.h"

extern char *p, *lp;
extern char *data;

extern int tk, src, line, *le, *e, ival, *id, *sym;

void stmt() {
    int *a, *b;
    if (tk == If) {
        next();
        if (tk == '(') {
            next();
        } else {
            printf("%d: open paren expected\n", line);
            exit(-1);
        }
        expr(Assign);
        if (tk == ')') {
            next();
        } else {
            printf("%d: close paren expected\n", line);
            exit(-1);
        }
        *++e = BZ; b = ++e;
        stmt();
        if (tk == Else) {
            *b = (int)(e + 3); *++e = JMP; b = ++e;
            next();
            stmt();
        }
        *b = (int)(e + 1);
    } else if (tk == While) {
        next();
        a = e + 1;
        if (tk == '(') {
            next();
        } else {
            printf("%d: open paren expected\n", line);
            exit(-1);
        }
        expr(Assign);
        if (tk == ')') {
            next();
        } else {
            printf("%d: close paren expected\n", line);
            exit(-1);
        }
        *++e = BZ; b = ++e;
        stmt();
        *++e = JMP; *++e = (int)a;
        *b = (int)(e + 1);
    } else if (tk == Return) {
        next();
        if (tk != ';') expr(Assign);
        *++e = LEV;
        if (tk == ';') {
            next();
        } else {
            printf("%d: semicolon expected\n", line);
            exit(-1);
        }
    } else if (tk == '{') {
        next();
        while (tk != 0 && tk != '}') {
            stmt();
        }
        next();
    } else if (tk == ';') {
        next();
    } else {
        expr(Assign);
        if (tk == ';') {
            next();
        } else {
            printf("%d: semicolon expected\n", line);
            exit(-1);
        }
    }
}


