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

impl Cpu {
    fn decode_and_execute(&mut self, instruction: Instruction) {
        match instruction.function() {
            0b001111 => self.op_lui(instruction),
            _ => panic!("Unhandled instruction_{:x}", instruction.0),
        }
    }

    fn op_lui(&mut self, instruction: Instruction) {
        let i = instruction.imm();
        let t = instruction.t();

        panic!("what_now?");