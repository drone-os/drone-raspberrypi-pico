use drone_svd::{Device, Field, Register};

pub fn patch(dev: &mut Device) {
    let periph = dev.periph("SIO");
    split_reg(periph.reg("GPIO_IN"), Some("GPIO_IN"), 30, |i| format!("GPIO{i}_IN"));
    split_reg(periph.reg("GPIO_OUT"), Some("GPIO_OUT"), 30, |i| format!("GPIO{i}_OUT"));
    split_reg(periph.reg("GPIO_OUT_SET"), Some("GPIO_OUT_SET"), 30, |i| format!("GPIO{i}_OUT_SET"));
    split_reg(periph.reg("GPIO_OUT_CLR"), Some("GPIO_OUT_CLR"), 30, |i| format!("GPIO{i}_OUT_CLR"));
    split_reg(periph.reg("GPIO_OUT_XOR"), Some("GPIO_OUT_XOR"), 30, |i| format!("GPIO{i}_OUT_XOR"));
    split_reg(periph.reg("GPIO_OE"), Some("GPIO_OE"), 30, |i| format!("GPIO{i}_OE"));
    split_reg(periph.reg("GPIO_OE_SET"), Some("GPIO_OE_SET"), 30, |i| format!("GPIO{i}_OE_SET"));
    split_reg(periph.reg("GPIO_OE_CLR"), Some("GPIO_OE_CLR"), 30, |i| format!("GPIO{i}_OE_CLR"));
    split_reg(periph.reg("GPIO_OE_XOR"), Some("GPIO_OE_XOR"), 30, |i| format!("GPIO{i}_OE_XOR"));

    let qspi_name = |i| match i {
        0 => "SCLK",
        1 => "SS",
        2 => "SD0",
        3 => "SD1",
        4 => "SD2",
        5 => "SD3",
        _ => unreachable!(),
    };
    split_reg(periph.reg("GPIO_HI_IN"), Some("GPIO_HI_IN"), 6, |i| {
        format!("GPIO_{}_HI_IN", qspi_name(i))
    });
    split_reg(periph.reg("GPIO_HI_OUT"), Some("GPIO_HI_OUT"), 6, |i| {
        format!("GPIO_{}_HI_OUT", qspi_name(i))
    });
    split_reg(periph.reg("GPIO_HI_OUT_SET"), Some("GPIO_HI_OUT_SET"), 6, |i| {
        format!("GPIO_{}_HI_OUT_SET", qspi_name(i))
    });
    split_reg(periph.reg("GPIO_HI_OUT_CLR"), Some("GPIO_HI_OUT_CLR"), 6, |i| {
        format!("GPIO_{}_HI_OUT_CLR", qspi_name(i))
    });
    split_reg(periph.reg("GPIO_HI_OUT_XOR"), Some("GPIO_HI_OUT_XOR"), 6, |i| {
        format!("GPIO_{}_HI_OUT_XOR", qspi_name(i))
    });
    split_reg(periph.reg("GPIO_HI_OE"), Some("GPIO_HI_OE"), 6, |i| {
        format!("GPIO_{}_HI_OE", qspi_name(i))
    });
    split_reg(periph.reg("GPIO_HI_OE_SET"), Some("GPIO_HI_OE_SET"), 6, |i| {
        format!("GPIO_{}_HI_OE_SET", qspi_name(i))
    });
    split_reg(periph.reg("GPIO_HI_OE_CLR"), Some("GPIO_HI_OE_CLR"), 6, |i| {
        format!("GPIO_{}_HI_OE_CLR", qspi_name(i))
    });
    split_reg(periph.reg("GPIO_HI_OE_XOR"), Some("GPIO_HI_OE_XOR"), 6, |i| {
        format!("GPIO_{}_HI_OE_XOR", qspi_name(i))
    });

    split_reg(periph.reg("SPINLOCK_ST"), None, 32, |i| format!("SPINLOCK{i}_ST"));
}

fn split_reg(reg: &mut Register, field: Option<&str>, count: u32, name: impl Fn(u32) -> String) {
    let field = if let Some(field) = field { reg.remove_field(field) } else { Field::default() };
    for i in 0..count {
        let mut field = field.clone();
        field.name = name(i);
        field.bit_range = None;
        field.bit_width = Some(1);
        field.bit_offset = Some(i);
        reg.add_field(field);
    }
}
