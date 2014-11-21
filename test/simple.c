#include <stdio.h>

// 2014-11-19 23:01:22 算术运算表达式求值器

/*
 3: ^
 2: * /
 1: + -
 */

int lookahead();
int next();
int is_op(char p);
int get_p(char p);
int op_x(int x, char p, int y);

const char * p;
int tk_value = 0;
char tk; // token类型: +-*/^ N

int eval_expr(int x, int min_precedence) {
    while (lookahead() && is_op(tk) && get_p(tk) >= min_precedence) {
        char op;
        next();
        op = tk;
        if(next() && tk == 'N') {
            int y = tk_value;
            if(lookahead() && is_op(tk) && (get_p(tk) > get_p(op))) {
                char nop = tk;
                y = eval_expr(y, get_p(nop));
            }
            x = op_x(x, op, y);
        } else {
            printf("ERROR! eval_expr: unexpected eof!\n");
        }
    }
    return x;
}

int eval() {
    if (next() && tk == 'N') {
        return eval_expr(tk_value, 0);
    } else {
        return -1;
    }
}

const char * es[] =
    { "1+2*3+4*2^2-1-2"
    , "3*3+2^2"
    , "1+2*3^8-2*3-2+2^4"
    , "1+2*3^2^2-2*3-2+2^4"
    , "1+2*3-1"
    , "2+3*4+5"
    , NULL
    };


int main() {
    const char ** e = es;
    while ((p = *e++)) {
        printf("%s = %d\n", p, eval());
    }
    return 0;
}

int is_op(char p) {
    switch(p) {
        case '+':
        case '-':
        case '*':
        case '/':
        case '^':
            return 1;
        default:
            return 0;
    }
}

int get_p(char p) {
    switch(p) {
        case '+':
        case '-':
            return 1;
        case '*':
        case '/':
            return 2;
        case '^':
            return 3;
        default:
            return 0;
    }
}

int pow_x(int x, int y) {
    int r = 1;
    while (y--) {
        r = x * r;
    }
    return r;
}

int op_x(int x, char c, int y) {
    int r;
    switch(c) {
        case '+': r = x + y; break;
        case '-': r = x - y; break;
        case '*': r = x * y; break;
        case '/': r = x / y; break;
        case '^': r = pow_x(x, y); break;
        default: printf("ERROR! op %c \n", c); r = 0;;
    }
    //printf("------------********** op_x %d %c %d = %d\n", x, c, y, r);
    return r;
}

int next() {
    char c = *p++;
    switch(c) {
        case '+': tk = '+'; break;
        case '-': tk = '-'; break;
        case '*': tk = '*'; break;
        case '/': tk = '/'; break;
        case '^': tk = '^'; break;
        default:
            if (c >= '0' && c <= '9') {
                tk_value = c - '0';
                tk = 'N';
                return 1;
            } else if(c == '\0') {
                return 0;
            } else {
                printf("ERROR! next: invalid char: %c!\n", c);
                return 0;
            }
    }
    return 1;
}

int lookahead() {
    int r = next();
    --p;
    return r;
}
