#![cfg_attr(not(test), windows_subsystem = "windows")]

use std::io;
use std::io::prelude::*;
use std::fmt::Write;
use std::fs::File;

use piston_window::*;
use ::image::{ImageBuffer, Rgba};

mod z80;
use z80::{cpu::*, machine::*};

const WINDOW_PADDING: f64 = 8.0;
const WINDOW_FONTSIZE: f64 = 16.0;
const SCREEN_WIDTH: f64 = 256.0;
const SCREEN_HEIGHT: f64 = 192.0;
const SCREEN_SCALE: f64 = 2.0;

const BACKGROUND: [f32; 4] = [0.0, 0.478, 0.8, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const COLORS: [[u8; 4]; 8] = [
    [0, 0, 0, 255],      // BLACK
    [0, 0, 215, 255],    // BLUE
    [215, 0, 0, 255],    // RED
    [215, 0, 215, 255],  // MAGENTA
    [0, 215, 0, 255],    // GREEN
    [0, 215, 215, 255],  // CYAN
    [215, 215, 0, 255],  // YELLOW
    [215, 215, 215, 255] // WHITE
];
const BRIGHT_COLORS: [[u8; 4]; 8] = [
    [0, 0, 0, 255],      // BLACK
    [0, 0, 255, 255],    // BLUE
    [255, 0, 0, 255],    // RED
    [255, 0, 255, 255],  // MAGENTA
    [0, 255, 0, 255],    // GREEN
    [0, 255, 255, 255],  // CYAN
    [255, 255, 0, 255],  // YELLOW
    [255, 255, 255, 255] // WHITE
];

fn print_cpu_state(state: Z80CPUState) {
    println!("   AF: {:02X}{:02X} AF': {:02X}{:02X}", state.a, state.f, state.a_alt, state.f_alt);
    println!("   BC: {:02X}{:02X} BC': {:02X}{:02X}", state.b, state.c, state.b_alt, state.c_alt);
    println!("   DE: {:02X}{:02X} DE': {:02X}{:02X}", state.d, state.e, state.d_alt, state.e_alt);
    println!("   HL: {:02X}{:02X} HL': {:02X}{:02X}", state.h, state.l, state.h_alt, state.l_alt);
    println!("    I:   {:02X}   R:   {:02X}", state.i, state.r);
    println!("   IX: {:04X}", state.ix);
    println!("   IY: {:04X}", state.iy);
    println!("   SP: {:04X}", state.sp);
    println!("   PC: {:04X}", state.pc);
}

fn print_ram_slice_state(ram_slice: &[u8], offset: u16) {
    let mut nb_bytes = 0;
    for byte in ram_slice.iter() {
        if nb_bytes % 16 == 0 {
            if nb_bytes != 0 {
                println!();
            }
            print!(" {:04X}: ", offset + nb_bytes);
        }
        print!("{:02X} ", byte);
        nb_bytes += 1;
    }
    println!();
}

fn print_next_cpu_instructions(instructions: Vec<String>) {
    for i in 0..instructions.len() {
        if i == 0 {
            print!(">");
        } else {
            print!(" ")
        }
        println!("{}", instructions[i]);
    }
}

fn print_stack_state(ram_slice: Result<&[u8], &str>, offset: u16) {
    match ram_slice {
        Ok(ram_slice) => {
            for i in (0..ram_slice.len()).step_by(2).rev() {
                if i + 1 < ram_slice.len() {
                    if i == 0 {
                        print!(">");
                    } else {
                        print!(" ");
                    }
                    println!("{:04X}: {:04X}", offset + i as u16, (u16::from(ram_slice[i + 1]) << 8) + u16::from(ram_slice[i]));
                }
            }
        },
        Err(msg) => {
            println!("{}", msg)
        }
    }
    
}

fn draw_cpu_state(state: Z80CPUState, c: Context, g: &mut G2d, glyphs: &mut Glyphs) {
    let lines = format!(
        "{}{}.{}.{}{}{} IM{} {}I\n\
        \n\
        AF: {:02X}{:02X} AF': {:02X}{:02X}\n\
        BC: {:02X}{:02X} BC': {:02X}{:02X}\n\
        DE: {:02X}{:02X} DE': {:02X}{:02X}\n\
        HL: {:02X}{:02X} HL': {:02X}{:02X}\n\
        \x20I:   {:02X}   R:   {:02X}\n\
        IX: {:04X}\n\
        IY: {:04X}\n\
        SP: {:04X}\n\
        PC: {:04X}",
        if state.f & 0b10000000 != 0 { "S" } else { "." },
        if state.f & 0b01000000 != 0 { "Z" } else { "." },
        if state.f & 0b00010000 != 0 { "H" } else { "." },
        if state.f & 0b00000100 != 0 { "V" } else { "." },
        if state.f & 0b00000010 != 0 { "N" } else { "." },
        if state.f & 0b00000001 != 0 { "C" } else { "." },
        state.interrupt_mode, if state.interrupts_enabled { "E" } else { "D" },
        state.a, state.f, state.a_alt, state.f_alt,
        state.b, state.c, state.b_alt, state.c_alt,
        state.d, state.e, state.d_alt, state.e_alt,
        state.h, state.l, state.h_alt, state.l_alt,
        state.i, state.r,
        state.ix,
        state.iy,
        state.sp,
        state.pc
    );
    let mut y = WINDOW_PADDING + WINDOW_FONTSIZE;
    for line in lines.split("\n") {
        text::Text::new_color(WHITE, WINDOW_FONTSIZE as u32).draw(
            &line,
            glyphs,
            &c.draw_state,
            c.transform.trans(WINDOW_PADDING + SCREEN_WIDTH * SCREEN_SCALE + WINDOW_PADDING, y),
            g
        ).unwrap();
        y += WINDOW_FONTSIZE;
    }
}

fn draw_ram_slice_state(ram_slice: &[u8], offset: u16, c: Context, g: &mut G2d, glyphs: &mut Glyphs) {
    let mut ram_string = String::new();
    let mut nb_bytes = 0;
    for byte in ram_slice.iter() {
        if nb_bytes % 16 == 0 {
            if nb_bytes != 0 {
                writeln!(ram_string).unwrap();
            }
            write!(ram_string, "{:04X}: ", offset + nb_bytes).unwrap();
        }
        write!(ram_string, "{:02X} ", byte).unwrap();
        nb_bytes += 1;
    }
    let mut y = WINDOW_PADDING + SCREEN_HEIGHT * SCREEN_SCALE + WINDOW_PADDING + WINDOW_FONTSIZE;
    for line in ram_string.split("\n") {
        text::Text::new_color(WHITE, WINDOW_FONTSIZE as u32).draw(
            &line,
            glyphs,
            &c.draw_state,
            c.transform.trans(WINDOW_PADDING, y),
            g
        ).unwrap();
        y += WINDOW_FONTSIZE;
    }
}

fn draw_next_cpu_instructions(instructions: Vec<String>, c: Context, g: &mut G2d, glyphs: &mut Glyphs, pointer_offset: usize) {
    let mut y = WINDOW_PADDING + WINDOW_FONTSIZE;
    for i in 0..instructions.len() {
        text::Text::new_color(if i == pointer_offset { RED } else { match i { 0 => YELLOW, _ => WHITE } }, WINDOW_FONTSIZE as u32).draw(
            &instructions[i],
            glyphs,
            &c.draw_state,
            c.transform.trans(WINDOW_PADDING + SCREEN_WIDTH * SCREEN_SCALE + WINDOW_PADDING + 240.0, y),
            g
        ).unwrap();
        y += WINDOW_FONTSIZE;
    }
}

fn draw_stack_state(ram_slice: Result<&[u8], &str>, offset: u16, c: Context, g: &mut G2d, glyphs: &mut Glyphs) {
    let mut y = WINDOW_PADDING + SCREEN_HEIGHT * SCREEN_SCALE + WINDOW_PADDING + WINDOW_FONTSIZE;
    match ram_slice {
        Ok(ram_slice) => {
            for i in (0..ram_slice.len()).step_by(2).rev() {
                if i + 1 < ram_slice.len() {
                    let word = format!("{:04X}: {:04X}", offset + i as u16, (u16::from(ram_slice[i + 1]) << 8) + u16::from(ram_slice[i]));
                    text::Text::new_color(if i == 0 { YELLOW } else { WHITE }, WINDOW_FONTSIZE as u32).draw(
                        &word,
                        glyphs,
                        &c.draw_state,
                        c.transform.trans(WINDOW_PADDING + SCREEN_WIDTH * SCREEN_SCALE + WINDOW_PADDING + 240.0, y),
                        g
                    ).unwrap();
                    y += WINDOW_FONTSIZE;
                }
            }
        },
        Err(msg) => {
            text::Text::new_color(WHITE, WINDOW_FONTSIZE as u32).draw(
                msg,
                glyphs,
                &c.draw_state,
                c.transform.trans(WINDOW_PADDING + SCREEN_WIDTH * SCREEN_SCALE + WINDOW_PADDING + 240.0, y),
                g
            ).unwrap();
        }
    }
    
}

fn main() -> io::Result<()> {
    println!("Zebu");
    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    
    let mut cpu = Z80CPU::new();
    let mut rom_file = File::open(assets.join("zxs48.bin"))?;
    let mut rom = [0; 16 * 1024];
    rom_file.read(&mut rom)?;
    let mut ram = [0; 48 * 1024];
    let mut machine = Z80Machine::new(&mut cpu, &rom, &mut ram);
    machine.reset();
    
    let mut window: PistonWindow = WindowSettings::new("Zebu", [1024, 768])
        .resizable(false)
        .exit_on_esc(true)
        .automatic_close(true)
        .build()
        .unwrap();
    let mut glyphs = window.load_font(assets.join("3270Medium.ttf")).unwrap();
    
    let mut texture_context = window.create_texture_context();
    let mut events = Events::new(EventSettings::new());

    let mut paused = true;
    let mut pointer_offset = 0usize;
    while let Some(e) = events.next(&mut window) {
        if paused {
            if let Some(Button::Keyboard(key)) = e.press_args() {
                if key == Key::S {
                    println!("    T: {}", machine.get_t_cycles());
                    print_cpu_state(machine.get_cpu_state());
                    print_ram_slice_state(machine.get_ram_slice_state(0, 32), 0x4000);
                    print_stack_state(machine.get_stack_slice_state(0, 8), machine.get_cpu_state().sp);
                    print_next_cpu_instructions(machine.get_next_cpu_instructions(3));
                } else if key == Key::F10 {
                    loop {
                        machine.clock();
                        if machine.cpu_instruction_complete() {
                            break;
                        }
                    }
                    println!("    T: {}", machine.get_t_cycles());
                    print_cpu_state(machine.get_cpu_state());
                    print_ram_slice_state(machine.get_ram_slice_state(0, 32), 0x4000);
                    print_stack_state(machine.get_stack_slice_state(0, 8), machine.get_cpu_state().sp);
                    print_next_cpu_instructions(machine.get_next_cpu_instructions(3));
                } else if key == Key::B {
                    match u16::from_str_radix(&(machine.get_next_cpu_instructions(pointer_offset + 1)[pointer_offset])[0..4], 16) {
                        Ok(breakpoint) => {
                            loop {
                                machine.clock();
                                if machine.get_cpu_state().pc >= breakpoint && machine.cpu_instruction_complete() {
                                    break;
                                }
                            }
                            pointer_offset = 0;
                            println!("    T: {}", machine.get_t_cycles());
                            print_cpu_state(machine.get_cpu_state());
                            print_ram_slice_state(machine.get_ram_slice_state(0, 32), 0x4000);
                            print_stack_state(machine.get_stack_slice_state(0, 8), machine.get_cpu_state().sp);
                            print_next_cpu_instructions(machine.get_next_cpu_instructions(3));
                        },
                        Err(e) => panic!("{}", e)
                    }
                } else if key == Key::Up {
                    if pointer_offset > 0 {
                        pointer_offset -= 1;
                    }
                } else if key == Key::Down {
                    if pointer_offset < 23 {
                        pointer_offset += 1;
                    }
                } else if key == Key::F5 {
                    paused = false;
                }
            }
        } else {
            if let Some(Button::Keyboard(key)) = e.press_args() {
                if key == Key::F6 {
                    paused = true;
                    // finish current instruction before pausing
                    if !machine.cpu_instruction_complete() {
                        loop {
                            machine.clock();
                            if machine.cpu_instruction_complete() {
                                break;
                            }
                        }
                    }
                }
            }
            // user may have just paused and finished the current instruction
            if !paused {
                machine.clock();
            }
        }
        
        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::R {
                machine.reset();
            }
        }
        
        if let Some(_) = e.render_args() {
            window.draw_2d(&e, |c, g, device| {
                clear(BACKGROUND, g);
                let image = Image::new().rect([WINDOW_PADDING, WINDOW_PADDING, SCREEN_WIDTH * SCREEN_SCALE, SCREEN_HEIGHT * SCREEN_SCALE]);
                let pixels_ram_slice = machine.get_ram_slice_state(0, 6144);
                let attributes_ram_slice = machine.get_ram_slice_state(0x1800, 768);
                let buffer = ImageBuffer::from_fn((SCREEN_WIDTH * SCREEN_SCALE) as u32, (SCREEN_HEIGHT * SCREEN_SCALE) as u32, |x, y| {
                    let pixel_x = x / SCREEN_SCALE as u32;
                    let pixel_y = y / SCREEN_SCALE as u32;
                    let pixel_byte_offset = pixel_x / 8;
                    let pixel_byte = pixels_ram_slice[(pixel_y * 32 + pixel_byte_offset) as usize];
                    let pixel_bit_offset = pixel_x % 8;
                    let pixel_bit_mask = 0b10000000u8 >> pixel_bit_offset;
                    let pixel_bit = pixel_byte & pixel_bit_mask != 0;

                    let attribute_byte = attributes_ram_slice[((pixel_y / 8) * 32 + (pixel_x / 8)) as usize];
                    let _ = attribute_byte & 0b10000000 != 0; // TODO FLASH ATTRIBUTE
                    let bright = attribute_byte & 0b01000000 != 0;
                    let paper_index = (attribute_byte & 0b00111000) as usize;
                    let ink_index = (attribute_byte & 0b00000111) as usize;

                    if pixel_bit {
                        Rgba(if bright { BRIGHT_COLORS[ink_index] } else { COLORS[ink_index] })
                    } else {
                        Rgba(if bright { BRIGHT_COLORS[paper_index] } else { COLORS[paper_index] })
                    }
                });
                let texture = Texture::from_image(&mut texture_context, &buffer, &TextureSettings::new()).unwrap();
                image.draw(&texture, &c.draw_state, c.transform, g);
                draw_cpu_state(machine.get_cpu_state(), c, g, &mut glyphs);
                draw_ram_slice_state(machine.get_ram_slice_state(0, 256), 0x4000, c, g, &mut glyphs);
                draw_next_cpu_instructions(machine.get_next_cpu_instructions(24), c, g, &mut glyphs, pointer_offset);
                draw_stack_state(machine.get_stack_slice_state(0, 16), machine.get_cpu_state().sp, c, g, &mut glyphs);
                
                glyphs.factory.encoder.flush(device);
            });
            window.set_title(format!("Zebu - T: {}", machine.get_t_cycles()));
        }
    }
    
    Ok(())
}