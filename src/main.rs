use device_query::{DeviceQuery, DeviceState, Keycode};
use std::num::Wrapping;

mod cpu;
mod machine;
use cpu::Z80CPU;
use machine::Z80Machine;

fn main() {
    println!("Zebu");

    let mut cpu = Z80CPU::new();
    let mut ram = [0; 64 * 1024];
    let mut machine = Z80Machine::new(&mut cpu, &mut ram);
    
    let mut t_cycles = Wrapping(0usize);
    let device_state = DeviceState::new();
    let mut old_keys: Vec<Keycode> = Vec::new();
    loop {
        let new_keys: Vec<Keycode> = device_state.get_keys();
        let pressed_keys: Vec<_> = new_keys.iter().filter(|k| !old_keys.contains(k)).cloned().collect();
        old_keys = new_keys;
        if pressed_keys.contains(&Keycode::S) {
            machine.print_state();
            println!("T: {}", t_cycles);
        }
        if pressed_keys.contains(&Keycode::Space) {
            machine.clock();
            t_cycles += Wrapping(1);
            machine.print_state();
            println!("T: {}", t_cycles);
        }
    }
}