use serde::{Deserialize, Serialize};
use std::fmt;

/// Part-of-speech tags based on the Penn Treebank tagset with custom modifications.
/// For details on added tags (e.g., CA, CS, NZ), modified tags (e.g., EX, CD), and removed punctuation tags (e.g., SS, PUNC, SYM), refer to the crate documentation.
#[derive(Default, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum POSTag {
    CC,
    CS,
    CA,
    DT,
    EX,
    #[default]
    FW,
    IN,
    JJ,
    JJR,
    JJS,
    LS,
    MD,
    MWE,
    NN,
    NNS,
    NNP,
    NNPS,
    NM,
    NZ,
    PDT,
    PR,
    PRP,
    PUNC,
    RB,
    RBR,
    RBS,
    SS,
    SYM,
    SYS,
    UH,
    VB,
    VBD,
    VBG,
    VBN,
    VBP,
    VBZ,
    VF,
    VFG,
    VH,
    VHF,
    VHP,
    VHZ,
    WDT,
    WPR,
    WPRP,
    WRB,
}

impl POSTag {
    /// Convert a string into an instance of the POSTag enum
    pub fn from_str(tag: &str) -> Self {
        match tag.to_uppercase().as_str() {
            "CC" => Self::CC,
            "CS" => Self::CS,
            "CA" => Self::CA,
            "DT" => Self::DT,
            "EX" => Self::EX,
            "FW" => Self::FW,
            "IN" => Self::IN,
            "JJ" => Self::JJ,
            "JJR" => Self::JJR,
            "JJS" => Self::JJS,
            "LS" => Self::LS,
            "MD" => Self::MD,
            "MWE" => Self::MWE,
            "NN" => Self::NN,
            "NNS" => Self::NNS,
            "NNP" => Self::NNP,
            "NNPS" => Self::NNPS,
            "NM" => Self::NM,
            "NZ" => Self::NZ,
            "PDT" => Self::PDT,
            "PR" => Self::PR,
            "PRP" => Self::PRP,
            "PUNC" => Self::PUNC,
            "RB" => Self::RB,
            "RBR" => Self::RBR,
            "RBS" => Self::RBS,
            "SS" => Self::SS,
            "SYM" => Self::SYM,
            "SYS" => Self::SYS,
            "UH" => Self::UH,
            "VB" => Self::VB,
            "VBD" => Self::VBD,
            "VBG" => Self::VBG,
            "VBN" => Self::VBN,
            "VBP" => Self::VBP,
            "VBZ" => Self::VBZ,
            "VF" => Self::VF,
            "VFG" => Self::VFG,
            "VH" => Self::VH,
            "VHF" => Self::VHF,
            "VHP" => Self::VHP,
            "VHZ" => Self::VHZ,
            "WDT" => Self::WDT,
            "WPR" => Self::WPR,
            "WPRP" => Self::WPRP,
            "WRB" => Self::WRB,
            _ => Self::FW,
        }
    }

    /// Convert an instance of POStag into its string counterpart
    pub fn to_str(&self) -> String {
        match self {
            Self::CC => "CC".to_string(),
            Self::CS => "CS".to_string(),
            Self::CA => "CA".to_string(),
            Self::DT => "DT".to_string(),
            Self::EX => "EX".to_string(),
            Self::FW => "FW".to_string(),
            Self::IN => "IN".to_string(),
            Self::JJ => "JJ".to_string(),
            Self::JJR => "JJR".to_string(),
            Self::JJS => "JJS".to_string(),
            Self::LS => "LS".to_string(),
            Self::MD => "MD".to_string(),
            Self::MWE => "MWE".to_string(),
            Self::NN => "NN".to_string(),
            Self::NNS => "NNS".to_string(),
            Self::NNP => "NNP".to_string(),
            Self::NNPS => "NNPS".to_string(),
            Self::NM => "NM".to_string(),
            Self::NZ => "NZ".to_string(),
            Self::PDT => "PDT".to_string(),
            Self::PR => "PR".to_string(),
            Self::PRP => "PRP".to_string(),
            Self::PUNC => "PUNC".to_string(),
            Self::RB => "RB".to_string(),
            Self::RBR => "RBR".to_string(),
            Self::RBS => "RBS".to_string(),
            Self::SS => "SS".to_string(),
            Self::SYM => "SYM".to_string(),
            Self::SYS => "SYS".to_string(),
            Self::UH => "UH".to_string(),
            Self::VB => "VB".to_string(),
            Self::VBD => "VBD".to_string(),
            Self::VBG => "VBG".to_string(),
            Self::VBN => "VBN".to_string(),
            Self::VBP => "VBP".to_string(),
            Self::VBZ => "VBZ".to_string(),
            Self::VF => "VF".to_string(),
            Self::VFG => "VFG".to_string(),
            Self::VH => "VH".to_string(),
            Self::VHF => "VHF".to_string(),
            Self::VHP => "VHP".to_string(),
            Self::VHZ => "VHZ".to_string(),
            Self::WDT => "WDT".to_string(),
            Self::WPR => "WPR".to_string(),
            Self::WPRP => "WPRP".to_string(),
            Self::WRB => "WRB".to_string(),
        }
    }

