/// Defines a global Drone Stream. On core 0 it uses `$stream0`, and on core 1
/// it uses `$stream1`.
#[allow(clippy::module_name_repetitions)]
#[macro_export]
macro_rules! global_stream {
    ($stream0:path, $stream1:path) => {
        const _: () = {
            #[inline]
            #[no_mangle]
            extern "C" fn drone_stream_runtime() -> *mut ::drone_core::_rt::drone_stream::Runtime {
                if $crate::cpuid() == 0 {
                    unsafe { ::core::ptr::addr_of_mut!((*$stream0.get()).runtime) }
                } else {
                    unsafe { ::core::ptr::addr_of_mut!((*$stream1.get()).runtime) }
                }
            }
        };
    };
}
