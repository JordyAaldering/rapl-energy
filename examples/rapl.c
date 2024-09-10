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
    struct EnergyC *rapl;
    rapl = start_rapl();

    sleep(1);

    print_energy(rapl);
    free_energy(rapl);
    return 0;
#else
    return 1;
#endif
}
