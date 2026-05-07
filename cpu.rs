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

        pub struct Cpu {
            pc: u32, 
            regs: [u32; 32]
            inter: Interconnect,
        }

        impl Cpu {
            pub fn new(inter: Interconnect) -> Cpu{
                let mut regs = [0xdeadbeef; 32];

                regs [0] = 0;
                regs: regs,
                inter: inter,
            }
        }

        fn reg(&self, index :u32) -> u32 {
            self.regs[index as usize]
        }

        fn set _reg(&mut self, index: u32, val: u32) {
            self.regs[index as usize] = va;;

            self.regs[0] = 0;
        }