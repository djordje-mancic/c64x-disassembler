use std::{
    fmt::Display,
    ops::{AddAssign, Not},
};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Register {
    A(u8),
    APair(u8, u8),
    B(u8),
    BPair(u8, u8),
}

impl Register {
    pub fn from(value: u8, side: bool) -> Self {
        if side == false {
            Self::A(value)
        } else {
            Self::B(value)
        }
    }

    pub fn from_pair(value: u8, side: bool) -> Self {
        let value2 = value - value % 2;
        let value1 = value2 + 1;
        if side == false {
            Self::APair(value1, value2)
        } else {
            Self::BPair(value1, value2)
        }
    }

    pub fn side(&self) -> bool {
        match self {
            Self::A(_) => false,
            Self::APair(_, _) => false,
            Self::B(_) => true,
            Self::BPair(_, _) => true,
        }
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A(num) => write!(f, "A{num}"),
            Self::APair(num1, num2) => write!(f, "A{num1}:A{num2}"),
            Self::B(num) => write!(f, "B{num}"),
            Self::BPair(num1, num2) => write!(f, "B{num1}:B{num2}"),
        }
    }
}

impl AddAssign<u8> for Register {
    fn add_assign(&mut self, rhs: u8) {
        match self {
            Self::A(num) | Self::B(num) => *num += rhs,
            Self::APair(num1, num2) | Self::BPair(num1, num2) => {
                *num1 += rhs - rhs % 2;
                *num2 = *num1 - 1;
            }
        }
    }
}

impl Not for Register {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::A(num) => Self::B(num),
            Self::APair(num1, num2) => Self::BPair(num1, num2),
            Self::B(num) => Self::A(num),
            Self::BPair(num1, num2) => Self::APair(num1, num2),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ControlRegister {
    /// Addressing mode register.
    AMR,
    /// Control status register.
    CSR,
    /// Galois field multiply control register.
    GFPGFR,
    /// Interrupt clear register.
    ICR,
    /// Interrupt enable register.
    IER,
    /// Interrupt flag register.
    IFR,
    /// Interrupt return pointer register.
    IRP,
    /// Interrupt set register.
    ISR,
    /// Interrupt service table pointer register.
    ISTP,
    /// Nonmaskable interrupt return pointer register.
    NRP,
    /// Program counter, E1 phase.
    PCE1,

    // Control Register File Extensions (C64x+ DSP)
    /// Debug interrupt enable register.
    DIER,
    /// DSP core number register.
    DNUM,
    /// Exception clear register.
    ECR,
    /// Exception flag register.
    EFR,
    /// GMPY A-side polynomial register.
    GPLYA,
    /// GMPY B-side polynomial register.
    GPLYB,
    /// Internal exception report register.
    IERR,
    /// Inner loop count register.
    ILC,
    /// Interrupt task state register.
    ITSR,
    /// NMI/Exception task state register.
    NTSR,
    /// Restricted entry point address register.
    REP,
    /// Reload inner loop count register.
    RILC,
    /// Saturation status register.
    SSR,
    /// Time-stamp counter (high 32) register.
    TSCH,
    /// Time-stamp counter (low 32) register.
    TSCL,
    /// Task state register.
    TSR,
}

impl ControlRegister {
    pub fn from(low: u8, high: u8) -> Option<Self> {
        match low {
            0b00000 => Some(Self::AMR),
            0b00001 => Some(Self::CSR),
            0b11001 => Some(Self::DIER),
            0b10001 => Some(Self::DNUM),
            0b11101 => Some(Self::ECR),
            0b11000 => Some(Self::GFPGFR),
            0b10110 => Some(Self::GPLYA),
            0b10111 => Some(Self::GPLYB),
            0b00011 => Some(Self::ICR),
            0b00100 => Some(Self::IER),
            0b11111 => Some(Self::IERR),
            0b00010 if high == 0b00000 || high == 0b00010 => Some(Self::IFR),
            0b00010 => Some(Self::ISR),
            0b01101 => Some(Self::ILC),
            0b00110 => Some(Self::IRP),
            0b00101 => Some(Self::ISTP),
            0b11011 => Some(Self::ITSR),
            0b00111 => Some(Self::NRP),
            0b11100 => Some(Self::NTSR),
            0b10000 => Some(Self::PCE1),
            0b01111 => Some(Self::REP),
            0b01110 => Some(Self::RILC),
            0b10101 => Some(Self::SSR),
            0b01011 => Some(Self::TSCH),
            0b01010 => Some(Self::TSCL),
            0b11010 => Some(Self::TSR),
            _ => None,
        }
    }
}

impl Display for ControlRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AMR => write!(f, "AMR"),
            Self::CSR => write!(f, "CSR"),
            Self::GFPGFR => write!(f, "GFPGFR"),
            Self::ICR => write!(f, "ICR"),
            Self::IER => write!(f, "IER"),
            Self::IFR => write!(f, "IFR"),
            Self::IRP => write!(f, "IRP"),
            Self::ISR => write!(f, "ISR"),
            Self::ISTP => write!(f, "ISTP"),
            Self::NRP => write!(f, "NRP"),
            Self::PCE1 => write!(f, "PCE1"),
            Self::DIER => write!(f, "DIER"),
            Self::DNUM => write!(f, "DNUM"),
            Self::ECR => write!(f, "ECR"),
            Self::EFR => write!(f, "EFR"),
            Self::GPLYA => write!(f, "GPLYA"),
            Self::GPLYB => write!(f, "GPLYB"),
            Self::IERR => write!(f, "IERR"),
            Self::ILC => write!(f, "ILC"),
            Self::ITSR => write!(f, "ITSR"),
            Self::NTSR => write!(f, "NTSR"),
            Self::REP => write!(f, "REP"),
            Self::RILC => write!(f, "RILC"),
            Self::SSR => write!(f, "SSR"),
            Self::TSCH => write!(f, "TSCH"),
            Self::TSCL => write!(f, "TSCL"),
            Self::TSR => write!(f, "TSR"),
        }
    }
}

pub enum RegisterFile {
    GeneralPurpose(Register),
    Control(ControlRegister),
}

impl RegisterFile {
    pub fn side(&self) -> Option<bool> {
        if let Self::GeneralPurpose(register) = self {
            Some(register.side())
        } else {
            None
        }
    }
}

impl Display for RegisterFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GeneralPurpose(register) => write!(f, "{register}"),
            Self::Control(register) => write!(f, "{register}"),
        }
    }
}
