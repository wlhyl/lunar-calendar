use ganzhiwuxin::GanZhi;

// 农历月信息
#[derive(Default, Debug, Clone)]
pub struct LunarMonth {
    pub num: u8,
    pub jd: f64,
    pub month_name: String,
    pub is_leap: bool,
}

#[derive(Debug)]
pub struct LunarCalendar {
    /**
     * 闰年:true
     */
    pub is_lean_year: bool,

    /**
     * 农历年，干支表示
     */
    pub lunar_year: GanZhi,

    /**
     * 农历月，以正月、二月、......、十月、冬月、腊月表示
     */
    pub lunar_month: String,

    /**
     * 农历日，以初一、初二、……、二十九、三十表示
     */
    pub lunar_day: String,

    /**
     * 农历年干支，按节气换年
     */
    pub lunar_year_gan_zhi: GanZhi,

    /**
     * 农历月干支，按节气换月
     */
    pub lunar_month_gan_zhi: GanZhi,

    /**
     * 日干支
     */
    pub lunar_day_gan_zhi: GanZhi,

    /**
     * 时干支
     */
    pub time_gan_zhi: GanZhi,

    /**
     * 节
     */
    pub solar_term_first: SolarTerm,

    /**
     * 中气
     */
    pub solar_term_second: SolarTerm,
}

/// 节气
#[derive(Debug)]
pub struct SolarTerm {
    pub name: String,
    pub year: i32,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}
