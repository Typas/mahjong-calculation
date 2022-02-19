pub const HANDVARIANT: usize = Hand::NoPoint as usize + 1;
pub const HANDMAXSCORE: u16 = 16;

#[derive(Clone, Copy)]
pub enum Hand {
    AllChows,              // 平和
    MoonPung,              // 役牌陰
    SunPung,               // 役牌陽
    WindPung,              // 役牌自風(中)
    AllPungs,              // 對對和
    LittleThreeWinds,      // 小三元
    BigThreeWinds,         // 大三元
    TwoDragons,            // 雙喜臨門
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
    AllTerminals,          // 清老頭
    NoPoint,               // 無役
}

impl Hand {
    pub fn score(&self) -> u16 {
        match self {
            &Self::AllChows => 0,
            &Self::MoonPung => 2,
            &Self::SunPung => 2,
            &Self::WindPung => 2,
            &Self::AllPungs => 4,
            &Self::LittleThreeWinds => 4,
            &Self::BigThreeWinds => 16,
            &Self::TwoDragons => 2,
            &Self::AllSimples => 2,
            &Self::OutsideHands => 0,
            &Self::TerminalsInAllSets => 1,
            &Self::AllTerminalsAndHonors => 5,
            &Self::HalfFlush => 4,
            &Self::FullFlush => 8,
            &Self::AllHonors => HANDMAXSCORE,
            &Self::TwoConcealedPungs => 1,
            &Self::ThreeConcealedPungs => 3,
            &Self::PureDoubleChow => 1,
            &Self::PureTripleChow => 12,
            &Self::MixedTripleChow => 3,
            &Self::TriplePung => 12,
            &Self::PureShiftedPungs => 4,
            &Self::ThreePureShiftedPungs => 4,
            &Self::AllTerminals => 9,
            // 無役
            &Self::NoPoint => 0,
        }
    }

    pub fn name(&self) -> String {
        match self {
            &Self::AllChows => "平和".to_string(),
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
            &Hand::AllTerminals => "清老頭".to_string(),
            &Self::NoPoint => "無役".to_string(),
        }
    }
}

impl TryFrom<usize> for Hand {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::AllChows),
            1 => Ok(Self::MoonPung),
            2 => Ok(Self::SunPung),
            3 => Ok(Self::WindPung),
            4 => Ok(Self::AllPungs),
            5 => Ok(Self::LittleThreeWinds),
            6 => Ok(Self::BigThreeWinds),
            7 => Ok(Self::TwoDragons),
            8 => Ok(Self::AllSimples),
            9 => Ok(Self::OutsideHands),
            10 => Ok(Self::TerminalsInAllSets),
            11 => Ok(Self::AllTerminalsAndHonors),
            12 => Ok(Self::HalfFlush),
            13 => Ok(Self::FullFlush),
            14 => Ok(Self::AllHonors),
            15 => Ok(Self::TwoConcealedPungs),
            16 => Ok(Self::ThreeConcealedPungs),
            17 => Ok(Self::PureDoubleChow),
            18 => Ok(Self::PureTripleChow),
            19 => Ok(Self::MixedTripleChow),
            20 => Ok(Self::TriplePung),
            21 => Ok(Self::PureShiftedPungs),
            22 => Ok(Self::ThreePureShiftedPungs),
            23 => Ok(Self::AllTerminals),
            24 => Ok(Self::NoPoint),
            _ => Err(()),
        }
    }
}
