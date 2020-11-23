use std::num::Wrapping;

use super::machine::ReadWrite;

const OPCODES: [(&str, fn(&mut Z80CPU, &mut dyn ReadWrite) -> u8, u8, u8); 256] = [
/*         0                                          1                                           2                                          3                                          4                                          5                                           6                                       7                                             8                                          9                                            a                                          b                                          c                                          d                                          e                                       f                                              */
/* 00 */ ("NOP"    , Z80CPU::nop           , 0, 4), ("???"    , Z80CPU::invalid_opcode, 0,  4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0,  4), ("LD B", Z80CPU::ld_b_n        , 1, 7), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"      , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("LD C", Z80CPU::ld_c_n        , 1, 7), ("???"    , Z80CPU::invalid_opcode, 0, 4), /* 00 */
/* 10 */ ("???"    , Z80CPU::invalid_opcode, 0, 4), ("LD DE"  , Z80CPU::ld_de_nn      , 2, 10), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0,  4), ("LD D", Z80CPU::ld_d_n        , 1, 7), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"      , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("LD E", Z80CPU::ld_e_n        , 1, 7), ("???"    , Z80CPU::invalid_opcode, 0, 4), /* 10 */
/* 20 */ ("???"    , Z80CPU::invalid_opcode, 0, 4), ("LD HL"  , Z80CPU::ld_hl_nn      , 2, 10), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0,  4), ("LD H", Z80CPU::ld_h_n        , 1, 7), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"      , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("LD L", Z80CPU::ld_l_n        , 1, 7), ("???"    , Z80CPU::invalid_opcode, 0, 4), /* 20 */
/* 30 */ ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0,  4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0,  4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"      , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("LD A", Z80CPU::ld_a_n        , 1, 7), ("???"    , Z80CPU::invalid_opcode, 0, 4), /* 30 */
/* 40 */ ("LD B, B", Z80CPU::ld_b_b        , 0, 4), ("LD B, C", Z80CPU::ld_b_c        , 0,  4), ("LD B, D", Z80CPU::ld_b_d        , 0, 4), ("LD B, E", Z80CPU::ld_b_e        , 0, 4), ("LD B, H", Z80CPU::ld_b_h        , 0, 4), ("LD B, L", Z80CPU::ld_b_l        , 0,  4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("LD B, A"   , Z80CPU::ld_b_a        , 0, 4), ("LD C, B", Z80CPU::ld_c_b        , 0, 4), ("LD C, C"  , Z80CPU::ld_c_c        , 0, 4), ("LD C, D", Z80CPU::ld_c_d        , 0, 4), ("LD C, E", Z80CPU::ld_c_e        , 0, 4), ("LD C, H", Z80CPU::ld_c_h        , 0, 4), ("LD C, L", Z80CPU::ld_c_l        , 0, 4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("LD C, A", Z80CPU::ld_c_a        , 0, 4), /* 40 */
/* 50 */ ("LD D, B", Z80CPU::ld_d_b        , 0, 4), ("LD D, C", Z80CPU::ld_d_c        , 0,  4), ("LD D, D", Z80CPU::ld_d_d        , 0, 4), ("LD D, E", Z80CPU::ld_d_e        , 0, 4), ("LD D, H", Z80CPU::ld_d_h        , 0, 4), ("LD D, L", Z80CPU::ld_d_l        , 0,  4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("LD D, A"   , Z80CPU::ld_d_a        , 0, 4), ("LD E, B", Z80CPU::ld_e_b        , 0, 4), ("LD E, C"  , Z80CPU::ld_e_c        , 0, 4), ("LD E, D", Z80CPU::ld_e_d        , 0, 4), ("LD E, E", Z80CPU::ld_e_e        , 0, 4), ("LD E, H", Z80CPU::ld_e_h        , 0, 4), ("LD E, L", Z80CPU::ld_e_l        , 0, 4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("LD E, A", Z80CPU::ld_e_a        , 0, 4), /* 50 */
/* 60 */ ("LD H, B", Z80CPU::ld_h_b        , 0, 4), ("LD H, C", Z80CPU::ld_h_c        , 0,  4), ("LD H, D", Z80CPU::ld_h_d        , 0, 4), ("LD H, E", Z80CPU::ld_h_e        , 0, 4), ("LD H, H", Z80CPU::ld_h_h        , 0, 4), ("LD H, L", Z80CPU::ld_h_l        , 0,  4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("LD H, A"   , Z80CPU::ld_h_a        , 0, 4), ("LD L, B", Z80CPU::ld_l_b        , 0, 4), ("LD L, C"  , Z80CPU::ld_l_c        , 0, 4), ("LD L, D", Z80CPU::ld_l_d        , 0, 4), ("LD L, E", Z80CPU::ld_l_e        , 0, 4), ("LD L, H", Z80CPU::ld_l_h        , 0, 4), ("LD L, L", Z80CPU::ld_l_l        , 0, 4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("LD L, A", Z80CPU::ld_l_a        , 0, 4), /* 60 */
/* 70 */ ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0,  4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0,  4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("LD (HL), A", Z80CPU::ld_reg_hl_a   , 0, 7), ("LD A, B", Z80CPU::ld_a_b        , 0, 4), ("LD A, C"  , Z80CPU::ld_a_c        , 0, 4), ("LD A, D", Z80CPU::ld_a_d        , 0, 4), ("LD A, E", Z80CPU::ld_a_e        , 0, 4), ("LD A, H", Z80CPU::ld_a_h        , 0, 4), ("LD A, L", Z80CPU::ld_a_l        , 0, 4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("LD A, A", Z80CPU::ld_a_a        , 0, 4), /* 70 */
/* 80 */ ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0,  4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0,  4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"      , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), /* 80 */
/* 90 */ ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0,  4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0,  4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"      , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), /* 90 */
/* a0 */ ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0,  4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0,  4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"      , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), /* a0 */
/* b0 */ ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0,  4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0,  4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"      , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), /* b0 */
/* c0 */ ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0,  4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0,  4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"      , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), /* c0 */
/* d0 */ ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0,  4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("PUSH DE", Z80CPU::push_de       , 0, 11), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"      , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), /* d0 */
/* e0 */ ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0,  4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0,  4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"      , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), /* e0 */
/* f0 */ ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0,  4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0,  4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"       , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("LD SP, HL", Z80CPU::ld_sp_hl      , 0, 6), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4), ("???" , Z80CPU::invalid_opcode, 0, 4), ("???"    , Z80CPU::invalid_opcode, 0, 4)  /* f0 */
]; 

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

    pub fn clock(&mut self, bus: &mut dyn ReadWrite) {
        if self.t_cycles == 0 {
            self.opcode = bus.read(self.pc.0);
            self.pc += Wrapping(1);
            self.t_cycles = OPCODES[usize::from(self.opcode)].3;
            self.t_cycles += OPCODES[usize::from(self.opcode)].1(self, bus);
        }

        self.t_cycles -= 1;
    }

    pub fn reset(&mut self) {
        self.pc = Wrapping(0);
        self.i = 0;
        self.r = 0;
        self.t_cycles = 3;
    }

    pub fn instruction_complete(&self) -> bool {
        self.t_cycles == 0
    }

    pub fn get_next_instructions(&self, bus: &dyn ReadWrite, nb: usize) -> Vec<String> {
        let mut instructions = vec![];
        let mut pc = self.pc.0;
        while instructions.len() < nb {
            let opcode = bus.read(pc);
            pc += 1;
            let nb_operands = OPCODES[usize::from(opcode)].2;
            if nb_operands == 0 {
                instructions.push(format!("{:04X}: {}", pc - 1, OPCODES[usize::from(opcode)].0));
            } else if nb_operands == 1 {
                let n = bus.read(pc);
                pc += 1;
                instructions.push(format!("{:04X}: {}, ${:X}", pc - 2, OPCODES[usize::from(opcode)].0, n));
            } else if nb_operands == 2 {
                let n_low = bus.read(pc);
                pc += 1;
                let n_high = bus.read(pc);
                pc += 1;
                instructions.push(format!("{:04X}: {}, ${:X}", pc - 3, OPCODES[usize::from(opcode)].0, (u16::from(n_high) << 8) + u16::from(n_low)));
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

    fn nop(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        0
    }

    fn ld_b_n(&mut self, bus: &mut dyn ReadWrite) -> u8 {
        let n = bus.read(self.pc.0);
        self.pc += Wrapping(1);
        self.b = n;
        0
    }

    fn ld_c_n(&mut self, bus: &mut dyn ReadWrite) -> u8 {
        let n = bus.read(self.pc.0);
        self.pc += Wrapping(1);
        self.c = n;
        0
    }

    fn ld_de_nn(&mut self, bus: &mut dyn ReadWrite) -> u8 {
        let n_low = bus.read(self.pc.0);
        self.pc += Wrapping(1);
        self.e = n_low;
        let n_high = bus.read(self.pc.0);
        self.pc += Wrapping(1);
        self.d = n_high;
        0
    }

    fn ld_d_n(&mut self, bus: &mut dyn ReadWrite) -> u8 {
        let n = bus.read(self.pc.0);
        self.pc += Wrapping(1);
        self.d = n;
        0
    }

    fn ld_e_n(&mut self, bus: &mut dyn ReadWrite) -> u8 {
        let n = bus.read(self.pc.0);
        self.pc += Wrapping(1);
        self.e = n;
        0
    }

    fn ld_hl_nn(&mut self, bus: &mut dyn ReadWrite) -> u8 {
        let n_low = bus.read(self.pc.0);
        self.pc += Wrapping(1);
        self.l = n_low;
        let n_high = bus.read(self.pc.0);
        self.pc += Wrapping(1);
        self.h = n_high;
        0
    }

    fn ld_h_n(&mut self, bus: &mut dyn ReadWrite) -> u8 {
        let n = bus.read(self.pc.0);
        self.pc += Wrapping(1);
        self.h = n;
        0
    }

    fn ld_l_n(&mut self, bus: &mut dyn ReadWrite) -> u8 {
        let n = bus.read(self.pc.0);
        self.pc += Wrapping(1);
        self.l = n;
        0
    }

    fn ld_a_n(&mut self, bus: &mut dyn ReadWrite) -> u8 {
        let n = bus.read(self.pc.0);
        self.pc += Wrapping(1);
        self.a = n;
        0
    }

    fn ld_b_b(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.b = self.b;
        0
    }

    fn ld_b_c(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.b = self.c;
        0
    }

    fn ld_b_d(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.b = self.d;
        0
    }

    fn ld_b_e(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.b = self.e;
        0
    }

    fn ld_b_h(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.b = self.h;
        0
    }

    fn ld_b_l(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.b = self.l;
        0
    }

    fn ld_b_a(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.b = self.a;
        0
    }

    fn ld_c_b(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.c = self.b;
        0
    }

    fn ld_c_c(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.c = self.c;
        0
    }

    fn ld_c_d(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.c = self.d;
        0
    }

    fn ld_c_e(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.c = self.e;
        0
    }

    fn ld_c_h(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.c = self.h;
        0
    }

    fn ld_c_l(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.c = self.l;
        0
    }

    fn ld_c_a(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.c = self.a;
        0
    }

    fn ld_d_b(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.d = self.b;
        0
    }

    fn ld_d_c(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.d = self.c;
        0
    }

    fn ld_d_d(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.d = self.d;
        0
    }

    fn ld_d_e(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.d = self.e;
        0
    }

    fn ld_d_h(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.d = self.h;
        0
    }

    fn ld_d_l(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.d = self.l;
        0
    }

    fn ld_d_a(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.d = self.a;
        0
    }

    fn ld_e_b(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.e = self.b;
        0
    }

    fn ld_e_c(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.e = self.c;
        0
    }

    fn ld_e_d(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.e = self.d;
        0
    }

    fn ld_e_e(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.e = self.e;
        0
    }

    fn ld_e_h(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.e = self.h;
        0
    }

    fn ld_e_l(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.e = self.l;
        0
    }

    fn ld_e_a(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.e = self.a;
        0
    }

    fn ld_h_b(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.h = self.b;
        0
    }

    fn ld_h_c(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.h = self.c;
        0
    }

    fn ld_h_d(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.h = self.d;
        0
    }

    fn ld_h_e(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.h = self.e;
        0
    }

    fn ld_h_h(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.h = self.h;
        0
    }

    fn ld_h_l(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.h = self.l;
        0
    }

    fn ld_h_a(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.h = self.a;
        0
    }

    fn ld_l_b(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.l = self.b;
        0
    }

    fn ld_l_c(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.l = self.c;
        0
    }

    fn ld_l_d(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.l = self.d;
        0
    }

    fn ld_l_e(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.l = self.e;
        0
    }

    fn ld_l_h(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.l = self.h;
        0
    }

    fn ld_l_l(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.l = self.l;
        0
    }

    fn ld_l_a(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.l = self.a;
        0
    }

    fn ld_reg_hl_a(&mut self, bus: &mut dyn ReadWrite) -> u8 {
        let hl = (u16::from(self.h) << 8) + u16::from(self.l);
        bus.write(hl, self.a);
        0
    }

    fn ld_a_b(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.a = self.b;
        0
    }

    fn ld_a_c(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.a = self.c;
        0
    }

    fn ld_a_d(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.a = self.d;
        0
    }

    fn ld_a_e(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.a = self.e;
        0
    }

    fn ld_a_h(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.a = self.h;
        0
    }

    fn ld_a_l(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.a = self.l;
        0
    }

    fn ld_a_a(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        self.a = self.a;
        0
    }

    fn push_de(&mut self, bus: &mut dyn ReadWrite) -> u8 {
        self.sp -= 1;
        bus.write(self.sp, self.d);
        self.sp -= 1;
        bus.write(self.sp, self.e);
        0
    }

    fn ld_sp_hl(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        let hl = (u16::from(self.h) << 8) + u16::from(self.l);
        self.sp = hl;
        0
    }

    fn invalid_opcode(&mut self, _bus: &mut dyn ReadWrite) -> u8 {
        0
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate::*;

    use super::*;
    use super::super::machine::MockReadWrite;

    #[test]
    fn test_nop() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x00);
        
        cpu.reset();
        cpu.t_cycles = 0;
        cpu.clock(&mut mock_bus);

        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_b_0xd9() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x06);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xd9);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.b, 0xd9);
        assert_eq!(1 + cpu.t_cycles, 7);
    }

    #[test]
    fn test_ld_c_0xd9() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x0e);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xd9);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.c, 0xd9);
        assert_eq!(1 + cpu.t_cycles, 7);
    }

    #[test]
    fn test_ld_de_0xbaad() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x11);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xad);
        mock_bus.expect_read().with(eq(2)).returning(|_| 0xba);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.d, 0xba);
        assert_eq!(cpu.e, 0xad);
        assert_eq!(1 + cpu.t_cycles, 10);
    }

    #[test]
    fn test_ld_d_0xd9() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x16);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xd9);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.d, 0xd9);
        assert_eq!(1 + cpu.t_cycles, 7);
    }

    #[test]
    fn test_ld_e_0xd9() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x1e);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xd9);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.e, 0xd9);
        assert_eq!(1 + cpu.t_cycles, 7);
    }

    #[test]
    fn test_ld_hl_0x4001() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x21);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0x01);
        mock_bus.expect_read().with(eq(2)).returning(|_| 0x40);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.h, 0x40);
        assert_eq!(cpu.l, 0x01);
        assert_eq!(1 + cpu.t_cycles, 10);
    }

    #[test]
    fn test_ld_h_0xd9() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x26);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xd9);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.h, 0xd9);
        assert_eq!(1 + cpu.t_cycles, 7);
    }

    #[test]
    fn test_ld_l_0xd9() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x2e);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0xd9);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.l, 0xd9);
        assert_eq!(1 + cpu.t_cycles, 7);
    }

    #[test]
    fn test_ld_a_0x2a() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x3e);
        mock_bus.expect_read().with(eq(1)).returning(|_| 0x2a);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.a, 0x2a);
        assert_eq!(1 + cpu.t_cycles, 7);
    }

    #[test]
    fn test_ld_b_b() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x40);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.b = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.b, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_b_c() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x41);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.c = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.b, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_b_d() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x42);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.d = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.b, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_b_e() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x43);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.e = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.b, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_b_h() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x44);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.h = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.b, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_b_l() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x45);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.l = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.b, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_b_a() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x47);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.a = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.b, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_c_b() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x48);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.b = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.c, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_c_c() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x49);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.c = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.c, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_c_d() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x4a);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.d = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.c, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_c_e() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x4b);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.e = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.c, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_c_h() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x4c);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.h = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.c, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_c_l() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x4d);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.l = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.c, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_c_a() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x4f);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.a = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.c, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_d_b() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x50);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.b = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.d, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_d_c() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x51);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.c = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.d, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_d_d() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x52);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.d = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.d, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_d_e() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x53);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.e = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.d, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_d_h() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x54);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.h = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.d, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_d_l() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x55);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.l = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.d, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_d_a() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x57);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.a = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.d, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_e_b() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x58);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.b = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.e, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_e_c() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x59);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.c = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.e, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_e_d() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x5a);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.d = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.e, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_e_e() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x5b);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.e = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.e, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_e_h() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x5c);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.h = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.e, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_e_l() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x5d);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.l = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.e, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_e_a() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x5f);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.a = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.e, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_h_b() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x60);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.b = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.h, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_h_c() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x61);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.c = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.h, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_h_d() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x62);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.d = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.h, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_h_e() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x63);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.e = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.h, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_h_h() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x64);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.h = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.h, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_h_l() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x65);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.l = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.h, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_h_a() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x67);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.a = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.h, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_l_b() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x68);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.b = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.l, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_l_c() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x69);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.c = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.l, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_l_d() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x6a);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.d = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.l, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_l_e() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x6b);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.e = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.l, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_l_h() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x6c);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.h = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.l, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_l_l() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x6d);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.l = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.l, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_l_a() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x6f);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.a = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.l, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_reg_hl_a() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x77);
        mock_bus.expect_write().with(eq(0x4001), eq(0x2a)).return_const(());

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.h = 0x40;
        cpu.l = 0x01;
        cpu.a = 0x2a;
        cpu.clock(&mut mock_bus);

        assert_eq!(1 + cpu.t_cycles, 7);
    }

    #[test]
    fn test_ld_a_b() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x78);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.b = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.a, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_a_c() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x79);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.c = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.a, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_a_d() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x7a);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.d = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.a, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_a_e() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x7b);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.e = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.a, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_a_h() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x7c);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.h = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.a, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_a_l() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x7d);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.l = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.a, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_ld_a_a() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0x7f);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.a = 0x57;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.a, 0x57);
        assert_eq!(1 + cpu.t_cycles, 4);
    }

    #[test]
    fn test_push_de() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xd5);
        mock_bus.expect_write().with(eq(0x4fff), eq(0xba)).return_const(());
        mock_bus.expect_write().with(eq(0x4ffe), eq(0xad)).return_const(());

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.d = 0xba;
        cpu.e = 0xad;
        cpu.sp = 0x5000;
        cpu.clock(&mut mock_bus);

        assert_eq!(1 + cpu.t_cycles, 11);
    }

    #[test]
    fn test_ld_sp_hl() {
        let mut cpu = Z80CPU::new();
        let mut mock_bus = MockReadWrite::new();
        mock_bus.expect_read().with(eq(0)).returning(|_| 0xf9);

        cpu.reset();
        cpu.t_cycles = 0;
        cpu.h = 0x40;
        cpu.l = 0xff;
        cpu.clock(&mut mock_bus);

        assert_eq!(cpu.sp, 0x40ff);
        assert_eq!(1 + cpu.t_cycles, 6);
    }
}