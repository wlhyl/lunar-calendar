use SolarTerm::*;

#[cfg(feature = "serde")]
use serde::ser::SerializeStruct;
#[cfg(feature = "serde")]
use serde::Serialize;

// #[cfg(feature = "swagger")]
// use utoipa::ToSchema;

// pub const SOLAR_TERM_NAMES: [&str; 24] = [
//     "大雪", "冬至", "小寒", "大寒", "立春", "雨水", "惊蛰", "春分", "清明", "谷雨", "立夏", "小满",
//     "芒种", "夏至", "小暑", "大暑", "立秋", "处暑", "白露", "秋分", "寒露", "霜降", "立冬", "小雪",
// ];

/// 节气
// #[cfg_attr(feature = "serde", derive(Serialize))]
// #[cfg_attr(feature = "swagger", derive(ToSchema))]
#[derive(Debug, Clone, Copy)]
pub enum SolarTerm {
    冬至(i32, u8, u8, u8, u8, u8),
    小寒(i32, u8, u8, u8, u8, u8),
    大寒(i32, u8, u8, u8, u8, u8),
    立春(i32, u8, u8, u8, u8, u8),
    雨水(i32, u8, u8, u8, u8, u8),
    惊蛰(i32, u8, u8, u8, u8, u8),
    春分(i32, u8, u8, u8, u8, u8),
    清明(i32, u8, u8, u8, u8, u8),
    谷雨(i32, u8, u8, u8, u8, u8),
    立夏(i32, u8, u8, u8, u8, u8),
    小满(i32, u8, u8, u8, u8, u8),
    芒种(i32, u8, u8, u8, u8, u8),
    夏至(i32, u8, u8, u8, u8, u8),
    小暑(i32, u8, u8, u8, u8, u8),
    大暑(i32, u8, u8, u8, u8, u8),
    立秋(i32, u8, u8, u8, u8, u8),
    处暑(i32, u8, u8, u8, u8, u8),
    白露(i32, u8, u8, u8, u8, u8),
    秋分(i32, u8, u8, u8, u8, u8),
    寒露(i32, u8, u8, u8, u8, u8),
    霜降(i32, u8, u8, u8, u8, u8),
    立冬(i32, u8, u8, u8, u8, u8),
    小雪(i32, u8, u8, u8, u8, u8),
    大雪(i32, u8, u8, u8, u8, u8),
}

impl SolarTerm {
    pub fn name(&self) -> &str {
        match self {
            冬至(_, _, _, _, _, _) => "冬至",
            小寒(_, _, _, _, _, _) => "小寒",
            大寒(_, _, _, _, _, _) => "大寒",
            立春(_, _, _, _, _, _) => "立春",
            雨水(_, _, _, _, _, _) => "雨水",
            惊蛰(_, _, _, _, _, _) => "惊蛰",
            春分(_, _, _, _, _, _) => "春分",
            清明(_, _, _, _, _, _) => "清明",
            谷雨(_, _, _, _, _, _) => "谷雨",
            立夏(_, _, _, _, _, _) => "立夏",
            小满(_, _, _, _, _, _) => "小满",
            芒种(_, _, _, _, _, _) => "芒种",
            夏至(_, _, _, _, _, _) => "夏至",
            小暑(_, _, _, _, _, _) => "小暑",
            大暑(_, _, _, _, _, _) => "大暑",
            立秋(_, _, _, _, _, _) => "立秋",
            处暑(_, _, _, _, _, _) => "处暑",
            白露(_, _, _, _, _, _) => "白露",
            秋分(_, _, _, _, _, _) => "秋分",
            寒露(_, _, _, _, _, _) => "寒露",
            霜降(_, _, _, _, _, _) => "霜降",
            立冬(_, _, _, _, _, _) => "立冬",
            小雪(_, _, _, _, _, _) => "小雪",
            大雪(_, _, _, _, _, _) => "大雪",
        }
    }

