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

pub fn decode(raw_inst: u16) -> Result<Instruction, InstructionDecodeError> {
    let opcode = raw_inst & 0xF000;

    let x = (raw_inst & 0x0F00) >> 8;
    let y = (raw_inst & 0x00F0) >> 4;
    
    let n: u8 = (raw_inst & 0x000F).try_into().expect("Instruction chunk N should fit into u8");
    let nn: u8 = (raw_inst & 0x00FF).try_into().expect("Instruction chunk NN should fit into u8");
    let nnn = raw_inst & 0x0FFF;

    match opcode {
        0x0000 => match raw_inst {
            0x00E0 => Ok(Instruction::DisplayClear),
            0x00EE => Ok(Instruction::SubReturn),
            _ => Err(InstructionDecodeError::UnsupportedOpcode { raw_inst: raw_inst })
        },
        0x1000 => Ok(Instruction::Jump { target: nnn }),
        0x6000 => Ok(Instruction::SetVX { index: x.into(), value: nn }),
        0x7000 => Ok(Instruction::AddVX { index: x.into(), value: nn }),
        0xA000 => Ok(Instruction::SetIR { value: nnn }),
        0xD000 => Ok(Instruction::Draw { x: x.into(), y: y.into(), height: raw_inst & 0x000F }),
        0x2000 => Ok(Instruction::SubCall { target: nnn }),
        0x3000 => Ok(Instruction::SkipEq { vx: x.into(), nn }),
        0x4000 => Ok(Instruction::SkipNotEq { vx: x.into(), nn }),
        0x5000 => Ok(Instruction::SkipVEq { vx: x.into(), vy: y.into() }),
        0x9000 => Ok(Instruction::SkipVNotEq { vx: x.into(), vy: y.into() }),
        0x8000 => match n {
            0x0 => Ok(Instruction::MSetVReg { vx: x.into(), vy: y.into() }),
            0x1 => Ok(Instruction::MSetVRegOr { vx: x.into(), vy: y.into() }),
            0x2 => Ok(Instruction::MSetVRegAnd { vx: x.into(), vy: y.into() }),
            0x3 => Ok(Instruction::MSetVRegXor { vx: x.into(), vy: y.into() }),
            0x4 => Ok(Instruction::MAddWithCarry { vx: x.into(), vy: y.into() }),
            0x5 => Ok(Instruction::MSubWithBorrow { vx: x.into(), vy: y.into() }),
            0x7 => Ok(Instruction::MSubInvWithBorrow { vx: x.into(), vy: y.into() }),
            0x6 => Ok(Instruction::MShiftRight { vx: x.into(), vy: y.into() }),
            0xE => Ok(Instruction::MShiftLeft { vx: x.into(), vy: y.into() }),
            _ => Err(InstructionDecodeError::UnsupportedOpcode { raw_inst: raw_inst })
        },
        0xB000 => Ok(Instruction::JumpOffset { vx: x.into(), offset: opcode & 0x0FFF }),
        0xC000 => Ok(Instruction::Random { vx: x.into(), nn }),
        0xE000 => match nn {
            0x9E => Ok(Instruction::SkipIfKey { vx: x.into() }),
            0xA1 => Ok(Instruction::SkipIfNotKey { vx: x.into() }),
            _ => Err(InstructionDecodeError::UnsupportedOpcode { raw_inst: raw_inst })
        },
        0xF000 => match nn {
            0x7 => Ok(Instruction::SetVXToDelayTimer { vx: x.into() }),
            0x15 => Ok(Instruction::SetDelayTimerToVX { vx: x.into() }),
            0x18 => Ok(Instruction::SetSoundTimerToVX { vx: x.into() }),
            0x1E => Ok(Instruction::AddVXToIndexRegister { vx: x.into() }),
            0x0A => Ok(Instruction::GetKeyBlock { vx: x.into() }),
            0x29 => Ok(Instruction::FontChar { vx: x.into() }),
            0x33 => Ok(Instruction::BinaryCodedDecimalConversion { vx: x.into() }),
            0x55 => Ok(Instruction::SaveVXToMem { vx: x.into() }),
            0x65 => Ok(Instruction::LoadVXFromMem { vx: x.into() }),
            _ => Err(InstructionDecodeError::UnsupportedOpcode { raw_inst: raw_inst })
        }
        _ => Err(InstructionDecodeError::UnsupportedOpcode { raw_inst: raw_inst })
    }
}