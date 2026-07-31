use std::os::raw::{c_int, c_void};

// -----------------------------------------------------------------------------
// Low-Level C Foreign Function Interface (FFI)
// -----------------------------------------------------------------------------
extern "C" {
    /// Direct binding to the C thermite teardown function.
    ///
    /// # Safety
    /// Must be called with a valid pointer to memory allocated for heap_ptr,
    /// its precise byte length, and an exit code for direct kernel termination.
    fn execute_thermite_teardown(heap_ptr: *mut c_void, heap_len: usize, exit_code: c_int) -> !;
}

// -----------------------------------------------------------------------------
// Safe Rust Abstraction
// -----------------------------------------------------------------------------

/// Safe wrapper around the volatile C memory scrub and teardown sequence.
pub struct ThermiteTeardown;

impl ThermiteTeardown {
    /// Executes the volatile teardown over a byte buffer before issuing 
    /// a kernel-level hard process exit.
    ///
    /// # Behavior
    /// 1. Volatile double-pass zero/entropy overwrite of target memory[cite: 129, 140].
    /// 2. Direct page unmapping (`munmap` / `VirtualFree`)[cite: 130, 141].
    /// 3. Immediate kernel syscall termination (`SYS_exit_group` / `_exit`)[cite: 131, 141].
    ///
    /// This function **never returns**.
    pub fn trigger(heap: &mut [u8], exit_code: i32) -> ! {
        let ptr = heap.as_mut_ptr() as *mut c_void;
        let len = heap.len();

        unsafe {
            // Hand execution off to low-level C thermite routine
            execute_thermite_teardown(ptr, len, exit_code as c_int);
        }
    }

    /// Triggers immediate teardown on an arbitrary raw pointer region.
    ///
    /// # Safety
    /// Caller must ensure `ptr` points to valid memory of at least `len` bytes.
    pub unsafe fn trigger_raw(ptr: *mut u8, len: usize, exit_code: i32) -> ! {
        execute_thermite_teardown(ptr as *mut c_void, len, exit_code as c_int);
    }
}