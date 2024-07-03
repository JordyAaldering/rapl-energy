#ifndef _RAPL_ENERGY_H_
#define _RAPL_ENERGY_H_

#ifdef __cplusplus
extern "C" {
#endif

extern void *start_msr(void **msr_out);
extern void *start_rapl(void **rapl_out);
extern void print_energy(void *energy_in);
extern void free_energy(void *energy_in);

#ifdef __cplusplus
}
#endif

#endif /* _RAPL_ENERGY_H_ */
