pub struct Z80Bus<'a> {
    pub rom: &'a [u8; 16 * 1024],
    pub ram: &'a mut [u8; 48 * 1024]
}

impl<'a> Z80Bus<'a> {
    pub fn read(&self, addr: u16) -> u8 {
        if addr < 0x4000 {
            self.rom[usize::from(addr)]
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