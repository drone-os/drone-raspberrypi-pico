#![feature(sync_unsafe_cell)]
#![no_implicit_prelude]

use ::drone_core::{override_layout, stream};
use ::drone_raspberrypi_pico::global_stream;

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

[stream.core1]
ram = "main"
size = "260"
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
