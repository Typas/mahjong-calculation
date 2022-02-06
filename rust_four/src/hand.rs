pub const HANDVARIANT: usize = 31;
pub const HANDMAXSCORE: u16 = 16;

#[derive(Clone, Copy)]
pub enum Hand {
    AllChows,              // 平和
    RedPung,               // 役牌中
    GreenPung,             // 役牌發
    WhitePung,             // 役牌白
    WindPung,              // 役牌自風(東)
    PureStraight,          // 一氣通貫
    AllPungs,              // 對對和
    LittleThreeDragons,    // 小三元
    BigThreeDragons,       // 大三元
    LittleFourWinds,       // 小四喜
    AllSimples,            // 斷幺九
    OutsideHands,          // 混全帶幺九
    TerminalsInAllSets,    // 清全帶幺九
    AllTerminalsAndHonors, // 混老頭
    HalfFlush,             // 混一色
    FullFlush,             // 清一色
    AllHonors,             // 字一色
    TwoConcealedPungs,     // 二暗刻
    ThreeConcealedPungs,   // 三暗刻
    FourConcealedPungs,    // 四暗刻
    PureDoubleChow,        // 一般高
    TwicePureDoubleChow,   // 二般高
    PureTripleChow,        // 三同順
    MixedTripleChow,       // 三色同順
    TriplePung,            // 三色同刻
    PureShiftedPungs,      // 三連刻
    FourPureShiftedPungs,  // 四連刻
    BigFourWinds,          // 大四喜
    AllTerminals,          // 清老頭
    QuadrupleChow,         // 四同順
    NoPoint,               // 無役
}

impl Hand {
    pub fn score(&self) -> u16 {
        match self {
            &Self::AllChows => 1,
            &Self::RedPung => 2,
            &Self::GreenPung => 2,
            &Self::WhitePung => 2,
            &Self::WindPung => 2,
            &Self::PureStraight => 4,
            &Self::AllPungs => 4,
            &Self::LittleThreeDragons => 6,
            &Self::BigThreeDragons => 12,
            &Self::LittleFourWinds => 12,
            &Self::AllSimples => 1,
            &Self::OutsideHands => 4,
            &Self::TerminalsInAllSets => 6,
            &Self::AllTerminalsAndHonors => 12,
            &Self::HalfFlush => 6,
            &Self::FullFlush => 8,
            &Self::AllHonors => 16,
            &Self::TwoConcealedPungs => 1,
            &Self::ThreeConcealedPungs => 3,
            &Self::FourConcealedPungs => 6,
            &Self::PureDoubleChow => 1,
            &Self::TwicePureDoubleChow => 6,
            &Self::PureTripleChow => 12,
            &Self::MixedTripleChow => 3,
            &Self::TriplePung => 12,
            &Self::PureShiftedPungs => 8,
            &Self::FourPureShiftedPungs => 16,
            // 絕對滿貫
            &Self::BigFourWinds => HANDMAXSCORE,
            &Self::AllTerminals => HANDMAXSCORE,
            &Self::QuadrupleChow => HANDMAXSCORE,
            // 無役
            &Self::NoPoint => 0,
        }
    }

    pub fn name(&self) -> String {
        match self {
            &Self::AllChows => "平和".to_string(),
            &Self::RedPung | &Self::GreenPung | &Self::WhitePung | &Self::WindPung => {
                "役牌".to_string()
            }
            &Self::PureStraight => "一氣".to_string(),
            &Self::AllPungs => "對對和".to_string(),
            &Self::LittleThreeDragons => "小三元".to_string(),
            &Self::BigThreeDragons => "大三元".to_string(),
            &Self::LittleFourWinds => "小四喜".to_string(),
            &Self::AllSimples => "斷幺九".to_string(),
            &Self::OutsideHands => "混全帶".to_string(),
            &Self::TerminalsInAllSets => "清全帶".to_string(),
            &Self::AllTerminalsAndHonors => "混老頭".to_string(),
            &Self::HalfFlush => "混一色".to_string(),
            &Self::FullFlush => "清一色".to_string(),
            &Self::AllHonors => "字一色".to_string(),
            &Self::TwoConcealedPungs => "二暗刻".to_string(),
            &Self::ThreeConcealedPungs => "三暗刻".to_string(),
            &Self::FourConcealedPungs => "四暗刻".to_string(),
            &Self::PureDoubleChow => "一般高".to_string(),
            &Self::TwicePureDoubleChow => "二般高".to_string(),
            &Self::PureTripleChow => "三同順".to_string(),
            &Self::MixedTripleChow => "三色順".to_string(),
            &Self::TriplePung => "三色刻".to_string(),
            &Self::PureShiftedPungs => "三連刻".to_string(),
            &Self::FourPureShiftedPungs => "四連刻".to_string(),
            &Self::BigFourWinds => "大四喜".to_string(),
            &Self::AllTerminals => "清老頭".to_string(),
            &Self::QuadrupleChow => "四同順".to_string(),
            &Self::NoPoint => "無役".to_string(),
        }
    }
}

impl TryFrom<usize> for Hand {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::AllChows),
            1 => Ok(Self::RedPung),
            2 => Ok(Self::GreenPung),
            3 => Ok(Self::WhitePung),
            4 => Ok(Self::WindPung),
            5 => Ok(Self::PureStraight),
            6 => Ok(Self::AllPungs),
            7 => Ok(Self::LittleThreeDragons),
            8 => Ok(Self::BigThreeDragons),
            9 => Ok(Self::LittleFourWinds),
            10 => Ok(Self::AllSimples),
            11 => Ok(Self::OutsideHands),
            12 => Ok(Self::TerminalsInAllSets),
            13 => Ok(Self::AllTerminalsAndHonors),
            14 => Ok(Self::HalfFlush),
            15 => Ok(Self::FullFlush),
            16 => Ok(Self::AllHonors),
            17 => Ok(Self::TwoConcealedPungs),
            18 => Ok(Self::ThreeConcealedPungs),
            19 => Ok(Self::FourConcealedPungs),
            20 => Ok(Self::PureDoubleChow),
            21 => Ok(Self::TwicePureDoubleChow),
            22 => Ok(Self::PureTripleChow),
            23 => Ok(Self::MixedTripleChow),
            24 => Ok(Self::TriplePung),
            25 => Ok(Self::PureShiftedPungs),
            26 => Ok(Self::FourPureShiftedPungs),
            27 => Ok(Self::BigFourWinds),
            28 => Ok(Self::AllTerminals),
            29 => Ok(Self::QuadrupleChow),
            30 => Ok(Self::NoPoint),
            _ => Err(()),
        }
    }
}
