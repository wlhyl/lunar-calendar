mod lunar_calendar;
mod lunar_day;
mod lunar_month;
mod mathutl;
mod solar_term;
mod utils;
mod vaild;

use ganzhiwuxing::{
    DiZhi::*,
    GanZhi::{self, *},
    TianGan::*,
};
use lunar_day::{LunarDay, DAY_NAMES};
use lunar_month::LunarMonth;
use mathutl::{get_ut8_date_time_from_jd, mod180, newton_iteration};
use solar_term::SolarTerm;
use swe::{
    swe_calc_ut, swe_close, swe_degnorm, swe_julday, swe_revjul, swe_set_ephe_path,
    swe_utc_time_zone, Body, Calendar,
};

pub use lunar_calendar::LunarCalendar;
use utils::{get15_new_moon_jds, get25_solar_term_jds};
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
        lunar_month: LunarMonth::正(false, 0.0),
        lunar_day: LunarDay::初一,
        lunar_year_gan_zhi: 甲子,
        lunar_month_gan_zhi: 甲子,
        lunar_day_gan_zhi: 甲子,
        time_gan_zhi: 甲子,
        solar_term_first: SolarTerm::冬至(0, 0, 0, 0, 0, 0),
        solar_term_second: SolarTerm::冬至(0, 0, 0, 0, 0, 0),
    };

    if ephe_path == "" {
        return Err("ephe_path must be specified".to_owned());
    }
    vaild_date_time(year, month, day, hour, minute, second)?;

    // 从前一年冬至到此年冬至间的25个节气，第25个节气=此年冬至
    let solar_term_jds = get25_solar_term_jds(year - 1, ephe_path)?;

    // 从前一年冬至所在农历开始的15个新月的jd
    let new_moon_jds = get15_new_moon_jds(solar_term_jds[0], ephe_path)?;

    // 从前一年冬至所在农历月开始的15个农历月的初一的儒略日及月名，以东八区时间为准
    // 共15个元素
    // let lunar_months = get15_lunar_month_jds(new_moon_jds);
    let lunar_months: Vec<_> = new_moon_jds
        .iter()
        .enumerate()
        .map(|(index, &jd)| {
            let (y, m, d, hour): (i32, i32, i32, f64) = swe_revjul(jd, Calendar::Gregorian);
            let h = hour.floor() as i32;
            let mi = ((hour - h as f64) * 60.0).floor() as i32;
            let sec = ((hour - h as f64) * 60.0 - mi as f64) * 60.0;

            // 将新月的jd换算到东八区
            let (y8, m8, d8, _h8, _mi8, _sec8) = swe_utc_time_zone(y, m, d, h, mi, sec, -8.0);

            // 以新月当天00:00:00为初一，计算儒略日
            let (y8, m8, d8, h8, mi8, sec8) = swe_utc_time_zone(y8, m8, d8, 0, 0, 0.0, 8.0);

            let jd = swe_julday(
                y8,
                m8,
                d8,
                h8 as f64 + mi8 as f64 / 60.0 + sec8 / 3600.0,
                Calendar::Gregorian,
            );

            // let mut n = (index + 11) % 12;
            // if n == 0 {
            //     n = 12;
            // }
            let n = (index + 11) % 12;
            // 如果n==0，则对应腊月

            match n {
                1 => LunarMonth::正(false, jd),
                2 => LunarMonth::二(false, jd),
                3 => LunarMonth::三(false, jd),
                4 => LunarMonth::四(false, jd),
                5 => LunarMonth::五(false, jd),
                6 => LunarMonth::六(false, jd),
                7 => LunarMonth::七(false, jd),
                8 => LunarMonth::八(false, jd),
                9 => LunarMonth::九(false, jd),
                10 => LunarMonth::十(false, jd),
                11 => LunarMonth::冬(false, jd),
                _ => LunarMonth::腊(false, jd),
            }

            // LunarMonth { jd, month_name }
            // first_day_jds[index].num = n as u8;
            // first_day_jds[index].jd = jd;
        })
        .collect();

    // 计算闰月，如果有闰月，修正月名

    // 从前一年冬至开始的中气的儒略日,最后一中气是此年的冬至
    // 共13个元素
    let jds_middle_solar_term: Vec<_> = solar_term_jds
        .iter()
        .enumerate()
        .filter(|(index, _)| index % 2 == 0)
        .map(|(_, &e)| e)
        .collect();
    // let jds_middle_solar_term: [f64; 13] = jds_middle_solar_term.try_into().unwrap();

    // 找出区间[m_0, m_1]间的农历月
    // 只计数[m_0, m_1)之间的月数，
    // 此月数等于13，则置闰
    // m_0: 前一年11月冬至jd，m_1:今年11月冬至日jd

    let lunar_mont_count = lunar_months
        .iter()
        .filter(|x| x.jd() <= jds_middle_solar_term[12])
        .count()
        - 1;

    // 找出第一个没有中气的农历月
    let first_month_without_middle_solar_term_index = lunar_months[..lunar_months.len() - 1]
        .iter()
        .enumerate()
        .find_map(|(index, _)| {
            // 月中有中气:true，无中气:false
            let mut middle_solar_term = false;
            // len(jdsMiddleSolarTerm) - 1是因为排除今年的冬至点
            // jdsMiddleSolarTerm 的最后一个值即是今年的冬至点
            for j in 0..jds_middle_solar_term.len() - 1 {
                if lunar_months[index].jd() < jds_middle_solar_term[j]
                    && jds_middle_solar_term[j] < lunar_months[index + 1].jd()
                {
                    middle_solar_term = true;
                    break;
                }
            }

            if middle_solar_term {
                None
            } else {
                Some(index)
            }
        });

    // 置闰
    let lunar_months = if lunar_mont_count != 12 {
        let first_month_without_middle_solar_term_index =
            first_month_without_middle_solar_term_index.unwrap();
        lunar_months
            .iter()
            .enumerate()
            .map(|(index, &lunar_month)| {
                if first_month_without_middle_solar_term_index == index {
                    lunar_month.to_pre_leap_month()
                } else if first_month_without_middle_solar_term_index < index {
                    lunar_month.to_pre_month()
                } else {
                    lunar_month
                }
            })
            .collect()
    } else {
        lunar_months
    };

    // 设置闰年

    if lunar_months.iter().find(|month| month.is_leap()).is_some() {
        lunar_calendar.is_lean_year = true;
    }

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
        if lunar_months[i].jd() <= current_jd && current_jd < lunar_months[i + 1].jd() {
            n = i;
            break;
        }
    }
    lunar_calendar.lunar_month = lunar_months[n];
    let days = (current_jd - lunar_months[n].jd()).floor() as usize;
    lunar_calendar.lunar_day = DAY_NAMES[days];

    // 计算年
    // 根据2017年国标，农历年用干支表示
    // firstLunarMonth： 农历正月
    let first_lunar_month = if let Some(month) = lunar_months
        .iter()
        .find(|month| month.to_num() == 1 && !month.is_leap())
    {
        month.clone()
    } else {
        lunar_months[0].clone()
    };

    // 计算农历年
    if current_jd < first_lunar_month.jd() {
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
    lunar_calendar.solar_term_first = match month_num * 2 {
        0 => SolarTerm::大雪(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        1 => SolarTerm::冬至(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        2 => SolarTerm::小寒(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        3 => SolarTerm::大寒(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        4 => SolarTerm::立春(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        5 => SolarTerm::雨水(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        6 => SolarTerm::惊蛰(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        7 => SolarTerm::春分(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        8 => SolarTerm::清明(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        9 => SolarTerm::谷雨(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        10 => SolarTerm::立夏(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        11 => SolarTerm::小满(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        12 => SolarTerm::芒种(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        13 => SolarTerm::夏至(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        14 => SolarTerm::小暑(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        15 => SolarTerm::大暑(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        16 => SolarTerm::立秋(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        17 => SolarTerm::处暑(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        18 => SolarTerm::白露(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        19 => SolarTerm::秋分(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        20 => SolarTerm::寒露(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        21 => SolarTerm::霜降(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        22 => SolarTerm::立冬(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        _ => SolarTerm::小雪(y8, m8, d8, h8, mi8, sec8.floor() as u8),
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
    lunar_calendar.solar_term_second = match month_num * 2 + 1 {
        0 => SolarTerm::大雪(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        1 => SolarTerm::冬至(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        2 => SolarTerm::小寒(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        3 => SolarTerm::大寒(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        4 => SolarTerm::立春(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        5 => SolarTerm::雨水(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        6 => SolarTerm::惊蛰(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        7 => SolarTerm::春分(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        8 => SolarTerm::清明(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        9 => SolarTerm::谷雨(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        10 => SolarTerm::立夏(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        11 => SolarTerm::小满(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        12 => SolarTerm::芒种(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        13 => SolarTerm::夏至(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        14 => SolarTerm::小暑(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        15 => SolarTerm::大暑(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        16 => SolarTerm::立秋(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        17 => SolarTerm::处暑(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        18 => SolarTerm::白露(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        19 => SolarTerm::秋分(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        20 => SolarTerm::寒露(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        21 => SolarTerm::霜降(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        22 => SolarTerm::立冬(y8, m8, d8, h8, mi8, sec8.floor() as u8),
        _ => SolarTerm::小雪(y8, m8, d8, h8, mi8, sec8.floor() as u8),
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
            solar_term.name() == "小寒"
                && solar_term.year() == 2022
                && solar_term.month() == 1
                && solar_term.day() == 5
                && solar_term.hour() == 17,
            "{}-{}-{} {}:{}:{} 的节是`小寒 2022-1-5 17:13:54`，而非{} {}-{}-{} {}:{}:{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            solar_term.name(),
            solar_term.year(),
            solar_term.month(),
            solar_term.day(),
            solar_term.hour(),
            solar_term.minute(),
            solar_term.second()
        );

        // 中气

        let solar_term = data.solar_term_second;
        assert!(
            solar_term.name() == "大寒"
                && solar_term.year() == 2022
                && solar_term.month() == 1
                && solar_term.day() == 20
                && solar_term.hour() == 10,
            "{}-{}-{} {}:{}:{} 的节是`大寒 2022-1-20 10:38:56`，而非{} {}-{}-{} {}:{}:{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            solar_term.name(),
            solar_term.year(),
            solar_term.month(),
            solar_term.day(),
            solar_term.hour(),
            solar_term.minute(),
            solar_term.second()
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
            solar_term.name() == "小寒"
                && solar_term.year() == 2022
                && solar_term.month() == 1
                && solar_term.day() == 5
                && solar_term.hour() == 17,
            "{}-{}-{} {}:{}:{} 的节是`小寒 2022-1-5 17:13:54`，而非{} {}-{}-{} {}:{}:{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            solar_term.name(),
            solar_term.year(),
            solar_term.month(),
            solar_term.day(),
            solar_term.hour(),
            solar_term.minute(),
            solar_term.second()
        );

        // 中气
        let solar_term = data.solar_term_second;
        assert!(
            solar_term.name() == "大寒"
                && solar_term.year() == 2022
                && solar_term.month() == 1
                && solar_term.day() == 20
                && solar_term.hour() == 10,
            "{}-{}-{} {}:{}:{} 的节是`大寒 2022-1-20 10:38:56`，而非{} {}-{}-{} {}:{}:{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            solar_term.name(),
            solar_term.year(),
            solar_term.month(),
            solar_term.day(),
            solar_term.hour(),
            solar_term.minute(),
            solar_term.second()
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
            solar_term.name() == "惊蛰"
                && solar_term.year() == 2022
                && solar_term.month() == 3
                && solar_term.day() == 5
                && solar_term.hour() == 22,
            "{}-{}-{} {}:{}:{} 的节是`小寒 2022-3-5 22:43:34`，而非{} {}-{}-{} {}:{}:{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            solar_term.name(),
            solar_term.year(),
            solar_term.month(),
            solar_term.day(),
            solar_term.hour(),
            solar_term.minute(),
            solar_term.second()
        );

        // 中气

        let solar_term = data.solar_term_second;
        assert!(
            solar_term.name() == "春分"
                && solar_term.year() == 2022
                && solar_term.month() == 3
                && solar_term.day() == 20
                && solar_term.hour() == 23,
            "{}-{}-{} {}:{}:{} 的节是`大寒 2022-3-20 23:33:15`，而非{} {}-{}-{} {}:{}:{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            solar_term.name(),
            solar_term.year(),
            solar_term.month(),
            solar_term.day(),
            solar_term.hour(),
            solar_term.minute(),
            solar_term.second()
        )
    }

    // 将2020-6-10 11:5:3转换为农历，此年闰四月
    #[test]
    fn test_convert_to_lunar_calendar2020_6_10_11_5_3() {
        // 测试公历转农历
        // 将2020-6-10 11:5:3转换为农历，此年闰四月
        dotenv::dotenv().ok();
        let ephe_path = env::var("EPHE_PATH")
            .expect("没设置 EPHE_PATH 环境变量，可在.env文件中设置或export EPHE_PATH=...");
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
            solar_term.name() == "芒种"
                && solar_term.year() == 2020
                && solar_term.month() == 6
                && solar_term.day() == 5
                && solar_term.hour() == 12,
            "{}-{}-{} {}:{}:{} 的节是`芒种 2020-6-5 12:58:18`，而非{} {}-{}-{} {}:{}:{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            solar_term.name(),
            solar_term.year(),
            solar_term.month(),
            solar_term.day(),
            solar_term.hour(),
            solar_term.minute(),
            solar_term.second()
        );

        // 中气

        let solar_term = data.solar_term_second;
        assert!(
            solar_term.name() == "夏至"
                && solar_term.year() == 2020
                && solar_term.month() == 6
                && solar_term.day() == 21
                && solar_term.hour() == 5,
            "{}-{}-{} {}:{}:{} 的节是`夏至 2020-6-21 5:43:33`，而非{} {}-{}-{} {}:{}:{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            solar_term.name(),
            solar_term.year(),
            solar_term.month(),
            solar_term.day(),
            solar_term.hour(),
            solar_term.minute(),
            solar_term.second()
        );
    }

    // 将2020-7-3 16:0:0转换为农历，此年闰四月，此日是闰月后的五月
    #[test]
    fn test_convert_to_lunar_calendar2020_7_3_16_0_0() {
        // 测试公历转农历
        // 将2020-7-3 16:0:0转换为农历，此年闰四月，此日是：农历五月13
        dotenv::dotenv().ok();
        let ephe_path = env::var("EPHE_PATH")
            .expect("没设置 EPHE_PATH 环境变量，可在.env文件中设置或export EPHE_PATH=...");
        let year = 2020;
        let month = 7;
        let day = 3;
        let hour = 16;
        let minute = 0;
        let second = 0;
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
            "五月",
            "{}-{}-{} {}:{}:{} 五月，而非{}",
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
            "十三",
            "{}-{}-{} {}:{}:{} 是十三，而非{}",
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
            "丁未",
            "{}-{}-{} {}:{}:{} 日干支是丁未，而非{}",
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
            "戊申",
            "{}-{}-{} {}:{}:{} 时干支是戊申，而非{}",
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
            solar_term.name() == "芒种"
                && solar_term.year() == 2020
                && solar_term.month() == 6
                && solar_term.day() == 5
                && solar_term.hour() == 12,
            "{}-{}-{} {}:{}:{} 的节是`芒种 2020-6-5 12:58:18`，而非{} {}-{}-{} {}:{}:{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            solar_term.name(),
            solar_term.year(),
            solar_term.month(),
            solar_term.day(),
            solar_term.hour(),
            solar_term.minute(),
            solar_term.second()
        );

        // 中气

        let solar_term = data.solar_term_second;
        assert!(
            solar_term.name() == "夏至"
                && solar_term.year() == 2020
                && solar_term.month() == 6
                && solar_term.day() == 21
                && solar_term.hour() == 5,
            "{}-{}-{} {}:{}:{} 的节是`夏至 2020-6-21 5:43:33`，而非{} {}-{}-{} {}:{}:{}",
            year,
            month,
            day,
            hour,
            minute,
            second,
            solar_term.name(),
            solar_term.year(),
            solar_term.month(),
            solar_term.day(),
            solar_term.hour(),
            solar_term.minute(),
            solar_term.second()
        );
    }
}
