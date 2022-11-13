use drone_svd::Device;

pub fn patch(dev: &mut Device) {
    let periph = dev.periph("CLOCKS");
    periph.reg("CLK_SYS_CTRL").field("SRC").force_bits = true;
}
