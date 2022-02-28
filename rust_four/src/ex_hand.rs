pub const HANDVARIANT: usize = Hand::NoPoint as usize + 1;
pub const HANDMAXSCORE: u16 = 32;

#[derive(Clone, Copy)]
pub enum Hand {
    AllChows,              // 平和
    AllRevealed,           // 全求人
    AllConcealed,          // 不求人
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
    OneKong,               // 一槓子
    TwoKongs,              // 二槓子
    ThreeKongs,            // 三槓子
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
    FourKongs,             // 四槓子
    NoPoint,               // 無役
}

impl Hand {
    pub fn score(&self) -> u16 {
        match self {
            &Self::AllChows => 1,
            &Self::AllRevealed => 2,
            &Self::AllConcealed => 2,
            &Self::RedPung => 2,
            &Self::GreenPung => 2,
            &Self::WhitePung => 2,
            &Self::WindPung => 2,
            &Self::PureStraight => 4,
            &Self::AllPungs => 4,
            &Self::LittleThreeDragons => 6,
            &Self::BigThreeDragons => 8,
            &Self::LittleFourWinds => 16,
            &Self::AllSimples => 1,
            &Self::OutsideHands => 4,
            &Self::TerminalsInAllSets => 6,
            &Self::AllTerminalsAndHonors => 12,
            &Self::HalfFlush => 4,
            &Self::FullFlush => 8,
            &Self::AllHonors => HANDMAXSCORE,
            &Self::TwoConcealedPungs => 1,
            &Self::ThreeConcealedPungs => 3,
            &Self::FourConcealedPungs => 4,
            &Self::OneKong => 1,
            &Self::TwoKongs => 4,
            &Self::ThreeKongs => 12,
            &Self::PureDoubleChow => 1,
            &Self::TwicePureDoubleChow => 8,
            &Self::PureTripleChow => 12,
            &Self::MixedTripleChow => 3,
            &Self::TriplePung => 12,
            &Self::PureShiftedPungs => 4,
            &Self::FourPureShiftedPungs => 16,
            // 絕對滿貫
            &Self::BigFourWinds => HANDMAXSCORE,
            &Self::AllTerminals => HANDMAXSCORE,
            &Self::QuadrupleChow => HANDMAXSCORE,
            &Self::FourKongs => HANDMAXSCORE,
            // 無役
            &Self::NoPoint => 0,
        }
    }

    pub fn name(&self) -> String {
        match self {
            &Self::AllChows => "平和".to_string(),
            &Self::AllRevealed => "全求人".to_string(),
            &Self::AllConcealed => "不求人".to_string(),
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
            &Self::OneKong => "一槓子".to_string(),
            &Self::TwoKongs => "二槓子".to_string(),
            &Self::ThreeKongs => "三槓子".to_string(),
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
            &Self::FourKongs => "四槓子".to_string(),
            &Self::NoPoint => "無役".to_string(),
        }
    }
}

impl TryFrom<usize> for Hand {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::AllChows),
            1 => Ok(Self::AllRevealed),
            2 => Ok(Self::AllConcealed),
            3 => Ok(Self::RedPung),
            4 => Ok(Self::GreenPung),
            5 => Ok(Self::WhitePung),
            6 => Ok(Self::WindPung),
            7 => Ok(Self::PureStraight),
            8 => Ok(Self::AllPungs),
            9 => Ok(Self::LittleThreeDragons),
            10 => Ok(Self::BigThreeDragons),
            11 => Ok(Self::LittleFourWinds),
            12 => Ok(Self::AllSimples),
            13 => Ok(Self::OutsideHands),
            14 => Ok(Self::TerminalsInAllSets),
            15 => Ok(Self::AllTerminalsAndHonors),
            16 => Ok(Self::HalfFlush),
            17 => Ok(Self::FullFlush),
            18 => Ok(Self::AllHonors),
            19 => Ok(Self::TwoConcealedPungs),
            20 => Ok(Self::ThreeConcealedPungs),
            21 => Ok(Self::FourConcealedPungs),
            22 => Ok(Self::OneKong),
            23 => Ok(Self::TwoKongs),
            24 => Ok(Self::ThreeKongs),
            25 => Ok(Self::PureDoubleChow),
            26 => Ok(Self::TwicePureDoubleChow),
            27 => Ok(Self::PureTripleChow),
            28 => Ok(Self::MixedTripleChow),
            29 => Ok(Self::TriplePung),
            30 => Ok(Self::PureShiftedPungs),
            31 => Ok(Self::FourPureShiftedPungs),
            32 => Ok(Self::BigFourWinds),
            33 => Ok(Self::AllTerminals),
            34 => Ok(Self::QuadrupleChow),
            35 => Ok(Self::FourKongs),
            36 => Ok(Self::NoPoint),
            _ => Err(()),
        }
    }
}
