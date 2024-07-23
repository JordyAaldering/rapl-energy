#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Energy Energy;

void msr(struct Energy **msr_out);

void rapl(struct Energy **rapl_out);

void ina(struct Energy **ina_out);

uintptr_t elapsed(struct Energy *energy, double **elapsed_out);

void print_energy(struct Energy *energy);
