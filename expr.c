#include <stdio.h>
#include <stdlib.h>

#include "ci.h"

extern char *p, *lp;
extern char *data;

extern int tk, src, line, *le, *e, ival, *id, *sym;

extern int
    loc,      // local variable offset
    ty;       // current expression type

void expr(int lev) {
    int t, *d;

    if (!tk) { printf("%d: unexpected eof in expression", line); exit(-1); }
    else if (tk == Num) { *++e = IMM; *++e = ival; next(); ty = INT; }
    else if (tk == '"') {
        *++e = IMM; *++e = ival; next();
        while (tk == '"') next();
        data = (char *)((int)data + 4 & -4); ty = PTR;
    }
    else if (tk == Id) {
        d = id; next();
        if (tk == '(') {
        next();
        t = 0;
        while (tk != 0 && tk != ')') { expr(Assign); *++e = PSH; ++t; if (tk == ',') next(); }
        next();
        if (d[Class] == Sys) *++e = d[Val];
        else if (d[Class] == Fun) { *++e = JSR; *++e = d[Val]; }
        else { printf("%d: bad function call\n", line); exit(-1); }
        if (t) { *++e = ADJ; *++e = t; }
        ty = d[Type];
        } else {
        if (d[Class] == Num) { *++e = IMM; *++e = d[Val]; ty = INT; }
        else {
            if (d[Class] == Loc) { *++e = LEA; *++e = loc - d[Val]; }
            else if (d[Class] == Glo) { *++e = IMM; *++e = d[Val]; }
            else { printf("%d: undefined variable\n", line); exit(-1); }
            *++e = ((ty = d[Type]) == CHAR) ? LC : LI;
        }
        }
    }
    else if (tk == '(') {
        next();
        if (tk == Int || tk == Char) {
        t = (tk == Int) ? INT : CHAR; next();
        while (tk == Mul) { next(); t = t + PTR; }
        if (tk == ')') next(); else { printf("%d: bad cast", line); exit(-1); }
        expr(Inc);
        ty = t;
        }
        else {
        expr(Assign);
        if (tk == ')') next(); else { printf("%d: close paren expected\n", line); exit(-1); }
        }
    }
    else if (tk == Mul) {
        next(); expr(Inc);
        if (ty > INT) ty = ty - PTR; else { printf("%d: bad dereference\n", line); exit(-1); }
        *++e = (ty == CHAR) ? LC : LI;
    }
    else if (tk == And) {
        next(); expr(Inc);
        if (*e == LC || *e == LI) --e; else { printf("%d: bad address-of\n", line); exit(-1); }
        ty = ty + PTR;
    }
    else if (tk == '!') { next(); expr(Inc); *++e = PSH; *++e = IMM; *++e = 0; *++e = EQ; ty = INT; }
    else if (tk == '~') { next(); expr(Inc); *++e = PSH; *++e = IMM; *++e = -1; *++e = XOR; ty = INT; }
    else if (tk == Add) { next(); expr(Inc); ty = INT; }
    else if (tk == Sub) {
        next(); *++e = IMM;
        if (tk == Num) { *++e = -ival; next(); } else { *++e = -1; *++e = PSH; expr(Inc); *++e = MUL; }
        ty = INT;
    }
    else if (tk == Inc || tk == Dec) {
        t = tk; next(); expr(Inc);
        if (*e == LC) { *e = PSH; *++e = LC; }
        else if (*e == LI) { *e = PSH; *++e = LI; }
        else { printf("%d: bad lvalue in pre-increment\n", line); exit(-1); }
        *++e = PSH;
        *++e = IMM; *++e = (ty > PTR) ? 4 : 1;
        *++e = (t == Inc) ? ADD : SUB;
        *++e = (ty == CHAR) ? SC : SI;
    }
    else { printf("%d: bad expression %c\n", line, tk); exit(-1); }

    while (tk >= lev) { // "precedence climbing" or "Top Down Operator Precedence" method
        t = ty;
        if (tk == Assign) {
        next();
        if (*e == LC || *e == LI) *e = PSH; else { printf("%d: bad lvalue in assignment\n", line); exit(-1); }
        expr(Assign); *++e = ((ty = t) == CHAR) ? SC : SI;
        }
        else if (tk == Cond) {
        next();
        *++e = BZ; d = ++e;
        expr(Assign);
        if (tk == ':') next(); else { printf("%d: conditional missing colon\n", line); exit(-1); }
        *d = (int)(e + 3); *++e = JMP; d = ++e;
        expr(Cond);
        *d = (int)(e + 1);
        }
        else if (tk == Lor) { next(); *++e = BNZ; d = ++e; expr(Lan); *d = (int)(e + 1); ty = INT; }
        else if (tk == Lan) { next(); *++e = BZ;  d = ++e; expr(Or);  *d = (int)(e + 1); ty = INT; }
        else if (tk == Or)  { next(); *++e = PSH; expr(Xor); *++e = OR;  ty = INT; }
        else if (tk == Xor) { next(); *++e = PSH; expr(And); *++e = XOR; ty = INT; }
        else if (tk == And) { next(); *++e = PSH; expr(Eq);  *++e = AND; ty = INT; }
        else if (tk == Eq)  { next(); *++e = PSH; expr(Lt);  *++e = EQ;  ty = INT; }
        else if (tk == Ne)  { next(); *++e = PSH; expr(Lt);  *++e = NE;  ty = INT; }
        else if (tk == Lt)  { next(); *++e = PSH; expr(Shl); *++e = LT;  ty = INT; }
        else if (tk == Gt)  { next(); *++e = PSH; expr(Shl); *++e = GT;  ty = INT; }
        else if (tk == Le)  { next(); *++e = PSH; expr(Shl); *++e = LE;  ty = INT; }
        else if (tk == Ge)  { next(); *++e = PSH; expr(Shl); *++e = GE;  ty = INT; }
        else if (tk == Shl) { next(); *++e = PSH; expr(Add); *++e = SHL; ty = INT; }
        else if (tk == Shr) { next(); *++e = PSH; expr(Add); *++e = SHR; ty = INT; }
        else if (tk == Add) {
        next(); *++e = PSH; expr(Mul);
        if ((ty = t) > PTR) { *++e = PSH; *++e = IMM; *++e = 4; *++e = MUL;  }
        *++e = ADD;
        }
        else if (tk == Sub) {
        next(); *++e = PSH; expr(Mul);
        if ((ty = t) > PTR) { *++e = PSH; *++e = IMM; *++e = 4; *++e = MUL;  }
        *++e = SUB;
        }
        else if (tk == Mul) { next(); *++e = PSH; expr(Inc); *++e = MUL; ty = INT; }
        else if (tk == Div) { next(); *++e = PSH; expr(Inc); *++e = DIV; ty = INT; }
        else if (tk == Mod) { next(); *++e = PSH; expr(Inc); *++e = MOD; ty = INT; }
        else if (tk == Inc || tk == Dec) {
        if (*e == LC) { *e = PSH; *++e = LC; }
        else if (*e == LI) { *e = PSH; *++e = LI; }
        else { printf("%d: bad lvalue in post-increment\n", line); exit(-1); }
        *++e = PSH; *++e = IMM; *++e = (ty > PTR) ? 4 : 1;
        *++e = (tk == Inc) ? ADD : SUB;
        *++e = (ty == CHAR) ? SC : SI;
        *++e = PSH; *++e = IMM; *++e = (ty > PTR) ? 4 : 1;
        *++e = (tk == Inc) ? SUB : ADD;
        next();
        }
        else if (tk == Brak) {
        next(); *++e = PSH; expr(Assign);
        if (tk == ']') next(); else { printf("%d: close bracket expected\n", line); exit(-1); }
        if (t > PTR) { *++e = PSH; *++e = IMM; *++e = 4; *++e = MUL;  }
        else if (t < PTR) { printf("%d: pointer type expected\n", line); exit(-1); }
        *++e = ADD;
        *++e = ((ty = t - PTR) == CHAR) ? LC : LI;
        }
        else { printf("%d: compiler error tk=%d\n", line, tk); exit(-1); }
    }
}

