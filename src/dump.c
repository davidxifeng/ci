#include <stdio.h>
#include <stdlib.h>
#include <memory.h>

#include "ci.h"

struct Process * create_process(int * e, int * be, char * data, char * bd, int * sym) {
  struct Process * p = malloc(sizeof *p);

  int ts = (e - be + 1) * sizeof (int);
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

extern char * bd;
extern int *be;

int run_process(int argc, char **argv, int debug) {
  struct Process * pl = load_process(*argv);
  be = pl->be;
  bd = pl->bd;
  int r = run_c(argc, argv, debug, pl->main_addr);
  free_process(pl);
  return r;
}


int save_process(const char * process_file, struct Process * p) {
  FILE * f = fopen(process_file, "wb");

  fwrite(&p->main_addr, sizeof(p->main_addr), 1, f);
  fwrite(&p->text_size, sizeof(p->text_size), 1, f);
  fwrite(&p->data_size, sizeof(p->data_size), 1, f);
  fwrite(p->be, 1, p->text_size, f);
  fwrite(p->bd, 1, p->data_size, f);

  fclose(f);
  return 0;
}

struct Process * load_process(const char * process_file) {
  FILE * f = fopen(process_file, "rb");

  struct Process * p = malloc(sizeof *p);

  fread(&p->main_addr, sizeof(p->main_addr), 1, f);
  fread(&p->text_size, sizeof(p->text_size), 1, f);
  fread(&p->data_size, sizeof(p->data_size), 1, f);
  p->be = malloc(p->text_size);
  fread(p->be, 1, p->text_size, f);

  p->bd = malloc(p->data_size);
  fread(p->bd, 1, p->data_size, f);

  fclose(f);
  return p;
}

void free_process(struct Process * process) {
  if(process) {
    free(process->be);
    free(process->bd);
    free(process);
  }
}


// vim: tabstop=2 shiftwidth=2 softtabstop=2
