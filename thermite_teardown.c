#define _GNU_SOURCE
#include <stdint.h>
#include <stddef.h>

#if defined(__unix__) || defined(__APPLE__) || defined(__linux__)
    #include <sys/mman.h>
    #include <unistd.h>
    #include <sys/syscall.h>
#elif defined(_WIN32)
    #include <windows.h>
#endif

/**
 * Compiler-proof memory zeroing function.
 * Standard memset() can be optimized away by dead-store elimination if 
 * the buffer is free'd or unmapped immediately after. Volatile pointer 
 * forces every byte write to hit physical memory.
 */
static void volatile_secure_zero(void *ptr, size_t len) {
    if (ptr == NULL || len == 0) return;

    volatile uint8_t *p = (volatile uint8_t *)ptr;
    while (len--) {
        *p++ = 0x00;
    }

    /* Read memory fence to enforce write completion before proceed */
#if defined(__i386__) || defined(__x86_64__)
    __asm__ __volatile__("" ::: "memory");
#elif defined(__aarch64__)
    __asm__ __volatile__("dmb sy" ::: "memory");
#endif
}

/**
 * Thermite Protocol Teardown
 * 
 * @param heap_ptr  Pointer to the allocated WASM runtime heap/memory block
 * @param heap_len  Exact byte length of the allocated memory region
 * @param exit_code Signal exit code to return directly to OS kernel
 */
void execute_thermite_teardown(void *heap_ptr, size_t heap_len, int exit_code) {
    /* Step 1: Volatile double-pass wipe (0x00 then 0xFF) to corrupt residual charge */
    if (heap_ptr != NULL && heap_len > 0) {
        volatile_secure_zero(heap_ptr, heap_len);

        /* Secondary fill pass with 0xFF for entropy destruction */
        volatile uint8_t *p = (volatile uint8_t *)heap_ptr;
        for (size_t i = 0; i < heap_len; i++) {
            p[i] = 0xFF;
        }

        /* Final Zero-fill */
        volatile_secure_zero(heap_ptr, heap_len);

        /* Step 2: Unmap pages directly from kernel virtual memory */
#if defined(__unix__) || defined(__APPLE__) || defined(__linux__)
        munmap(heap_ptr, heap_len);
#elif defined(_WIN32)
        VirtualFree(heap_ptr, 0, MEM_RELEASE);
#endif
    }

    /* Step 3: Direct Syscall Termination
     * We purposefully bypass exit(), exit handlers (atexit), and standard 
     * libc cleanup frames to ensure no stack trace or memory dump occurs.
     */
#if defined(__linux__) && defined(SYS_exit_group)
    /* Kills all threads in the thread group instantly */
    syscall(SYS_exit_group, exit_code);
#elif defined(__unix__) || defined(__APPLE__)
    _exit(exit_code);
#elif defined(_WIN32)
    ExitProcess((UINT)exit_code);
#else
    _exit(exit_code);
#endif

    /* Unreachable safeguard */
    while (1) { __asm__ __volatile__(""); }
}