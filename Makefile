

ALL_C = ci.c vm.c next.c expr.c stmt.c parse.c

ci_bin:
	clang -Wall -m32 -O -o ci $(ALL_C)

debug_ci_bin:
	clang -g -Wall -m32 -O -o ci $(ALL_C)

test_ci:
	./ci test/test.c
