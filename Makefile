
ci_bin:
	clang -Wall -m32 -O -o ci ci.c vm.c next.c expr.c stmt.c

test_ci:
	./ci test/test.c

full_test_ci:
	./ci test/ci.c test/hello.c

c4_bin:
	clang -Wno-return-type -m32 -O -o c4 c4.c
