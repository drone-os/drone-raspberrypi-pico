/// Defines a global allocator. On core 0 it uses `$heap0`, and on core 1 it
/// uses `$heap1`.
#[allow(clippy::module_name_repetitions)]
#[macro_export]
macro_rules! global_heap {
    ($heap0:path, $heap1:path) => {
        const _: () = {
            struct Heap;

            #[cfg_attr(not(feature = "std"), global_allocator)]
            static HEAP: Heap = Heap;

            unsafe impl ::core::alloc::GlobalAlloc for Heap {
                #[inline(never)]
                #[export_name = "alloc"]
                unsafe fn alloc(&self, layout: ::core::alloc::Layout) -> *mut u8 {
                    if $crate::cpuid() == 0 {
                        ::core::alloc::Allocator::allocate(&$heap0, layout)
                    } else {
                        ::core::alloc::Allocator::allocate(&$heap1, layout)
                    }
                    .map(|ptr| ptr.as_mut_ptr())
                    .unwrap_or(::core::ptr::null_mut())
                }

                #[inline(never)]
                #[export_name = "dealloc"]
                unsafe fn dealloc(&self, ptr: *mut u8, layout: ::core::alloc::Layout) {
                    let ptr = ::core::ptr::NonNull::new_unchecked(ptr);
                    if $crate::cpuid() == 0 {
                        ::core::alloc::Allocator::deallocate(&$heap0, ptr, layout)
                    } else {
                        ::core::alloc::Allocator::deallocate(&$heap1, ptr, layout)
                    }
                }
            }
        };
    };
}
