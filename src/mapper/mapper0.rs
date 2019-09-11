use crate::mapper::Mapper;
use crate::mapper::MirrorMode;

//
// NROM (mapper 0)
//
pub struct Mapper0 {
    chr_rom: Vec<u8>,
    prg_rom: Vec<u8>,
    sram: [u8; 0x2000],

    mirror_mode: MirrorMode,
}

impl Mapper for Mapper0 {
    fn nametable_offset(&self, table: usize) -> usize {
        self.mirror_mode.lookup()[table]
    }

    fn read(&mut self, address: u16) -> Result<u8, String> {
        match address {
            0x0000 ... 0x1fff => {
                let len = self.chr_rom.len();
                Ok(self.chr_rom[address as usize % len])
            },
            0x6000 ... 0x7fff => Ok(self.sram[address as usize - 0x6000]),
            0x8000 ... 0xffff => Ok(self.prg_rom[address as usize % self.prg_rom.len()]),
            _ => Ok(0),
        }
    }

    fn write(&mut self, address: u16, val: u8) -> Result<u8, String> {
        match address {
            0x0000 ... 0x1fff => {
                let len = self.chr_rom.len();
                self.chr_rom[address as usize % len] = val;
                Ok(val)
            },
            0x6000 ... 0x7fff => {
                self.sram[address as usize - 0x6000] = val;
                Ok(val)
            },
            _ => Ok(0),
        }
    }
}

impl Mapper0 {
    pub fn new_mapper(rom: Vec<u8>, vrom: Vec<u8>, mirror_mode: u8) -> Self {
        Self {
            chr_rom: vrom,
            prg_rom: rom,
            sram: [0; 0x2000],
            mirror_mode: MirrorMode::from(mirror_mode),
        }
    }
}