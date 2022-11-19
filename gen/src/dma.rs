use drone_svd::{Access, Device, Peripheral, Register};

const CH_COUNT: u32 = 12;

pub fn patch(dev: &mut Device) {
    let periph = dev.periph("DMA");
    for i in 0..CH_COUNT {
        set_alt(periph, &format!("CH{i}_READ_ADDR"), &format!("CH{i}_AL1_READ_ADDR"));
        set_alt(periph, &format!("CH{i}_READ_ADDR"), &format!("CH{i}_AL2_READ_ADDR"));
        set_alt(periph, &format!("CH{i}_READ_ADDR"), &format!("CH{i}_AL3_READ_ADDR_TRIG"));
        set_alt(periph, &format!("CH{i}_WRITE_ADDR"), &format!("CH{i}_AL1_WRITE_ADDR"));
        set_alt(periph, &format!("CH{i}_WRITE_ADDR"), &format!("CH{i}_AL2_WRITE_ADDR_TRIG"));
        set_alt(periph, &format!("CH{i}_WRITE_ADDR"), &format!("CH{i}_AL3_WRITE_ADDR"));
        set_alt(periph, &format!("CH{i}_TRANS_COUNT"), &format!("CH{i}_AL1_TRANS_COUNT_TRIG"));
        set_alt(periph, &format!("CH{i}_TRANS_COUNT"), &format!("CH{i}_AL2_TRANS_COUNT"));
        set_alt(periph, &format!("CH{i}_TRANS_COUNT"), &format!("CH{i}_AL3_TRANS_COUNT"));
        set_alt(periph, &format!("CH{i}_CTRL_TRIG"), &format!("CH{i}_AL1_CTRL"));
        set_alt(periph, &format!("CH{i}_CTRL_TRIG"), &format!("CH{i}_AL2_CTRL"));
        set_alt(periph, &format!("CH{i}_CTRL_TRIG"), &format!("CH{i}_AL3_CTRL"));
    }
    split_reg(periph.reg("INTR"), "INTR", |i| format!("INTR{i}"));
    split_reg(periph.reg("INTE0"), "INTE0", |i| format!("INTE0{i}"));
    split_reg(periph.reg("INTF0"), "INTF0", |i| format!("INTF0{i}"));
    split_reg(periph.reg("INTS0"), "INTS0", |i| format!("INTS0{i}"));
    split_reg(periph.reg("INTE1"), "INTE1", |i| format!("INTE1{i}"));
    split_reg(periph.reg("INTF1"), "INTF1", |i| format!("INTF1{i}"));
    split_reg(periph.reg("INTS1"), "INTS1", |i| format!("INTS1{i}"));
    periph.reg("MULTI_CHAN_TRIGGER").field("MULTI_CHAN_TRIGGER").access = Some(Access::WriteOnly);
    split_reg(periph.reg("MULTI_CHAN_TRIGGER"), "MULTI_CHAN_TRIGGER", |i| {
        format!("MULTI_CHAN_TRIGGER{i}")
    });
    split_reg(periph.reg("CHAN_ABORT"), "CHAN_ABORT", |i| format!("CHAN_ABORT{i}"));
}

fn set_alt(periph: &mut Peripheral, main_name: &str, alt_name: &str) {
    let fields = periph.reg(main_name).fields().map(|field| field.clone()).collect::<Vec<_>>();
    let alt = periph.reg(alt_name);
    alt.alternate_register = Some(main_name.to_string());
    for field in fields {
        alt.add_field(field);
    }
}

fn split_reg(reg: &mut Register, field: &str, name: impl Fn(u32) -> String) {
    let field = reg.remove_field(field);
    for i in 0..CH_COUNT {
        let mut field = field.clone();
        field.name = name(i);
        field.bit_range = None;
        field.bit_width = Some(1);
        field.bit_offset = Some(i);
        reg.add_field(field);
    }
}
