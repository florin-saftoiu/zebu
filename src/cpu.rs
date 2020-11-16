use std::num::Wrapping;

use crate::machine::Z80Bus;

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

pub struct Z80CPUState {
    pub a: u8, pub f: u8, pub a_alt: u8, pub f_alt: u8,
    pub d: u8, pub e: u8, pub d_alt: u8, pub e_alt: u8,
    pub h: u8, pub l: u8, pub h_alt: u8, pub l_alt: u8,
    pub b: u8, pub c: u8, pub b_alt: u8, pub c_alt: u8,
    pub i: u8, pub r: u8,
    pub ix: u16,
    pub iy: u16,
    pub sp: u16,
    pub pc: u16
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

    pub fn clock(&mut self, bus: &mut Z80Bus) {
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

    pub fn instruction_complete(&self) -> bool {
        self.t_cycles == 0
    }

    pub fn get_next_instructions(&self, bus: &Z80Bus, nb: usize) -> Vec<String> {
        let mut instructions = vec![];
        let mut pc = self.pc.0;
        while instructions.len() < nb {
            let opcode = bus.read(pc);
            pc += 1;
            if opcode == 0x3e {
                let n = bus.read(pc);
                pc += 1;
                instructions.push(format!("{:04X} LD A, {:X}", pc - 2, n));
            } else if opcode == 0x21 {
                let n_low = bus.read(pc);
                pc += 1;
                let n_high = bus.read(pc);
                pc += 1;
                instructions.push(format!("{:04X} LD HL, {:X}", pc - 3, (u16::from(n_high) << 8) + u16::from(n_low)));
            } else if opcode == 0x77 {
                instructions.push(format!("{:04X} LD (HL), A", pc - 1));
            } else {
                instructions.push(format!("{:04X} NOP", pc - 1));
            }
        }
        instructions
    }

    pub fn get_state(&self) -> Z80CPUState {
        Z80CPUState {
            a: self.a, f: self.f, a_alt: self.a_alt, f_alt: self.f_alt,
            b: self.b, c: self.c, b_alt: self.b_alt, c_alt: self.c_alt,
            d: self.d, e: self.e, d_alt: self.d_alt, e_alt: self.e_alt,
            h: self.h, l: self.l, h_alt: self.h_alt, l_alt: self.l_alt,
            i: self.i, r: self.r,
            ix: self.ix,
            iy: self.iy,
            sp: self.sp,
            pc: self.pc.0
        }
    }
}