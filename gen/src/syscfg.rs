use drone_svd::{Device, Field, Register};

pub fn patch(dev: &mut Device) {
    let periph = dev.periph("SYSCFG");
    split_reg(periph.reg("PROC_IN_SYNC_BYPASS"), 30, |i| format!("PROC_IN_SYNC_BYPASS{i}"));
    let qspi_name = |i| match i {
        0 => "SCLK",
        1 => "SS",
        2 => "SD0",
        3 => "SD1",
        4 => "SD2",
        5 => "SD3",
        _ => unreachable!(),
    };
    split_reg(periph.reg("PROC_IN_SYNC_BYPASS_HI"), 6, |i| {
        format!("PROC_IN_SYNC_BYPASS_HI_{}", qspi_name(i))
    });
}

fn split_reg(reg: &mut Register, count: u32, name: impl Fn(u32) -> String) {
    let field = Field::default();
    for i in 0..count {
        let mut field = field.clone();
        field.name = name(i);
        field.bit_range = None;
        field.bit_width = Some(1);
        field.bit_offset = Some(i);
        reg.add_field(field);
    }
}
