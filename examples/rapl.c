#include <stdbool.h>
#include <stdio.h>
#include <unistd.h>

#ifdef __has_include
#if __has_include(<rapl_energy.h>)
#define RAPL_ENERGY
#include <rapl_energy.h>
#endif
#endif

int main()
{
#ifdef RAPL_ENERGY
    struct Rapl *rapl;
    rapl = rapl_start(false);

    sleep(1);

    struct EnergyElapsed *elapsed;
    elapsed = rapl_elapsed(&rapl);

    for (uintptr_t i = 0; i < elapsed->len; i++) {
        printf("%s: %f\n", elapsed->keys[i], elapsed->energy[i]);
    }

    elapsed_free(elapsed);
    rapl_free(rapl);
    return 0;
#else
    return 1;
#endif
}
