use crate::cpu::{Z80CPU, Z80CPUState};

pub struct Z80Bus<'a> {
    rom: &'a [u8; 16 * 1024],
    ram: &'a mut [u8; 48 * 1024]
}

impl<'a> Z80Bus<'a> {
    pub fn read(&self, addr: u16) -> u8 {
        if addr < 0x4000 {
            self.rom[usize::from(addr % 6)]
        } else {
            self.ram[usize::from(addr)]
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        if addr < 0x4000 {
            // do nothing, can't write to rom
        } else {
            self.ram[usize::from(addr - 0x4000)] = data;
        }
    }
}

pub struct Z80Machine<'a> {
    cpu: &'a mut Z80CPU,
    bus: Z80Bus<'a>
}

impl<'a> Z80Machine<'a> {
    pub fn new(cpu: &'a mut Z80CPU, rom: &'a [u8; 16 * 1024], ram: &'a mut [u8; 48 * 1024]) -> Z80Machine<'a> {
        Z80Machine {
            cpu: cpu,
            bus: Z80Bus {
                rom: rom,
                ram: ram
            }
        }
    }

    pub fn clock(&mut self) {
        self.cpu.clock(&mut self.bus);
    }

    pub fn cpu_instruction_complete(&self) -> bool {
        self.cpu.instruction_complete()
    }

    pub fn get_next_cpu_instructions(&self, nb: usize) -> Vec<String> {
        self.cpu.get_next_instructions(&self.bus, nb)
    }

    pub fn get_cpu_state(&self) -> Z80CPUState {
        self.cpu.get_state()
    }

    pub fn get_ram_slice_state(&self, start: usize, len: usize) -> &[u8] {
        &self.bus.ram[start..len]
    }
}