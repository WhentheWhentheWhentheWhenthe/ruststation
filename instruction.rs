struct Instruction(u32);

impl Instruction {
    fn function(self) -> u32 {
        let Instruction(op) = self;

        (op >> 16) & 0x1f
    }
    
    fn imm(self) -> u32 {
        let Instruction(op) = self;

        op & 0xffff
    }
}