    /// Convert a u8 into an instance of POSTag enum, mainly used by the POS tagger
    pub fn from_u8(value: u8) -> Self {
        match value {
            1 => Self::CC,
            2 => Self::CS,
            3 => Self::CA,
            4 => Self::DT,
            5 => Self::EX,
            6 => Self::FW,
            7 => Self::IN,
            8 => Self::JJ,
            9 => Self::JJR,
            10 => Self::JJS,
            11 => Self::LS,
            12 => Self::MD,
            13 => Self::MWE,
            14 => Self::NN,
            15 => Self::NNS,
            16 => Self::NNP,
            17 => Self::NNPS,
            18 => Self::NM,
            19 => Self::NZ,
            20 => Self::PDT,
            21 => Self::PR,
            22 => Self::PRP,
            23 => Self::PUNC,
            24 => Self::RB,
            25 => Self::RBR,
            26 => Self::RBS,
            27 => Self::SS,
            28 => Self::SYM,
            29 => Self::SYS,
            30 => Self::UH,
            31 => Self::VB,
            32 => Self::VBD,
            33 => Self::VBG,
            34 => Self::VBN,
            35 => Self::VBP,
            36 => Self::VBZ,
            37 => Self::VF,
            38 => Self::VFG,
            39 => Self::VH,
            40 => Self::VHF,
            41 => Self::VHP,
            42 => Self::VHZ,
            43 => Self::WDT,
            44 => Self::WPR,
            45 => Self::WPRP,
            46 => Self::WRB,
            _ => Self::FW,
        }
    }

    /// Converts an instance of the POSTag enum into its u8 counterpart, generally only used by the POS tagger
    pub fn to_u8(&self) -> u8 {
        match self {
            Self::CC => 1,
            Self::CS => 2,
            Self::CA => 3,
            Self::DT => 4,
            Self::EX => 5,
            Self::FW => 6,
            Self::IN => 7,
            Self::JJ => 8,
            Self::JJR => 9,
            Self::JJS => 10,
            Self::LS => 11,
            Self::MD => 12,
            Self::MWE => 13,
            Self::NN => 14,
            Self::NNS => 15,
            Self::NNP => 16,
            Self::NNPS => 17,
            Self::NM => 18,
            Self::NZ => 19,
            Self::PDT => 20,
            Self::PR => 21,
            Self::PRP => 22,
            Self::PUNC => 23,
            Self::RB => 24,
            Self::RBR => 25,
            Self::RBS => 26,
            Self::SS => 27,
            Self::SYM => 28,
            Self::SYS => 29,
            Self::UH => 30,
            Self::VB => 31,
            Self::VBD => 32,
            Self::VBG => 33,
            Self::VBN => 34,
            Self::VBP => 35,
            Self::VBZ => 36,
            Self::VF => 37,
            Self::VFG => 38,
            Self::VH => 39,
            Self::VHF => 40,
            Self::VHP => 41,
            Self::VHZ => 42,
            Self::WDT => 43,
            Self::WPR => 44,
            Self::WPRP => 45,
            Self::WRB => 46,
        }
    }

    /// Converts an instance of the POSTag enum into a u8 used for training the POS tagger
    pub fn to_train_u8(&self) -> u8 {
        let chk_tag = match self {
            Self::PR | Self::PRP => Self::PR,
            Self::JJ | Self::JJR | Self::JJS => Self::JJ,
            Self::RB | Self::RBR | Self::RBS => Self::RB,
            _ => *self,
        };
        chk_tag.to_u8()
    }

    /// Check whether the POS tag belongs to a noun
    pub fn is_noun(&self) -> bool {
        self.to_str().starts_with("N") || *self == Self::SYS
    }

    /// Check whether the POS tag belongs to a verb
    pub fn is_verb(&self) -> bool {
        self.to_str().starts_with("V")
    }

    /// Check whether the POS tag belongs to a adjective
    pub fn is_adjective(&self) -> bool {
        self.to_str().starts_with("J")
    }

    /// Check whether the POS tag belongs to a adverb
    pub fn is_adverb(&self) -> bool {
        self.to_str().starts_with("R")
    }

    /// Check whether the POS tag belongs to a named entiy
    pub fn is_named_entity(&self) -> bool {
        self.to_str().starts_with("NNP")
    }
}

impl fmt::Display for POSTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl fmt::Debug for POSTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}
