#ifndef _RAPL_ENERGY_H_
#define _RAPL_ENERGY_H_

#ifdef __cplusplus
extern "C" {
#endif

extern const int rapl_amd_start(void **rapl_ptr);
extern const int rapl_intel_start(void **rapl_ptr);

extern const int rapl_amd_stop(void *rapl_ptr, void **elapsed_ptr);
extern const int rapl_intel_stop(void *rapl_ptr, void **elapsed_ptr);

extern void rapl_amd_free(void *rapl_ptr, void *elapsed_ptr);
extern void rapl_intel_free(void *rapl_ptr, void *elapsed_ptr);

extern const int rapl_print(void *elapsed_ptr);

#ifdef __cplusplus
}
#endif

#endif /* _RAPL_ENERGY_H_ */
