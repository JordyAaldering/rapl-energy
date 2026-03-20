#include <stdbool.h>
#include <stdio.h>
#include <unistd.h>

#ifdef __has_include
# if __has_include(<rapl_energy.h>)
#  define RAPL_ENERGY
#  include <rapl_energy.h>
# endif
#endif

int main()
{
#ifdef RAPL_ENERGY
    struct Rapl *rapl;
    rapl = rapl_start(true);

    sleep(1);

    struct RaplElapsed *res;
    res = rapl_elapsed(&rapl);

    for (uintptr_t i = 0; i < res->len; i++) {
        printf("%s: %f\n", res->keys[i], res->values[i]);
    }

    rapl_elapsed_free(res);
    rapl_free(rapl);
#endif

    return 0;
}
