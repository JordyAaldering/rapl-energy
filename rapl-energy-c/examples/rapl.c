#include<unistd.h>

extern void start_rapl(void **rapl_out);
extern void print_energy(void *rapl_in);

// gcc rapl-energy-c/examples/rapl.c target/debug/librapl_energy_c.a
int main()
{
    void *rapl;
    start_rapl(&rapl);

    sleep(1);

    print_energy(rapl);
    return 0;
}
