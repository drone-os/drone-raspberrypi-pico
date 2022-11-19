use drone_svd::{Access, Device};

pub fn patch(dev: &mut Device) {
    let sspicr = dev.periph("SPI0").reg("SSPICR");
    sspicr.field("RTIC").access = Some(Access::WriteOnly);
    sspicr.field("RORIC").access = Some(Access::WriteOnly);
}
