#include <unistd.h>

#include "rapl_energy.h"

int main()
{
    struct EnergyC *rapl;

    rapl = start_rapl();
    sleep(1);
    print_energy(rapl);

    free_energy(rapl);

    return 0;
}
