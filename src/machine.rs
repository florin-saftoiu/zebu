use crate::cpu::Z80CPU;

pub struct ZebuZ80Bus<'a> {
    ram: &'a mut [u8; 64 * 1024]
}

impl<'a> ZebuZ80Bus<'a> {
    pub fn read(&self, addr: u16) -> u8 {
        self.ram[usize::from(addr)]
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.ram[usize::from(addr)] = data;
    }

    pub fn print_ram_state(&self, start: usize, len: usize) {
        print!("M:");
        let mut offset = 0;
        while offset < len {
            print!(" {:02X}", self.ram[start + offset]);
            offset += 1;
        }
        println!();
    }
}

pub struct Z80Machine<'a> {
    cpu: &'a mut Z80CPU,
    bus: ZebuZ80Bus<'a>
}

impl<'a> Z80Machine<'a> {
    pub fn new(cpu: &'a mut Z80CPU, ram: &'a mut [u8; 64 * 1024]) -> Z80Machine<'a> {
        Z80Machine {
            cpu: cpu,
            bus: ZebuZ80Bus {
                ram: ram
            }
        }
    }

    pub fn clock(&mut self) {
        self.cpu.clock(&mut self.bus);
    }

    pub fn print_state(&self) {
        self.cpu.print_state();
        self.bus.print_ram_state(0, 4);
    }
}