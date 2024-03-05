mod constdef;
mod mathutl;
mod typedef;
mod utils;
mod vaild;

use constdef::{DAY_NAMES, MONTH_NAMES, SOLAR_TERM_NAMES};
use ganzhiwuxing::{
    DiZhi::*,
    GanZhi::{self, *},
    TianGan::*,
};
use mathutl::{get_ut8_date_time_from_jd, mod180, newton_iteration};
use swe::{
    swe_calc_ut, swe_close, swe_degnorm, swe_julday, swe_set_ephe_path, swe_utc_time_zone, Body,
    Calendar,
};

use typedef::LunarMonth;
pub use typedef::{LunarCalendar, SolarTerm};
use utils::{calc_leap_month, get15_lunar_month_jds, get15_new_moon_jds, get25_solar_term_jds};
use vaild::vaild_date_time;

/// 从公历日期得到农历日期
pub fn lunar_calendar(
    year: i32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    ephe_path: &str,
) -> Result<LunarCalendar, String> {
    let mut lunar_calendar = LunarCalendar {
        is_lean_year: false,
        lunar_year: 甲子,
        lunar_month: "".to_string(),
        lunar_day: "".to_string(),
        lunar_year_gan_zhi: 甲子,
        lunar_month_gan_zhi: 甲子,
        lunar_day_gan_zhi: 甲子,
        time_gan_zhi: 甲子,
        solar_term_first: SolarTerm {
            name: "".to_string(),
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
            minute: 0,
            second: 0,
        },
        solar_term_second: SolarTerm {
            name: "".to_string(),
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
            minute: 0,
            second: 0,
        },
    };

    if ephe_path == "" {
        return Err("ephe_path must be specified".to_owned());
    }
    vaild_date_time(year, month, day, hour, minute, second)?;

    // 从前一年冬至到此年冬至间的25个节气，第25个节气=此年冬至
    let solar_term_jds = get25_solar_term_jds(year - 1, ephe_path)?;

    // 从前一年冬至所在农历开始的15个新月的jd
    let new_moon_jds = get15_new_moon_jds(solar_term_jds[0], ephe_path)?;

    // 从前一年冬至所在农历月开始的15个农历月的初一的儒略日，以东八区时间为准
    let lunar_months = get15_lunar_month_jds(new_moon_jds);

    // 计算闰月，如果有闰月，修正月的num
    let lunar_months = calc_leap_month(
        lunar_months,
        solar_term_jds
            .iter()
            .enumerate()
            .filter(|(index, _)| index % 2 == 0)
            .map(|(_, &e)| e)
            .collect::<Vec<f64>>()
            .try_into()
            .unwrap(),
    );

    // 设置闰年

    if lunar_months.iter().find(|month| month.is_leap).is_some() {
        lunar_calendar.is_lean_year = true;
    }

    // 得到月名
    let lunar_months: Vec<_> = lunar_months
        .into_iter()
        .map(|month| LunarMonth {
            num: month.num,
            jd: month.jd,
            month_name: if month.is_leap {
                format!("闰{}月", MONTH_NAMES[month.num as usize - 1])
            } else {
                format!("{}月", MONTH_NAMES[month.num as usize - 1])
            },
            is_leap: month.is_leap,
        })
        .collect();

    /*
       将公历转换为农历
       为方便计算，可以取20:00:00，utc此时为12:00:00
       此处默认计算00:00:00
    */
    // 计算农历月和农历日
    // 1582年10月15日00:00:00起为格里高利历
    let mut calendar = Calendar::Gregorian;
    if year < 1582 {
        calendar = Calendar::Julian;
    }
    if year == 1582 && month < 10 {
        calendar = Calendar::Julian;
    }
    if year == 1582 && month == 10 && day < 15 {
        calendar = Calendar::Julian;
    }

    let (cyear, cmonth, cday, chour, cminute, csecond): (i32, i32, i32, i32, i32, f64) =
        swe_utc_time_zone(
            year,
            month.into(),
            day.into(),
            hour.into(),
            minute.into(),
            second.into(),
            8.0,
        );
    // swephgo.UtcTimeZone(year, month, day, hour, minute, float64(second), 8.0,
    // 	cyear, cmonth, cday, chour, cminute, csecond)
    let current_jd = swe_julday(
        cyear,
        cmonth,
        cday,
        chour as f64 + cminute as f64 / 60.0 + csecond as f64 / 3600.0,
        calendar,
    );

    // 找出当前日期所在农历月
    let mut n = 0;
    for i in 0..lunar_months.len() {
        if lunar_months[i].jd <= current_jd && current_jd < lunar_months[i + 1].jd {
            n = i;
            break;
        }
    }
    lunar_calendar.lunar_month = lunar_months[n].month_name.clone();
    let days = (current_jd - lunar_months[n].jd).floor() as usize;
    lunar_calendar.lunar_day = DAY_NAMES[days].to_string();

    // 计算年
    // 根据2017年国标，农历年用干支表示
    // firstLunarMonth： 农历正月
    let first_lunar_month = if let Some(month) = lunar_months
        .iter()
        .find(|month| month.num == 1 && !month.is_leap)
    {
        month.clone()
    } else {
        lunar_months[0].clone()
    };

    // 计算农历年
    if current_jd < first_lunar_month.jd {
        lunar_calendar.lunar_year = 甲子.plus(year as isize - 1 - 1864);
    } else {
        lunar_calendar.lunar_year = 甲子.plus(year as isize - 1864)
    }

    // 计算农历日干支
    // 计算日柱, 以2017年4月7日，甲子日为起点
    let d = current_jd - swe_julday(2017, 4, 6, 16.0, Calendar::Gregorian);
    let d = d.floor() as isize;
    lunar_calendar.lunar_day_gan_zhi = 甲子.plus(d);

    // 年干支，以立春换年
    // solarTermJds[3]是立春
    if current_jd < solar_term_jds[3] {
        lunar_calendar.lunar_year_gan_zhi = 甲子.plus(year as isize - 1 - 1864);
    } else {
        lunar_calendar.lunar_year_gan_zhi = 甲子.plus(year as isize - 1864);
    }

    // 计算月柱
    // 默认为00:00:00所在的月柱，立春换月柱
    // 大雪的黄经=255度
    // monthNum在计算节气时会用到
    // var monthNum int
    // lunarCalendar.LunarMonthGanZhi, err = func() (ganzhiwuxin.GanZhi, error) {

    swe_set_ephe_path(ephe_path);

    let xx: [f64; 6] = match swe_calc_ut(current_jd, &Body::SeSun, Default::default()) {
        Ok(xx) => xx,
        Err(e) => {
            swe_close();
            return Err(e);
        }
    };

    swe_close();

    let month_num = (swe_degnorm(xx[0] - 255.0) / 30.0).floor() as usize;

    let month_di_zhi = 子.plus(month_num.try_into().unwrap());

    // 求月柱，按节气换年，不能使用农历正月初一换年，如果 2017年1月7日，节气年、农历年都是丙申，
    // 不能以monthNum < 2，将农历的丙申 - 1
    let year_gan = lunar_calendar.lunar_year_gan_zhi.gan();
    if year_gan == 甲 || year_gan == 己 {
        let n = month_di_zhi.minus(&寅);
        let g = 丙.plus(n as isize);
        lunar_calendar.lunar_month_gan_zhi = GanZhi::new(&g, &month_di_zhi).unwrap();
    } else if year_gan == 乙 || year_gan == 庚 {
        let n = month_di_zhi.minus(&寅);
        let g = 戊.plus(n as isize);
        lunar_calendar.lunar_month_gan_zhi = GanZhi::new(&g, &month_di_zhi).unwrap();
    } else if year_gan == 丙 || year_gan == 辛 {
        let n = month_di_zhi.minus(&寅);
        let g = 庚.plus(n as isize);
        lunar_calendar.lunar_month_gan_zhi = GanZhi::new(&g, &month_di_zhi).unwrap();
    } else if year_gan == 丁 || year_gan == 壬 {
        let n = month_di_zhi.minus(&寅);
        let g = 壬.plus(n as isize);
        lunar_calendar.lunar_month_gan_zhi = GanZhi::new(&g, &month_di_zhi).unwrap();
    } else {
        //戊，癸
        let n = month_di_zhi.minus(&寅);
        let g = 甲.plus(n as isize);
        lunar_calendar.lunar_month_gan_zhi = GanZhi::new(&g, &month_di_zhi).unwrap()
    }

    // 计算时柱, (hour + 1) / 2 = 时辰数-1, 0点子时=1,丑时=2,辰时=3... 亥时=11,23点=12
    // lunarCalendar.TimeGanZhi = func() ganzhiwuxin.GanZhi {
    let n = (hour + 1) / 2;

    let day_gan = lunar_calendar.lunar_day_gan_zhi.gan();

    if day_gan == 甲 || day_gan == 己 {
        lunar_calendar.time_gan_zhi = 甲子.plus(n.into());
    } else if day_gan == 乙 || day_gan == 庚 {
        lunar_calendar.time_gan_zhi = 丙子.plus(n.into());
    } else if day_gan == 丙 || day_gan == 辛 {
        lunar_calendar.time_gan_zhi = 戊子.plus(n.into());
    } else if day_gan == 丁 || day_gan == 壬 {
        lunar_calendar.time_gan_zhi = 庚子.plus(n.into());
    } else {
        lunar_calendar.time_gan_zhi = 壬子.plus(n.into());
    }

    // 计算此日期所在的节气
    let solar_term_jd0 = newton_iteration(current_jd, |jd| {
        swe_set_ephe_path(ephe_path);

        let xx: [f64; 6] = match swe_calc_ut(jd, &Body::SeSun, Default::default()) {
            Ok(xx) => xx,
            Err(e) => {
                swe_close();
                return Err(e);
            }
        };
        swe_close();
        Ok(mod180(xx[0] - swe_degnorm(month_num as f64 * 30.0 + 255.0)))
    })?;

    let (y8, m8, d8, h8, mi8, sec8) = get_ut8_date_time_from_jd(solar_term_jd0);
    lunar_calendar.solar_term_first = SolarTerm {
        name: SOLAR_TERM_NAMES[month_num * 2].to_owned(),
        year: y8,
        month: m8,
        day: d8,
        hour: h8,
        minute: mi8,
        second: sec8.floor() as u8,
    };

    let solar_term_jd1 = newton_iteration(solar_term_jd0 + 15.0, |jd| {
        swe_set_ephe_path(ephe_path);

        let xx: [f64; 6] = match swe_calc_ut(jd, &Body::SeSun, Default::default()) {
            Ok(xx) => xx,
            Err(e) => {
                swe_close();
                return Err(e);
            }
        };
        swe_close();
        Ok(mod180(
            xx[0] - swe_degnorm(month_num as f64 * 30.0 + 255.0 + 15.0),
        ))
    })?;

    let (y8, m8, d8, h8, mi8, sec8) = get_ut8_date_time_from_jd(solar_term_jd1);
    lunar_calendar.solar_term_second = SolarTerm {
        name: SOLAR_TERM_NAMES[month_num * 2 + 1].to_owned(),
        year: y8,
        month: m8,
        day: d8,
        hour: h8,
        minute: mi8,
        second: sec8.floor() as u8,
    };

    Ok(lunar_calendar)
}
#[cfg(test)]
mod tests {
    use std::env;

