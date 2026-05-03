const BIOS_SIZE: u64 = 512 * 1024;

pub struct Bios {
    data: Vec<u8>,
}

impl Bios {
    pub fn new(path: &Path) -> Result<Bios, std::io::Error> {
        let mut file = std::fs::File::open(path)?;
        let mut data = Vec::new();

        file.take(BIOS_SIZE).read_to_end(&mut data)?;

        if data.len() == BIOS_SIZE as usize {
            Ok(Bios { data })
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "BIOS size is incorrect",
            ))
        }
    }

    pub fn load32(&self, offset: u32) -> u32 {
        let offset = offset as usize;

        let b0 = self.data[offset] as u32;
        let b1 = self.data[offset + 1] as u32;
        let b2 = self.data[offset + 2] as u32;
        let b3 = self.data[offset + 3] as u32;

        (b3 << 24) | (b2 << 16) | (b1 << 8) | b0
    }
}