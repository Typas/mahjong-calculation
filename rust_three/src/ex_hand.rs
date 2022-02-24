pub const HANDVARIANT: usize = Hand::NoPoint as usize + 1;
pub const HANDMAXSCORE: u16 = 16;

#[derive(Clone, Copy)]
pub enum Hand {
    AllChows,              // 平和
    AllRevealed,           // 全求人
    AllConcealed,          // 不求人
    MoonPung,              // 役牌陰
    SunPung,               // 役牌陽
    WindPung,              // 役牌自風(中)
    AllPungs,              // 對對和
    TwoDragons,            // 雙喜臨門
    LittleThreeWinds,      // 小三元
    BigThreeWinds,         // 大三元
    AllSimples,            // 斷幺
    OutsideHands,          // 混全帶幺
    TerminalsInAllSets,    // 清全帶幺
    AllTerminalsAndHonors, // 混老頭
    HalfFlush,             // 混一色
    FullFlush,             // 清一色
    AllHonors,             // 字一色
    TwoConcealedPungs,     // 二暗刻
    ThreeConcealedPungs,   // 三暗刻
    PureDoubleChow,        // 一般高
    PureTripleChow,        // 三同順
    MixedTripleChow,       // 三色同順
    TriplePung,            // 三色同刻
    PureShiftedPungs,      // 二連刻
    ThreePureShiftedPungs, // 三連刻
    OneKong,               // 一槓子
    TwoKongs,              // 二槓子
    ThreeKongs,            // 三槓子
    AllTerminals,          // 清老頭
    NoPoint,               // 無役
}

impl Hand {
    pub fn score(&self) -> u16 {
        match self {
            &Self::AllChows => 0,
            &Self::AllRevealed => 2,
            &Self::AllConcealed => 2,
            &Self::MoonPung => 2,
            &Self::SunPung => 2,
            &Self::WindPung => 2,
            &Self::AllPungs => 3,
            &Self::TwoDragons => 2,
            &Self::LittleThreeWinds => 4,
            &Self::BigThreeWinds => 16,
            &Self::AllSimples => 2,
            &Self::OutsideHands => 1,
            &Self::TerminalsInAllSets => 2,
            &Self::AllTerminalsAndHonors => 4,
            &Self::HalfFlush => 3,
            &Self::FullFlush => 6,
            &Self::AllHonors => HANDMAXSCORE,
            &Self::TwoConcealedPungs => 1,
            &Self::ThreeConcealedPungs => 3,
            &Self::PureDoubleChow => 2,
            &Self::PureTripleChow => 12,
            &Self::MixedTripleChow => 3,
            &Self::TriplePung => 12,
            &Self::PureShiftedPungs => 3,
            &Self::ThreePureShiftedPungs => 6,
            &Self::OneKong => 2,
            &Self::TwoKongs => 6,
            &Self::ThreeKongs => HANDMAXSCORE,
            &Self::AllTerminals => 12,
            // 無役
            &Self::NoPoint => 0,
        }
    }

    pub fn name(&self) -> String {
        match self {
            &Self::AllChows => "平和".to_string(),
            &Self::AllRevealed => "全求人".to_string(),
            &Self::AllConcealed => "不求人".to_string(),
            &Self::MoonPung | &Self::SunPung | &Self::WindPung => "役牌".to_string(),
            &Self::AllPungs => "對對和".to_string(),
            &Self::LittleThreeWinds => "小三元".to_string(),
            &Self::BigThreeWinds => "大三元".to_string(),
            &Self::TwoDragons => "雙喜".to_string(),
            &Self::AllSimples => "斷幺".to_string(),
            &Self::OutsideHands => "混全帶".to_string(),
            &Self::TerminalsInAllSets => "清全帶".to_string(),
            &Self::AllTerminalsAndHonors => "混老頭".to_string(),
            &Self::HalfFlush => "混一色".to_string(),
            &Self::FullFlush => "清一色".to_string(),
            &Self::AllHonors => "字一色".to_string(),
            &Self::TwoConcealedPungs => "二暗刻".to_string(),
            &Self::ThreeConcealedPungs => "三暗刻".to_string(),
            &Self::PureDoubleChow => "一般高".to_string(),
            &Self::PureTripleChow => "三同順".to_string(),
            &Self::MixedTripleChow => "三色順".to_string(),
            &Self::TriplePung => "三色刻".to_string(),
            &Self::PureShiftedPungs => "二連刻".to_string(),
            &Self::ThreePureShiftedPungs => "三連刻".to_string(),
            &Self::OneKong => "一槓子".to_string(),
            &Self::TwoKongs => "二槓子".to_string(),
            &Self::ThreeKongs => "三槓子".to_string(),
            &Self::AllTerminals => "清老頭".to_string(),
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
            3 => Ok(Self::MoonPung),
            4 => Ok(Self::SunPung),
            5 => Ok(Self::WindPung),
            6 => Ok(Self::AllPungs),
            7 => Ok(Self::TwoDragons),
            8 => Ok(Self::LittleThreeWinds),
            9 => Ok(Self::BigThreeWinds),
            10 => Ok(Self::AllSimples),
            11 => Ok(Self::OutsideHands),
            12 => Ok(Self::TerminalsInAllSets),
            13 => Ok(Self::AllTerminalsAndHonors),
            14 => Ok(Self::HalfFlush),
            15 => Ok(Self::FullFlush),
            16 => Ok(Self::AllHonors),
            17 => Ok(Self::TwoConcealedPungs),
            18 => Ok(Self::ThreeConcealedPungs),
            19 => Ok(Self::PureDoubleChow),
            20 => Ok(Self::PureTripleChow),
            21 => Ok(Self::MixedTripleChow),
            22 => Ok(Self::TriplePung),
            23 => Ok(Self::PureShiftedPungs),
            24 => Ok(Self::ThreePureShiftedPungs),
            25 => Ok(Self::OneKong),
            26 => Ok(Self::TwoKongs),
            27 => Ok(Self::ThreeKongs),
            28 => Ok(Self::AllTerminals),
            29 => Ok(Self::NoPoint),
            _ => Err(()),
        }
    }
}
