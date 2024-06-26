#include<unistd.h>

extern const int rapl_amd_start(void **rapl_ptr);
extern const int rapl_amd_stop(void *rapl_ptr, void **elapsed_ptr);
extern const void rapl_amd_free(void *rapl_ptr, void *elapsed_ptr);
extern const int rapl_print(void *elapsed_ptr);

// gcc -o amdc rapl-energy-c/examples/amd.c  target/debug/librapl_energy_c.a
int main()
{
    void *rapl_ptr, *elapsed_ptr;

    rapl_amd_start(&rapl_ptr);

    sleep(1);

    rapl_amd_stop(rapl_ptr, &elapsed_ptr);
    rapl_print(elapsed_ptr);

    // TODO: This segfaults
    rapl_amd_free(rapl_ptr, elapsed_ptr);
    rapl_ptr = NULL;
    elapsed_ptr = NULL;

    return 0;
}
