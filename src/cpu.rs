use std::num::Wrapping;

use crate::machine::ZebuZ80Bus;

pub struct Z80CPU {
    a: u8, f: u8, a_alt: u8, f_alt: u8,
    b: u8, c: u8, b_alt: u8, c_alt: u8,
    d: u8, e: u8, d_alt: u8, e_alt: u8,
    h: u8, l: u8, h_alt: u8, l_alt: u8,
    i: u8, r: u8,
    ix: u16,
    iy: u16,
    sp: u16,
    pc: Wrapping<u16>,
    t_cycles: u8,
    opcode: u8
}

impl Z80CPU {
    pub fn new() -> Z80CPU {
        Z80CPU {
            a: 0, f: 0, a_alt: 0, f_alt: 0,
            b: 0, c: 0, b_alt: 0, c_alt: 0,
            d: 0, e: 0, d_alt: 0, e_alt: 0,
            h: 0, l: 0, h_alt: 0, l_alt: 0,
            i: 0, r: 0,
            ix: 0,
            iy: 0,
            sp: 0,
            pc: Wrapping(0),
            t_cycles: 0,
            opcode: 0
        }
    }

    pub fn clock(&mut self, bus: &mut ZebuZ80Bus) {
        if self.t_cycles == 0 {
            self.opcode = bus.read(self.pc.0);
            self.pc += Wrapping(1);
            if self.opcode == 0x3e {
                self.t_cycles = 7;
                let n = bus.read(self.pc.0);
                self.pc += Wrapping(1);
                self.a = n;
            } else if self.opcode == 0x21 {
                self.t_cycles = 10;
                let n_low = bus.read(self.pc.0);
                self.pc += Wrapping(1);
                self.l = n_low;
                let n_high = bus.read(self.pc.0);
                self.pc += Wrapping(1);
                self.h = n_high;
            } else if self.opcode == 0x77 {
                self.t_cycles = 7;
                let hl = (u16::from(self.h) << 8) + u16:: from(self.l);
                bus.write(hl, self.a);
            } else {
                self.t_cycles = 4;
            }
        }

        self.t_cycles -= 1;
    }

    pub fn print_state(&self) {
        println!("AF: {:02X}{:02X} AF': {:02X}{:02X}", self.a, self.f, self.a_alt, self.f_alt);
        println!("BC: {:02X}{:02X} BC': {:02X}{:02X}", self.b, self.c, self.b_alt, self.c_alt);
        println!("DE: {:02X}{:02X} DE': {:02X}{:02X}", self.d, self.e, self.d_alt, self.e_alt);
        println!("HL: {:02X}{:02X} HL': {:02X}{:02X}", self.h, self.l, self.h_alt, self.l_alt);
        println!("I: {:02X}, R: {:02X}", self.i, self.r);
        println!("IX: {:04X}", self.ix);
        println!("IY: {:04X}", self.iy);
        println!("SP: {:04X}", self.sp);
        println!("PC: {:04X}", self.pc);
    }
}