use std::fmt::Display;

use LunarMonth::*;

#[cfg(feature = "serde")]
use serde::Serialize;

/// 格式
/// 正(闰月:true,初一日0:0:0的jd)
// #[cfg_attr(feature = "serde", derive(Serialize))]
// #[cfg_attr(feature = "swagger", derive(ToSchema))]
#[derive(Debug, Clone, Copy)]
pub enum LunarMonth {
    正(bool, f64),
    二(bool, f64),
    三(bool, f64),
    四(bool, f64),
    五(bool, f64),
    六(bool, f64),
    七(bool, f64),
    八(bool, f64),
    九(bool, f64),
    十(bool, f64),
    冬(bool, f64),
    腊(bool, f64),
}

impl LunarMonth {
    pub fn jd(&self) -> f64 {
        match self {
            正(_, jd) => *jd,
            二(_, jd) => *jd,
            三(_, jd) => *jd,
            四(_, jd) => *jd,
            五(_, jd) => *jd,
            六(_, jd) => *jd,
            七(_, jd) => *jd,
            八(_, jd) => *jd,
            九(_, jd) => *jd,
            十(_, jd) => *jd,
            冬(_, jd) => *jd,
            腊(_, jd) => *jd,
        }
    }

    pub fn is_leap(&self) -> bool {
        match self {
            正(leap, _) => *leap,
            二(leap, _) => *leap,
            三(leap, _) => *leap,
            四(leap, _) => *leap,
            五(leap, _) => *leap,
            六(leap, _) => *leap,
            七(leap, _) => *leap,
            八(leap, _) => *leap,
            九(leap, _) => *leap,
            十(leap, _) => *leap,
            冬(leap, _) => *leap,
            腊(leap, _) => *leap,
        }
    }

    pub fn to_num(&self) -> u8 {
        match self {
            正(_, _) => 1,
            二(_, _) => 2,
            三(_, _) => 3,
            四(_, _) => 4,
            五(_, _) => 5,
            六(_, _) => 6,
            七(_, _) => 7,
            八(_, _) => 8,
            九(_, _) => 9,
            十(_, _) => 10,
            冬(_, _) => 11,
            腊(_, _) => 12,
        }
    }

    /// 将一个月更改为前一个月的闰月
    pub fn to_pre_leap_month(&self) -> Self {
        match self {
            正(_, jd) => 腊(true, *jd),
            二(_, jd) => 正(true, *jd),
            三(_, jd) => 二(true, *jd),
            四(_, jd) => 三(true, *jd),
            五(_, jd) => 四(true, *jd),
            六(_, jd) => 五(true, *jd),
            七(_, jd) => 六(true, *jd),
            八(_, jd) => 七(true, *jd),
            九(_, jd) => 八(true, *jd),
            十(_, jd) => 九(true, *jd),
            冬(_, jd) => 十(true, *jd),
            腊(_, jd) => 冬(true, *jd),
        }
    }

    /// 将一个月更改为前一个非闰月
    pub fn to_pre_month(&self) -> Self {
        match self {
            正(_, jd) => 腊(false, *jd),
            二(_, jd) => 正(false, *jd),
            三(_, jd) => 二(false, *jd),
            四(_, jd) => 三(false, *jd),
            五(_, jd) => 四(false, *jd),
            六(_, jd) => 五(false, *jd),
            七(_, jd) => 六(false, *jd),
            八(_, jd) => 七(false, *jd),
            九(_, jd) => 八(false, *jd),
            十(_, jd) => 九(false, *jd),
            冬(_, jd) => 十(false, *jd),
            腊(_, jd) => 冬(false, *jd),
        }
    }
}

impl Display for LunarMonth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            正(leap, _) => {
                if *leap {
                    "闰正月"
                } else {
                    "正月"
                }
            }
            二(leap, _) => {
                if *leap {
                    "闰二月"
                } else {
                    "二月"
                }
            }
            三(leap, _) => {
                if *leap {
                    "闰三月"
                } else {
                    "三月"
                }
            }
            四(leap, _) => {
                if *leap {
                    "闰四月"
                } else {
                    "四月"
                }
            }
            五(leap, _) => {
                if *leap {
                    "闰五月"
                } else {
                    "五月"
                }
            }
            六(leap, _) => {
                if *leap {
                    "闰六月"
                } else {
                    "六月"
                }
            }
            七(leap, _) => {
                if *leap {
                    "闰七月"
                } else {
                    "七月"
                }
            }
            八(leap, _) => {
                if *leap {
                    "闰八月"
                } else {
                    "八月"
                }
            }
            九(leap, _) => {
                if *leap {
                    "闰九月"
                } else {
                    "九月"
                }
            }
            十(leap, _) => {
                if *leap {
                    "闰十月"
                } else {
                    "十月"
                }
            }
            冬(leap, _) => {
                if *leap {
                    "闰冬月"
                } else {
                    "冬月"
                }
            }
            腊(leap, _) => {
                if *leap {
                    "闰腊月"
                } else {
                    "腊月"
                }
            }
        };
        write!(f, "{}", s)
    }
}

#[cfg(feature = "serde")]
impl Serialize for LunarMonth {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
