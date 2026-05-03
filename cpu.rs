pub struct Cpu {
    pc: u32,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            pc: 0xbfc00000,
        }
    }

    pub fn run_next_instruction(&mut self) {
        let pc = self.pc;

        let instruction = self.load32(pc);

        self.pc = pc.wrapping_add(4);

        self.decode_and_execute(instruction);
    }
}