#include <stdio.h>
#include <stdlib.h>
#include <memory.h>

#include "ci.h"

extern char *p, *lp;
extern char *data;

extern int tk, src, line, *le, *e, ival, *id, *sym;

void next() {
    char *pp;
    while (1) {
        if (!(tk = *p)) return;
        ++p;
        if (tk == '\n') {
            // use callback to print compile info
            // TODO: fix LEV opcode of function display bug
            if (src) {
                printf("%d: %.*s", line, (int)(p - lp), lp);
                lp = p;
                while (le < e) {
                    printf("%8.4s",&op_codes[*++le * 5]);
                    if (*le <= ADJ) {
                        printf(" %d\n", *++le);
                    } else {
                        printf("\n");
                    }
                }
            }
            ++line;
        } else if (tk == '#') {
            while (*p != 0 && *p != '\n') ++p;
        } else if ((tk >= 'a' && tk <= 'z') || (tk >= 'A' && tk <= 'Z') || tk == '_') {
            pp = p - 1;
            while ((*p >= 'a' && *p <= 'z') || (*p >= 'A' && *p <= 'Z') || (*p >= '0' && *p <= '9') || *p == '_')
                tk = tk * 147 + *p++;
            tk = (tk << 6) + (p - pp);
            id = sym;
            while (id[Tk]) {
                if (tk == id[Hash] && !memcmp((char *)id[Name], pp, p - pp)) { tk = id[Tk]; return; }
                id = id + Idsz;
            }
            id[Name] = (int)pp;
            id[Hash] = tk;
            tk = id[Tk] = Id;
            return;
        } else if (tk >= '0' && tk <= '9') {
            ival = tk - '0';
            while (*p >= '0' && *p <= '9') ival = ival * 10 + *p++ - '0';
            tk = Num;
            return;
        } else if (tk == '/') {
            if (*p == '/') {
                ++p;
                while (*p != 0 && *p != '\n') ++p;
            } else {
                tk = Div;
                return;
            }
        } else if (tk == '\'' || tk == '"') {
            pp = data;
            while (*p != 0 && *p != tk) {
                if ((ival = *p++) == '\\') {
                    if ((ival = *p++) == 'n') ival = '\n';
                }
                if (tk == '"') *data++ = ival;
            }
            ++p;
            if (tk == '"') ival = (int)pp; else tk = Num;
            return;
        } else if (tk == '=') {
            if (*p == '=') {
                ++p; tk = Eq;
            } else {
                tk = Assign;
            }
            return;
        } else if (tk == '+') {
            if (*p == '+') { ++p; tk = Inc; } else tk = Add;
            return;
        } else if (tk == '-') {
            if (*p == '-') { ++p; tk = Dec; } else tk = Sub;
            return;
        } else if (tk == '!') {
            if (*p == '=') {
                ++p;
                tk = Ne;
            }
            return;
        } else if (tk == '<') {
            if (*p == '=') {
                ++p;
                tk = Le;
            } else if (*p == '<') {
                ++p; tk = Shl;
            } else {
                tk = Lt;
            }
            return;
        } else if (tk == '>') {
            if (*p == '=') {
                ++p;
                tk = Ge;
            } else if (*p == '>') {
                ++p;
                tk = Shr;
            } else {
                tk = Gt;
            }
            return;
        } else if (tk == '|') {
            if (*p == '|') {
                ++p;
                tk = Lor;
            } else {
                tk = Or;
            }
            return;
        } else if (tk == '&') {
            if (*p == '&') {
                ++p;
                tk = Lan;
            } else {
                tk = And;
            }
            return;
        } else if (tk == '^') {
            tk = Xor;
            return;
        } else if (tk == '%') {
            tk = Mod;
            return;
        } else if (tk == '*') {
            tk = Mul;
            return;
        } else if (tk == '[') {
            tk = Brak;
            return;
        } else if (tk == '?') {
            tk = Cond;
            return;
        }
        else if (tk == '~' || tk == ';' || tk == '{' || tk == '}'
                || tk == '(' || tk == ')' || tk == ']' || tk == ','
                || tk == ':') {
            return;
        }
    }
}

