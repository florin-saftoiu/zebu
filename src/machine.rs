use crate::cpu::{Z80CPU, Z80CPUState};

pub struct ZebuZ80Bus<'a> {
    rom: &'a [u8; 6],
    ram: &'a mut [u8; 48 * 1024]
}

impl<'a> ZebuZ80Bus<'a> {
    pub fn read(&self, addr: u16) -> u8 {
        self.rom[usize::from(addr % 6)]
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.ram[usize::from(addr)] = data;
    }
}

pub struct Z80Machine<'a> {
    cpu: &'a mut Z80CPU,
    bus: ZebuZ80Bus<'a>
}

impl<'a> Z80Machine<'a> {
    pub fn new(cpu: &'a mut Z80CPU, rom: &'a [u8; 6], ram: &'a mut [u8; 48 * 1024]) -> Z80Machine<'a> {
        Z80Machine {
            cpu: cpu,
            bus: ZebuZ80Bus {
                rom: rom,
                ram: ram
            }
        }
    }

    pub fn clock(&mut self) {
        self.cpu.clock(&mut self.bus);
    }

    pub fn get_cpu_state(&self) -> Z80CPUState {
        self.cpu.get_state()
    }

    pub fn get_ram_slice_state(&self, start: usize, len: usize) -> &[u8] {
        &self.bus.ram[start..len]
    }
}