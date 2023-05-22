#[derive(Debug, Clone)]
pub enum InstructionDecodeError {
    UnsupportedOpcode { raw_inst: u16, }
}

impl std::fmt::Display for InstructionDecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "invalid instruction")
    }
    
}
#[derive(Debug)]
pub enum Instruction {
    DisplayClear,
    SubReturn,
    Jump { target: u16 },
    SetVX { index: usize, value: u8 },
    AddVX { index: usize, value: u8 },
    SetIR { value: u16 },
    Draw { x: usize, y: usize, height: u16 },
    SubCall { target: u16 },
    SkipEq { vx : usize, nn: u8 },
    SkipNotEq { vx: usize, nn: u8 },
    SkipVEq { vx: usize, vy: usize },
    SkipVNotEq { vx: usize, vy: usize },

    MSetVReg { vx: usize, vy: usize },
    MSetVRegOr { vx: usize, vy: usize },
    MSetVRegAnd { vx: usize, vy: usize },
    MSetVRegXor { vx: usize, vy: usize },
    MAddWithCarry { vx: usize, vy: usize },
    MSubWithBorrow { vx: usize, vy: usize },
    MSubInvWithBorrow { vx: usize, vy: usize },
    MShiftRight { vx: usize, vy: usize },
    MShiftLeft { vx: usize, vy: usize },

    JumpOffset { vx: usize, offset: u16 },
    Random { vx: usize, nn: u8 },

    SkipIfKey { vx: usize },
    SkipIfNotKey { vx: usize },

    SetVXToDelayTimer { vx: usize },
    SetDelayTimerToVX { vx: usize },
    SetSoundTimerToVX { vx: usize },
    AddVXToIndexRegister { vx: usize },
    GetKeyBlock { vx: usize },
    FontChar { vx: usize },
    BinaryCodedDecimalConversion { vx: usize },
    SaveVXToMem { vx: usize },
    LoadVXFromMem { vx: usize },
}