    use crate::lunar_calendar;

    // 将2022-1-10 22:5:3转换为农历
    #[test]
    fn test_convert_to_lunar_calendar2022_1_10_22_5_3() {
        // "测试公历转农历"
        // "将2022-1-10 22:5:3转换为农历
        dotenv::dotenv().ok();
        let ephe_path = env::var("EPHE_PATH")
            .expect("没设置 EPHE_PATH 环境变量，可在.env文件中设置或export EPHE_PATH=...");
        let year = 2022;
        let month = 1;
        let day = 10;
        let hour = 22;
        let minute = 5;
        let second = 3;

        let data = lunar_calendar(year, month, day, hour, minute, second, &ephe_path);
        assert!(data.is_ok(), "{:?}", data);
        let data = data.unwrap();

        assert!(
            !data.is_lean_year,
            "{}-{}-{} {}:{}:{} 不是闰年",
            year, month, day, hour, minute, second
        );

        // 农历年，干支表示
        assert_eq!(
            data.lunar_year.to_string(),
            "辛丑",
            "{}-{}-{} {}:{}:{} 是辛丑，而非{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            data.lunar_year
        );

        // 农历月，以正月、二月、......、十月、冬月、腊月表示
        assert_eq!(
            data.lunar_month.to_string(),
            "腊月",
            "{}-{}-{} {}:{}:{} 是腊月，而非{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            data.lunar_month
        );

        //  农历日，以初一、初二、……、二十九、三十表示
        assert_eq!(
            data.lunar_day.to_string(),
            "初八",
            "{}-{}-{} {}:{}:{} 是初八，而非{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            data.lunar_day.to_string()
        );

        // 农历年干支，按节气换年

        assert_eq!(
            data.lunar_year_gan_zhi.to_string(),
            "辛丑",
            "{}-{}-{} {}:{}:{} 节气年干支是辛丑，而非{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            data.lunar_year_gan_zhi
        );

        // 农历月干支，按节气换月

        assert_eq!(
            data.lunar_month_gan_zhi.to_string(),
            "辛丑",
            "{}-{}-{} {}:{}:{} 月干支是辛丑，而非{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            data.lunar_month_gan_zhi
        );

        // 日干支
        assert_eq!(
            data.lunar_day_gan_zhi.to_string(),
            "癸亥",
            "{}-{}-{} {}:{}:{} 日干支是癸亥，而非{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            data.lunar_day_gan_zhi
        );

        // 时干支
        assert_eq!(
            data.time_gan_zhi.to_string(),
            "癸亥",
            "{}-{}-{} {}:{}:{} 时干支是癸亥，而非{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            data.time_gan_zhi
        );

        // 节
        let solar_term = data.solar_term_first;
        assert!(
            solar_term.name == "小寒"
                && solar_term.year == 2022
                && solar_term.month == 1
                && solar_term.day == 5
                && solar_term.hour == 17,
            "{}-{}-{} {}:{}:{} 的节是`小寒 2022-1-5 17:13:54`，而非{} {}-{}-{} {}:{}:{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            solar_term.name,
            solar_term.year,
            solar_term.month,
            solar_term.day,
            solar_term.hour,
            solar_term.minute,
            solar_term.second
        );

        // 中气

        let solar_term = data.solar_term_second;
        assert!(
            solar_term.name == "大寒"
                && solar_term.year == 2022
                && solar_term.month == 1
                && solar_term.day == 20
                && solar_term.hour == 10,
            "{}-{}-{} {}:{}:{} 的节是`大寒 2022-1-20 10:38:56`，而非{} {}-{}-{} {}:{}:{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            solar_term.name,
            solar_term.year,
            solar_term.month,
            solar_term.day,
            solar_term.hour,
            solar_term.minute,
            solar_term.second
        );
    }

