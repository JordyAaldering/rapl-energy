#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Energy Energy;

void start_msr(struct Energy **msr_out);

void start_rapl(struct Energy **rapl_out);

void start_ina(struct Energy **ina_out);

void print_energy(struct Energy *energy_in);
