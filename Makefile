

ALL_C = src/ci.c src/vm.c src/next.c src/expr.c src/stmt.c src/parse.c src/dump.c

ci_bin:
	clang -Wall -m32 -O -o ci $(ALL_C)

debug_ci_bin:
	clang -g -Wall -m32 -o ci $(ALL_C)

test_ci:
	./ci test/test.c