    // 将2022-2-3 22:5:3转换为农历
    #[test]
    fn test_convert_to_lunar_calendar2022_2_3_22_5_3() {
        // 测试公历转农历
        // 将2022-3-3 22:5:3转换为农历
        dotenv::dotenv().ok();
        let ephe_path = env::var("EPHE_PATH")
            .expect("没设置 EPHE_PATH 环境变量，可在.env文件中设置或export EPHE_PATH=...");
        let year = 2022;
        let month = 2;
        let day = 3;
        let hour = 22;
        let minute = 5;
        let second = 3;
        let data = lunar_calendar(year, month, day, hour, minute, second, &ephe_path);
        assert!(data.is_ok(), "{:?}", data);
        let data = data.unwrap();

        assert!(
            !data.is_lean_year,
            "{}-{}-{} {}:{}:{} 不是闰年",
            year, month, day, hour, minute, second
        );

        // 农历年，干支表示
        assert_eq!(
            data.lunar_year.to_string(),
            "壬寅",
            "{}-{}-{} {}:{}:{} 是壬寅，而非{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            data.lunar_year
        );

        // 农历月，以正月、二月、......、十月、冬月、腊月表示
        assert_eq!(
            data.lunar_month.to_string(),
            "正月",
            "{}-{}-{} {}:{}:{} 是正月，而非{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            data.lunar_month
        );

        //  农历日，以初一、初二、……、二十九、三十表示
        assert_eq!(
            data.lunar_day.to_string(),
            "初三",
            "{}-{}-{} {}:{}:{} 是初三，而非{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            data.lunar_day.to_string()
        );

        // 农历年干支，按节气换年

        assert_eq!(
            data.lunar_year_gan_zhi.to_string(),
            "辛丑",
            "{}-{}-{} {}:{}:{} 节气年干支是辛丑，而非{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            data.lunar_year_gan_zhi
        );

        // 农历月干支，按节气换月

        assert_eq!(
            data.lunar_month_gan_zhi.to_string(),
            "辛丑",
            "{}-{}-{} {}:{}:{} 月干支是辛丑，而非{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            data.lunar_month_gan_zhi
        );

        // 日干支
        assert_eq!(
            data.lunar_day_gan_zhi.to_string(),
            "丁亥",
            "{}-{}-{} {}:{}:{} 日干支是丁亥，而非{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            data.lunar_day_gan_zhi
        );

        // 时干支
        assert_eq!(
            data.time_gan_zhi.to_string(),
            "辛亥",
            "{}-{}-{} {}:{}:{} 时干支是辛亥，而非{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            data.time_gan_zhi
        );

        // 节
        let solar_term = data.solar_term_first;
        assert!(
            solar_term.name == "小寒"
                && solar_term.year == 2022
                && solar_term.month == 1
                && solar_term.day == 5
                && solar_term.hour == 17,
            "{}-{}-{} {}:{}:{} 的节是`小寒 2022-1-5 17:13:54`，而非{} {}-{}-{} {}:{}:{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            solar_term.name,
            solar_term.year,
            solar_term.month,
            solar_term.day,
            solar_term.hour,
            solar_term.minute,
            solar_term.second
        );

        // 中气
        let solar_term = data.solar_term_second;
        assert!(
            solar_term.name == "大寒"
                && solar_term.year == 2022
                && solar_term.month == 1
                && solar_term.day == 20
                && solar_term.hour == 10,
            "{}-{}-{} {}:{}:{} 的节是`大寒 2022-1-20 10:38:56`，而非{} {}-{}-{} {}:{}:{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            solar_term.name,
            solar_term.year,
            solar_term.month,
            solar_term.day,
            solar_term.hour,
            solar_term.minute,
            solar_term.second
        );
    }

