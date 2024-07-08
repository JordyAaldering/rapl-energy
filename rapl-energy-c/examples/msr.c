#include<unistd.h>

extern void start_msr(void **msr_out);
extern void print_energy(void *msr_in);

// gcc rapl-energy-c/examples/msr.c target/debug/librapl_energy_c.a
int main()
{
    void *msr;
    start_msr(&msr);

    sleep(1);

    print_energy(msr);
    return 0;
}