    pub fn year(&self) -> i32 {
        match self {
            冬至(year, _, _, _, _, _) => *year,
            小寒(year, _, _, _, _, _) => *year,
            大寒(year, _, _, _, _, _) => *year,
            立春(year, _, _, _, _, _) => *year,
            雨水(year, _, _, _, _, _) => *year,
            惊蛰(year, _, _, _, _, _) => *year,
            春分(year, _, _, _, _, _) => *year,
            清明(year, _, _, _, _, _) => *year,
            谷雨(year, _, _, _, _, _) => *year,
            立夏(year, _, _, _, _, _) => *year,
            小满(year, _, _, _, _, _) => *year,
            芒种(year, _, _, _, _, _) => *year,
            夏至(year, _, _, _, _, _) => *year,
            小暑(year, _, _, _, _, _) => *year,
            大暑(year, _, _, _, _, _) => *year,
            立秋(year, _, _, _, _, _) => *year,
            处暑(year, _, _, _, _, _) => *year,
            白露(year, _, _, _, _, _) => *year,
            秋分(year, _, _, _, _, _) => *year,
            寒露(year, _, _, _, _, _) => *year,
            霜降(year, _, _, _, _, _) => *year,
            立冬(year, _, _, _, _, _) => *year,
            小雪(year, _, _, _, _, _) => *year,
            大雪(year, _, _, _, _, _) => *year,
        }
    }

    pub fn month(&self) -> u8 {
        match self {
            冬至(_, month, _, _, _, _) => *month,
            小寒(_, month, _, _, _, _) => *month,
            大寒(_, month, _, _, _, _) => *month,
            立春(_, month, _, _, _, _) => *month,
            雨水(_, month, _, _, _, _) => *month,
            惊蛰(_, month, _, _, _, _) => *month,
            春分(_, month, _, _, _, _) => *month,
            清明(_, month, _, _, _, _) => *month,
            谷雨(_, month, _, _, _, _) => *month,
            立夏(_, month, _, _, _, _) => *month,
            小满(_, month, _, _, _, _) => *month,
            芒种(_, month, _, _, _, _) => *month,
            夏至(_, month, _, _, _, _) => *month,
            小暑(_, month, _, _, _, _) => *month,
            大暑(_, month, _, _, _, _) => *month,
            立秋(_, month, _, _, _, _) => *month,
            处暑(_, month, _, _, _, _) => *month,
            白露(_, month, _, _, _, _) => *month,
            秋分(_, month, _, _, _, _) => *month,
            寒露(_, month, _, _, _, _) => *month,
            霜降(_, month, _, _, _, _) => *month,
            立冬(_, month, _, _, _, _) => *month,
            小雪(_, month, _, _, _, _) => *month,
            大雪(_, month, _, _, _, _) => *month,
        }
    }

    pub fn day(&self) -> u8 {
        match self {
            冬至(_, _, day, _, _, _) => *day,
            小寒(_, _, day, _, _, _) => *day,
            大寒(_, _, day, _, _, _) => *day,
            立春(_, _, day, _, _, _) => *day,
            雨水(_, _, day, _, _, _) => *day,
            惊蛰(_, _, day, _, _, _) => *day,
            春分(_, _, day, _, _, _) => *day,
            清明(_, _, day, _, _, _) => *day,
            谷雨(_, _, day, _, _, _) => *day,
            立夏(_, _, day, _, _, _) => *day,
            小满(_, _, day, _, _, _) => *day,
            芒种(_, _, day, _, _, _) => *day,
            夏至(_, _, day, _, _, _) => *day,
            小暑(_, _, day, _, _, _) => *day,
            大暑(_, _, day, _, _, _) => *day,
            立秋(_, _, day, _, _, _) => *day,
            处暑(_, _, day, _, _, _) => *day,
            白露(_, _, day, _, _, _) => *day,
            秋分(_, _, day, _, _, _) => *day,
            寒露(_, _, day, _, _, _) => *day,
            霜降(_, _, day, _, _, _) => *day,
            立冬(_, _, day, _, _, _) => *day,
            小雪(_, _, day, _, _, _) => *day,
            大雪(_, _, day, _, _, _) => *day,
        }
    }

