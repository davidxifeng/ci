
ci_bin:
	clang -m32 -O -o ci ci.c vm.c

c4_bin:
	clang -Wno-return-type -m32 -O -o c4 c4.c
