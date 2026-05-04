use crate::map;

pub struct Interconnect {
    bios: Bios,
}

impl Interconnect {
    pub fn new(bios: Bios) -> Interconnect {
        Interconnect { bios }
    }

    pub fn load32(&self, addr: u32) -> u32 {
        if let Some(offset) = map::BIOS.contains(addr) {
            self.bios.load32(offset)
        } else {
            panic!("Unhandled load32 address: 0x{:08x}", addr)
        }
    }
}