    pub fn hour(&self) -> u8 {
        match self {
            冬至(_, _, _, hour, _, _) => *hour,
            小寒(_, _, _, hour, _, _) => *hour,
            大寒(_, _, _, hour, _, _) => *hour,
            立春(_, _, _, hour, _, _) => *hour,
            雨水(_, _, _, hour, _, _) => *hour,
            惊蛰(_, _, _, hour, _, _) => *hour,
            春分(_, _, _, hour, _, _) => *hour,
            清明(_, _, _, hour, _, _) => *hour,
            谷雨(_, _, _, hour, _, _) => *hour,
            立夏(_, _, _, hour, _, _) => *hour,
            小满(_, _, _, hour, _, _) => *hour,
            芒种(_, _, _, hour, _, _) => *hour,
            夏至(_, _, _, hour, _, _) => *hour,
            小暑(_, _, _, hour, _, _) => *hour,
            大暑(_, _, _, hour, _, _) => *hour,
            立秋(_, _, _, hour, _, _) => *hour,
            处暑(_, _, _, hour, _, _) => *hour,
            白露(_, _, _, hour, _, _) => *hour,
            秋分(_, _, _, hour, _, _) => *hour,
            寒露(_, _, _, hour, _, _) => *hour,
            霜降(_, _, _, hour, _, _) => *hour,
            立冬(_, _, _, hour, _, _) => *hour,
            小雪(_, _, _, hour, _, _) => *hour,
            大雪(_, _, _, hour, _, _) => *hour,
        }
    }

    pub fn minute(&self) -> u8 {
        match self {
            冬至(_, _, _, _, minute, _) => *minute,
            小寒(_, _, _, _, minute, _) => *minute,
            大寒(_, _, _, _, minute, _) => *minute,
            立春(_, _, _, _, minute, _) => *minute,
            雨水(_, _, _, _, minute, _) => *minute,
            惊蛰(_, _, _, _, minute, _) => *minute,
            春分(_, _, _, _, minute, _) => *minute,
            清明(_, _, _, _, minute, _) => *minute,
            谷雨(_, _, _, _, minute, _) => *minute,
            立夏(_, _, _, _, minute, _) => *minute,
            小满(_, _, _, _, minute, _) => *minute,
            芒种(_, _, _, _, minute, _) => *minute,
            夏至(_, _, _, _, minute, _) => *minute,
            小暑(_, _, _, _, minute, _) => *minute,
            大暑(_, _, _, _, minute, _) => *minute,
            立秋(_, _, _, _, minute, _) => *minute,
            处暑(_, _, _, _, minute, _) => *minute,
            白露(_, _, _, _, minute, _) => *minute,
            秋分(_, _, _, _, minute, _) => *minute,
            寒露(_, _, _, _, minute, _) => *minute,
            霜降(_, _, _, _, minute, _) => *minute,
            立冬(_, _, _, _, minute, _) => *minute,
            小雪(_, _, _, _, minute, _) => *minute,
            大雪(_, _, _, _, minute, _) => *minute,
        }
    }

