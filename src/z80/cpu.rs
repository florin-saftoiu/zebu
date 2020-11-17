use std::num::Wrapping;

use super::machine::ReadWrite;

const OPCODES: [(&str, fn(&mut Z80CPU, &mut dyn ReadWrite) -> u8, u8, u8); 256] = [
    ("NOP", Z80CPU::nop, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4),
    ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4),
    ("???", Z80CPU::invalid_opcode, 0, 4), ("LD HL", Z80CPU::ld_hl_nn, 2, 10), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4),
    ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("LD A", Z80CPU::ld_a_n, 1, 7), ("???", Z80CPU::invalid_opcode, 0, 4),
    ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4),
    ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4),
    ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4),
    ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("LD (HL), A", Z80CPU::ld_reg_hl_a, 0, 7), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4),
    ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4),
    ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4),
    ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4),
    ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4),
    ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4),
    ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4),
    ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4),
    ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4), ("???", Z80CPU::invalid_opcode, 0, 4)
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

    fn ld_hl_nn(&mut self, bus: &mut dyn ReadWrite) -> u8 {
        let n_low = bus.read(self.pc.0);
        self.pc += Wrapping(1);
        self.l = n_low;
        let n_high = bus.read(self.pc.0);
        self.pc += Wrapping(1);
        self.h = n_high;
        0
    }

    fn ld_a_n(&mut self, bus: &mut dyn ReadWrite) -> u8 {
        let n = bus.read(self.pc.0);
        self.pc += Wrapping(1);
        self.a = n;
        0
    }

    fn ld_reg_hl_a(&mut self, bus: &mut dyn ReadWrite) -> u8 {
        let hl = (u16::from(self.h) << 8) + u16::from(self.l);
        bus.write(hl, self.a);
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
}