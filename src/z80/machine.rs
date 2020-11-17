use std::cmp;
use std::num::Wrapping;

use super::cpu::{Z80CPU, Z80CPUState};
use super::bus::Z80Bus;

pub struct Z80Machine<'a> {
    cpu: &'a mut Z80CPU,
    bus: Z80Bus<'a>,
    t_cycles: Wrapping<usize>
}

impl<'a> Z80Machine<'a> {
    pub fn new(cpu: &'a mut Z80CPU, rom: &'a [u8; 16 * 1024], ram: &'a mut [u8; 48 * 1024]) -> Z80Machine<'a> {
        Z80Machine {
            cpu: cpu,
            bus: Z80Bus {
                rom: rom,
                ram: ram
            },
            t_cycles: Wrapping(0usize)
        }
    }

    pub fn clock(&mut self) {
        self.cpu.clock(&mut self.bus);
        self.t_cycles += Wrapping(1);
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        self.t_cycles = Wrapping(0);
        loop {
            self.clock();
            if self.cpu.instruction_complete() {
                break;
            }
        }
    }

    pub fn cpu_instruction_complete(&self) -> bool {
        self.cpu.instruction_complete()
    }

    pub fn get_t_cycles(&self) -> usize {
        self.t_cycles.0
    }

    pub fn get_next_cpu_instructions(&self, nb: usize) -> Vec<String> {
        self.cpu.get_next_instructions(&self.bus, nb)
    }

    pub fn get_cpu_state(&self) -> Z80CPUState {
        self.cpu.get_state()
    }

    pub fn get_ram_slice_state(&self, start: usize, len: usize) -> &[u8] {
        &self.bus.ram[start..cmp::min(start + len, 0xFFFF - 0x4000 + 1)]
    }

    pub fn get_stack_slice_state(&self, start: usize, len: usize) -> Result<&[u8], &str> {
        let sp = usize::from(self.cpu.get_state().sp);
        if sp < 0x4000 {
            return Err("INVALID SP");
        }
        Ok(&self.bus.ram[sp - 0x4000 + start..cmp::min(sp - 0x4000 + start + len, 0xFFFF - 0x4000 + 1)])
    }
}