use ganzhiwuxing::GanZhi;

#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg(feature = "swagger")]
use utoipa::ToSchema;

use crate::lunar_day::LunarDay;
use crate::lunar_month::LunarMonth;
use crate::solar_term::SolarTerm;

#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
#[derive(Debug)]
pub struct LunarCalendar {
    /// 闰年:true
    pub is_lean_year: bool,

    /// 农历年，干支表示
    pub lunar_year: GanZhi,

    /// 农历月，以正月、二月、......、十月、冬月、腊月表示
    #[schema(example="正月", value_type=String, example="正月")]
    pub lunar_month: LunarMonth,

    /// 农历日，以初一、初二、……、二十九、三十表示
    #[cfg_attr(feature = "swagger", schema(value_type = String, example="初一"))]
    pub lunar_day: LunarDay,

    /// 农历年干支，按节气换年
    pub lunar_year_gan_zhi: GanZhi,

    /// 农历月干支，按节气换月
    pub lunar_month_gan_zhi: GanZhi,

    /// 日干支
    pub lunar_day_gan_zhi: GanZhi,

    /// 时干支
    pub time_gan_zhi: GanZhi,

    /// 节
    #[cfg_attr(feature = "swagger", schema(value_type = String, example = json!({"name": "大雪", "year": 2024, "month": 12, "day": 6, "hour": 23, "minute": 17, "second": 2}  )))]
    pub solar_term_first: SolarTerm,

    /// 中气
    #[cfg_attr(feature = "swagger", schema(value_type = String,example =json!( {"name": "冬至", "year": 2024, "month": 12, "day": 21, "hour": 17, "minute": 20, "second": 34})  ))]
    pub solar_term_second: SolarTerm,
}