    // 将2022-3-10 11:5:3转换为农历
    #[test]
    fn test_convert_to_lunar_calendar2022_3_10_11_5_3() {
        // 测试公历转农历
        // 将2022-3-10 11:5:3转换为农历
        dotenv::dotenv().ok();
        let ephe_path = env::var("EPHE_PATH").expect(
            "没设置 EPHE_PATH 环境变量，可在.env
文件中设置或export EPHE_PATH=...",
        );
        let year = 2022;
        let month = 3;
        let day = 10;
        let hour = 11;
        let minute = 5;
        let second = 3;
        let data = lunar_calendar(year, month, day, hour, minute, second, &ephe_path);
        assert!(data.is_ok(), "{:?}", data);
        let data = data.unwrap();

        assert!(
            !data.is_lean_year,
            "{}-{}-{} {}:{}:{} 不是闰年",
            year, month, day, hour, minute, second
        );

        // 农历年，干支表示
        assert_eq!(
            data.lunar_year.to_string(),
            "壬寅",
            "{}-{}-{} {}:{}:{} 是壬寅，而非{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            data.lunar_year
        );

        // 农历月，以正月、二月、......、十月、冬月、腊月表示
        assert_eq!(
            data.lunar_month.to_string(),
            "二月",
            "{}-{}-{} {}:{}:{} 是二月，而非{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            data.lunar_month
        );

        //  农历日，以初一、初二、……、二十九、三十表示
        assert_eq!(
            data.lunar_day.to_string(),
            "初八",
            "{}-{}-{} {}:{}:{} 是初八，而非{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            data.lunar_day.to_string()
        );

        // 农历年干支，按节气换年

        assert_eq!(
            data.lunar_year_gan_zhi.to_string(),
            "壬寅",
            "{}-{}-{} {}:{}:{} 节气年干支是壬寅，而非{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            data.lunar_year_gan_zhi
        );

        // 农历月干支，按节气换月

        assert_eq!(
            data.lunar_month_gan_zhi.to_string(),
            "癸卯",
            "{}-{}-{} {}:{}:{} 月干支是癸卯，而非{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            data.lunar_month_gan_zhi
        );

        // 日干支
        assert_eq!(
            data.lunar_day_gan_zhi.to_string(),
            "壬戌",
            "{}-{}-{} {}:{}:{} 日干支是壬戌，而非{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            data.lunar_day_gan_zhi
        );

        // 时干支
        assert_eq!(
            data.time_gan_zhi.to_string(),
            "丙午",
            "{}-{}-{} {}:{}:{} 时干支是丙午，而非{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            data.time_gan_zhi
        );

        // 节
        let solar_term = data.solar_term_first;
        assert!(
            solar_term.name == "惊蛰"
                && solar_term.year == 2022
                && solar_term.month == 3
                && solar_term.day == 5
                && solar_term.hour == 22,
            "{}-{}-{} {}:{}:{} 的节是`小寒 2022-3-5 22:43:34`，而非{} {}-{}-{} {}:{}:{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            solar_term.name,
            solar_term.year,
            solar_term.month,
            solar_term.day,
            solar_term.hour,
            solar_term.minute,
            solar_term.second
        );

        // 中气

        let solar_term = data.solar_term_second;
        assert!(
            solar_term.name == "春分"
                && solar_term.year == 2022
                && solar_term.month == 3
                && solar_term.day == 20
                && solar_term.hour == 23,
            "{}-{}-{} {}:{}:{} 的节是`大寒 2022-3-20 23:33:15`，而非{} {}-{}-{} {}:{}:{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            solar_term.name,
            solar_term.year,
            solar_term.month,
            solar_term.day,
            solar_term.hour,
            solar_term.minute,
            solar_term.second
        )
    }

