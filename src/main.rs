use device_query::{DeviceQuery, DeviceState, Keycode};
use std::num::Wrapping;

struct ZebuZ80CPU {
    a: u8, f: u8, a_alt: u8, f_alt: u8,
    b: u8, c: u8, b_alt: u8, c_alt: u8,
    d: u8, e: u8, d_alt: u8, e_alt: u8,
    h: u8, l: u8, h_alt: u8, l_alt: u8,
    i: u8, r: u8,
    ix: u16,
    iy: u16,
    sp: u16,
    pc: Wrapping<u16>
}

impl ZebuZ80CPU {
    pub fn nop(&mut self) {
        self.pc += Wrapping(1);
    }
}

fn print_state(cpu: &ZebuZ80CPU) {
    println!("AF: {:02X}{:02X} AF': {:02X}{:02X}", cpu.a, cpu.f, cpu.a_alt, cpu.f_alt);
    println!("BC: {:02X}{:02X} BC': {:02X}{:02X}", cpu.b, cpu.c, cpu.b_alt, cpu.c_alt);
    println!("DE: {:02X}{:02X} DE': {:02X}{:02X}", cpu.d, cpu.e, cpu.d_alt, cpu.e_alt);
    println!("HL: {:02X}{:02X} HL': {:02X}{:02X}", cpu.h, cpu.l, cpu.h_alt, cpu.l_alt);
    println!("I: {:02X}, R: {:02X}", cpu.i, cpu.r);
    println!("IX: {:04X}", cpu.ix);
    println!("IY: {:04X}", cpu.iy);
    println!("SP: {:04X}", cpu.sp);
    println!("PC: {:04X}", cpu.pc);
}

fn main() {
    println!("zebu");
    let device_state = DeviceState::new();
    let mut sys_cycles = Wrapping(0usize);

    let mut cpu = ZebuZ80CPU {
        a: 0, f: 0, a_alt: 0, f_alt: 0,
        b: 0, c: 0, b_alt: 0, c_alt: 0,
        d: 0, e: 0, d_alt: 0, e_alt: 0,
        h: 0, l: 0, h_alt: 0, l_alt: 0,
        i: 0, r: 0,
        ix: 0,
        iy: 0,
        sp: 0,
        pc: Wrapping(0)
    };

    let mut old_keys: Vec<Keycode> = Vec::new();

    loop {
        let new_keys: Vec<Keycode> = device_state.get_keys();
        let pressed_keys: Vec<_> = new_keys.iter().filter(|k| !old_keys.contains(k)).cloned().collect();
        old_keys = new_keys;
        if pressed_keys.contains(&Keycode::S) {
                print_state(&cpu);
                println!("T: {}", sys_cycles);
        }
        if pressed_keys.contains(&Keycode::Space) {
            cpu.nop();
            sys_cycles += Wrapping(4);
            print_state(&cpu);
            println!("T: {}", sys_cycles);
        }
    }
}