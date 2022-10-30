use drone_svd::{Device, Register};

pub fn patch(dev: &mut Device) {
    let periph = dev.periph("SIO");
    split_gpio(periph.reg("GPIO_IN"), "GPIO_IN", |i| format!("GPIO{i}_IN"));
    split_gpio(periph.reg("GPIO_OUT"), "GPIO_OUT", |i| format!("GPIO{i}_OUT"));
    split_gpio(periph.reg("GPIO_OUT_SET"), "GPIO_OUT_SET", |i| format!("GPIO{i}_OUT_SET"));
    split_gpio(periph.reg("GPIO_OUT_CLR"), "GPIO_OUT_CLR", |i| format!("GPIO{i}_OUT_CLR"));
    split_gpio(periph.reg("GPIO_OUT_XOR"), "GPIO_OUT_XOR", |i| format!("GPIO{i}_OUT_XOR"));
    split_gpio(periph.reg("GPIO_OE"), "GPIO_OE", |i| format!("GPIO{i}_OE"));
    split_gpio(periph.reg("GPIO_OE_SET"), "GPIO_OE_SET", |i| format!("GPIO{i}_OE_SET"));
    split_gpio(periph.reg("GPIO_OE_CLR"), "GPIO_OE_CLR", |i| format!("GPIO{i}_OE_CLR"));
    split_gpio(periph.reg("GPIO_OE_XOR"), "GPIO_OE_XOR", |i| format!("GPIO{i}_OE_XOR"));
}

fn split_gpio(reg: &mut Register, field: &str, name: impl Fn(u32) -> String) {
    let field = reg.remove_field(field);
    for i in 0..30 {
        let mut field = field.clone();
        field.name = name(i);
        field.bit_range = None;
        field.bit_width = Some(1);
        field.bit_offset = Some(i);
        reg.add_field(field);
    }
}