    // 将2020-6-10 11:5:3转换为农历，此年闰四月
    #[test]
    fn test_convert_to_lunar_calendar2020_6_10_11_5_3() {
        // 测试公历转农历
        // 将2020-6-10 11:5:3转换为农历，此年闰四月
        dotenv::dotenv().ok();
        let ephe_path = env::var("EPHE_PATH").expect(
            "没设置 EPHE_PATH 环境变量，可在.env
文件中设置或export EPHE_PATH=...",
        );
        let year = 2020;
        let month = 6;
        let day = 10;
        let hour = 11;
        let minute = 5;
        let second = 3;
        let data = lunar_calendar(year, month, day, hour, minute, second, &ephe_path);
        assert!(data.is_ok(), "{:?}", data);
        let data = data.unwrap();

        assert!(
            data.is_lean_year,
            "{}-{}-{} {}:{}:{} 是闰年",
            year, month, day, hour, minute, second
        );

        // 农历年，干支表示
        assert_eq!(
            data.lunar_year.to_string(),
            "庚子",
            "{}-{}-{} {}:{}:{} 是庚子，而非{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            data.lunar_year
        );

        // 农历月，以正月、二月、......、十月、冬月、腊月表示
        assert_eq!(
            data.lunar_month.to_string(),
            "闰四月",
            "{}-{}-{} {}:{}:{} 是闰四月，而非{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            data.lunar_month
        );

        //  农历日，以初一、初二、……、二十九、三十表示
        assert_eq!(
            data.lunar_day.to_string(),
            "十九",
            "{}-{}-{} {}:{}:{} 是十九，而非{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            data.lunar_day.to_string()
        );

        // 农历年干支，按节气换年

        assert_eq!(
            data.lunar_year_gan_zhi.to_string(),
            "庚子",
            "{}-{}-{} {}:{}:{} 节气年干支是庚子，而非{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            data.lunar_year_gan_zhi
        );

        // 农历月干支，按节气换月

        assert_eq!(
            data.lunar_month_gan_zhi.to_string(),
            "壬午",
            "{}-{}-{} {}:{}:{} 月干支是壬午，而非{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            data.lunar_month_gan_zhi
        );

        // 日干支
        assert_eq!(
            data.lunar_day_gan_zhi.to_string(),
            "甲申",
            "{}-{}-{} {}:{}:{} 日干支是甲申，而非{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            data.lunar_day_gan_zhi
        );

        // 时干支
        assert_eq!(
            data.time_gan_zhi.to_string(),
            "庚午",
            "{}-{}-{} {}:{}:{} 时干支是庚午，而非{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            data.time_gan_zhi
        );

        // 节
        let solar_term = data.solar_term_first;
        assert!(
            solar_term.name == "芒种"
                && solar_term.year == 2020
                && solar_term.month == 6
                && solar_term.day == 5
                && solar_term.hour == 12,
            "{}-{}-{} {}:{}:{} 的节是`芒种 2020-6-5 12:58:18`，而非{} {}-{}-{} {}:{}:{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            solar_term.name,
            solar_term.year,
            solar_term.month,
            solar_term.day,
            solar_term.hour,
            solar_term.minute,
            solar_term.second
        );

        // 中气

        let solar_term = data.solar_term_second;
        assert!(
            solar_term.name == "夏至"
                && solar_term.year == 2020
                && solar_term.month == 6
                && solar_term.day == 21
                && solar_term.hour == 5,
            "{}-{}-{} {}:{}:{} 的节是`夏至 2020-6-21 5:43:33`，而非{} {}-{}-{} {}:{}:{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            solar_term.name,
            solar_term.year,
            solar_term.month,
            solar_term.day,
            solar_term.hour,
            solar_term.minute,
            solar_term.second
        );
    }
}
