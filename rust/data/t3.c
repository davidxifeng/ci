
struct st
{
	int (*f)(int);
};

struct st st;

int func()
{
	int x = 1;
	int y = 2;
	st.f(y = 3);
	int c = y + (x == 1 ? x + 1 : x + y);
	return c;
}