#![feature(allocator_api)]
#![feature(slice_ptr_get)]
#![feature(sync_unsafe_cell)]
#![no_implicit_prelude]

use ::drone_core::{heap, override_layout, stream};
use ::drone_raspberrypi_pico::{global_heap, global_stream};

override_layout! { r#"
[ram]
main = { origin = 0x20000000, size = "256K" }

[data]
ram = "main"

[stream]
ram = "main"

[stream.core0]
ram = "main"
size = "260"
init-primary = true

[stream.core1]
ram = "main"
size = "260"

[heap.core0]
ram = "main"
size = "10K"
pools = [
    { block = "4", count = "896" },
    { block = "32", count = "80" },
    { block = "256", count = "16" },
]

[heap.core1]
ram = "main"
size = "6K"
pools = [
    { block = "4", count = "896" },
    { block = "32", count = "80" },
]
"# }

stream! {
    layout => core0;
    metadata => pub Stream0;
    instance => pub STREAM0;
}

stream! {
    layout => core1;
    metadata => pub Stream1;
    instance => pub STREAM1;
}

global_stream!(STREAM0, STREAM1);

heap! {
    layout => core0;
    metadata => pub Heap0;
    instance => pub HEAP0;
}

heap! {
    layout => core1;
    metadata => pub Heap1;
    instance => pub HEAP1;
}

global_heap!(HEAP0, HEAP1);
