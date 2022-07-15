#include <assert.h>
#include <memory.h>
#include <stdio.h>
#include <stdlib.h>

#include "ci.h"

struct Process {
	int *be;  // 代码段基地址
	char *bd; // 数据段基地址
	int text_size;
	int data_size;
	int main_addr;
};

struct Process *create_process(int *e, int *be, char *data, char *bd, int *sym) {
	struct Process *p = malloc(sizeof *p);

	int ts = (e - be + 1) * sizeof(int);
	p->text_size = ts;
	p->be = malloc(ts);
	memcpy(p->be, be, ts);

	int ds = data - bd;
	p->data_size = ds;
	p->bd = malloc(ds);
	memcpy(p->bd, bd, ds);

	int *id = sym;
	while (id[Tk]) {
		if (!memcmp((const void *)id[Name], "main", 4)) {
			break;
		}
		id = id + Idsz;
	}

	if (!(p->main_addr = id[Val])) {
		printf("main() not defined\n");
		// TODO clean up
		return NULL;
	}
	return p;
}

extern char *bd;
extern int *be;

void free_process(struct Process *process) {
	if (process) {
		free(process->be);
		free(process->bd);
		free(process);
	}
}

int save_process(const char *process_file, int *e, int *be, char *data, char *bd, int *sym) {
	struct Process *p = create_process(e, be, data, bd, sym);
	if (p == NULL)
		return -1;

	FILE *f = fopen(process_file, "wb");

	fwrite(&p->main_addr, sizeof(p->main_addr), 1, f);
	fwrite(&p->text_size, sizeof(p->text_size), 1, f);
	fwrite(&p->data_size, sizeof(p->data_size), 1, f);
	fwrite(p->be, 1, p->text_size, f);
	fwrite(p->bd, 1, p->data_size, f);

	fclose(f);

	free_process(p);

	return 0;
}

struct Process *load_process(const char *process_file) {
	FILE *f = fopen(process_file, "rb");
	if (!f)
		return NULL;

	// 这些代码质量太差了，仅仅是可以跑通预期的正常流程，几乎没有考虑错误处理，资源释放

	struct Process *p = malloc(sizeof *p);

	size_t rr;
	// just for silent warning
	rr = fread(&p->main_addr, sizeof(p->main_addr), 1, f);
	assert(rr == sizeof(p->main_addr));
	rr = fread(&p->text_size, sizeof(p->text_size), 1, f);
	rr = fread(&p->data_size, sizeof(p->data_size), 1, f);
	p->be = malloc(p->text_size);
	rr = fread(p->be, 1, p->text_size, f);

	p->bd = malloc(p->data_size);
	rr = fread(p->bd, 1, p->data_size, f);

	fclose(f);
	return p;
}

int run_process(int argc, char **argv, int debug) {
	struct Process *pl = load_process(*argv);
	if (!pl)
		return -1;

	be = pl->be;
	bd = pl->bd;
	int r = run_c(argc, argv, debug, pl->main_addr);
	free_process(pl);
	return r;
}