    pub fn second(&self) -> u8 {
        match self {
            冬至(_, _, _, _, _, second) => *second,
            小寒(_, _, _, _, _, second) => *second,
            大寒(_, _, _, _, _, second) => *second,
            立春(_, _, _, _, _, second) => *second,
            雨水(_, _, _, _, _, second) => *second,
            惊蛰(_, _, _, _, _, second) => *second,
            春分(_, _, _, _, _, second) => *second,
            清明(_, _, _, _, _, second) => *second,
            谷雨(_, _, _, _, _, second) => *second,
            立夏(_, _, _, _, _, second) => *second,
            小满(_, _, _, _, _, second) => *second,
            芒种(_, _, _, _, _, second) => *second,
            夏至(_, _, _, _, _, second) => *second,
            小暑(_, _, _, _, _, second) => *second,
            大暑(_, _, _, _, _, second) => *second,
            立秋(_, _, _, _, _, second) => *second,
            处暑(_, _, _, _, _, second) => *second,
            白露(_, _, _, _, _, second) => *second,
            秋分(_, _, _, _, _, second) => *second,
            寒露(_, _, _, _, _, second) => *second,
            霜降(_, _, _, _, _, second) => *second,
            立冬(_, _, _, _, _, second) => *second,
            小雪(_, _, _, _, _, second) => *second,
            大雪(_, _, _, _, _, second) => *second,
        }
    }
}

#[cfg(feature = "serde")]
impl Serialize for SolarTerm {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let (name, year, month, day, hour, minute, second) = match self {
            冬至(y, m, d, h, mi, sec) => ("冬至", y, m, d, h, mi, sec),
            小寒(y, m, d, h, mi, sec) => ("小寒", y, m, d, h, mi, sec),
            大寒(y, m, d, h, mi, sec) => ("大寒", y, m, d, h, mi, sec),
            立春(y, m, d, h, mi, sec) => ("立春", y, m, d, h, mi, sec),
            雨水(y, m, d, h, mi, sec) => ("雨水", y, m, d, h, mi, sec),
            惊蛰(y, m, d, h, mi, sec) => ("惊蛰", y, m, d, h, mi, sec),
            春分(y, m, d, h, mi, sec) => ("春分", y, m, d, h, mi, sec),
            清明(y, m, d, h, mi, sec) => ("清明", y, m, d, h, mi, sec),
            谷雨(y, m, d, h, mi, sec) => ("谷雨", y, m, d, h, mi, sec),
            立夏(y, m, d, h, mi, sec) => ("立夏", y, m, d, h, mi, sec),
            小满(y, m, d, h, mi, sec) => ("小满", y, m, d, h, mi, sec),
            芒种(y, m, d, h, mi, sec) => ("芒种", y, m, d, h, mi, sec),
            夏至(y, m, d, h, mi, sec) => ("夏至", y, m, d, h, mi, sec),
            小暑(y, m, d, h, mi, sec) => ("小暑", y, m, d, h, mi, sec),
            大暑(y, m, d, h, mi, sec) => ("大暑", y, m, d, h, mi, sec),
            立秋(y, m, d, h, mi, sec) => ("立秋", y, m, d, h, mi, sec),
            处暑(y, m, d, h, mi, sec) => ("处暑", y, m, d, h, mi, sec),
            白露(y, m, d, h, mi, sec) => ("白露", y, m, d, h, mi, sec),
            秋分(y, m, d, h, mi, sec) => ("秋分", y, m, d, h, mi, sec),
            寒露(y, m, d, h, mi, sec) => ("寒露", y, m, d, h, mi, sec),
            霜降(y, m, d, h, mi, sec) => ("霜降", y, m, d, h, mi, sec),
            立冬(y, m, d, h, mi, sec) => ("立冬", y, m, d, h, mi, sec),
            小雪(y, m, d, h, mi, sec) => ("小雪", y, m, d, h, mi, sec),
            大雪(y, m, d, h, mi, sec) => ("大雪", y, m, d, h, mi, sec),
        };

        let mut s = serializer.serialize_struct("SolarTerm", 7)?;
        s.serialize_field("name", name)?;
        s.serialize_field("year", year)?;
        s.serialize_field("month", month)?;
        s.serialize_field("day", day)?;
        s.serialize_field("hour", hour)?;
        s.serialize_field("minute", minute)?;
        s.serialize_field("second", second)?;
        s.end()
    }
}
