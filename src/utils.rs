use swe::{
    swe_calc_ut, swe_close, swe_degnorm, swe_julday, swe_revjul, swe_set_ephe_path,
    swe_utc_time_zone, Calendar, Planet,
};

use crate::{
    mathutl::{mod180, newton_iteration},
    typedef::LunarMonth,
};

/**
 * 计算某一年冬至开始的连续25个节气
 * 第25个节气=下一年冬至
 * @param year
 * 冬至点所在年份
 */
pub fn get25_solar_term_jds(year: i32, ephe_path: &str) -> Result<[f64; 25], String> {
    let mut jds = [0.0; 25];

    // 计算冬至点太阳位置的函数
    // 冬至点太阳黄道经度:270
    //此函数实际是 y=sun位置(jd)-270

    // 计算此年冬至点jd
    let jd = swe_julday(year, 12, 20, 0.0, Calendar::Gregorian);
    jds[0] = sun_long_to_jd(jd, 270.0, ephe_path)?;

    //计算从此年冬至到下一年冬至的25个节所的jd(utc) ,第25个节气=下一年冬至
    for i in 1..25 {
        // 每个节气大约差15天，因此将前一节气的jd + 15作为迭代初值，jds[i-1] + 15
        let mut angle = 270 + i * 15;
        if angle >= 360 {
            angle -= 360;
        }
        jds[i] = sun_long_to_jd(jds[i - 1] + 15.0, angle as f64, ephe_path)?;
    }

    Ok(jds)
}

/**
 * 求给定太阳黄道经度的儒略日
 * x: 初值
 */
fn sun_long_to_jd(x: f64, angle: f64, ephe_path: &str) -> Result<f64, String> {
    let f = |jd: f64| {
        swe_set_ephe_path(ephe_path);

        let xx: Result<[f64; 6], String> = swe_calc_ut(jd, Planet::SUN, Default::default());
        swe_close();

        match xx {
            Ok(xx) => Ok(mod180(xx[0] - angle)),
            Err(s) => Err(s),
        }
    };
    newton_iteration(x, f)
}

/**
 * 计算从某一年冬至开始的连续15个新月
 * @param jd
 * 冬至点的儒略日
 */
pub fn get15_new_moon_jds(jd: f64, ephe_path: &str) -> Result<[f64; 15], String> {
    let mut moon_jds = [0.0; 15];

    // 如果冬至点在满月之后会得到下一个合朔
    let mut shuo_dong_zhi_jd = get_new_moon_jd(jd, ephe_path)?;

    if shuo_dong_zhi_jd > jd {
        shuo_dong_zhi_jd = get_new_moon_jd(jd - 29.53, ephe_path)?;
    }
    moon_jds[0] = shuo_dong_zhi_jd;
    for i in 1..15 {
        moon_jds[i] = get_new_moon_jd(moon_jds[i - 1] + 29.53, ephe_path)?;
    }
    Ok(moon_jds)
}

/**
 * 计算给定jd所在农历月，日月合朔的jd
 * 如果jd在满月之后，迭代值为下一个合朔
 */
fn get_new_moon_jd(jd: f64, ephe_path: &str) -> Result<f64, String> {
    let f = |jd| {
        swe_set_ephe_path(ephe_path);

        //计算太阳黄道经度

        // let xx: Result<[f64; 6], String> = swe_calc_ut(jd, Planet::SUN, Default::default());

        let sun_posi = match swe_calc_ut(jd, Planet::SUN, Default::default()) {
            Ok(xx) => xx[0],
            Err(s) => {
                swe_close();
                return Err(s);
            }
        };

        // 计算月亮黄道经度
        let moon_posi = match swe_calc_ut(jd, Planet::MOON, Default::default()) {
            Ok(xx) => xx[0],
            Err(s) => {
                swe_close();
                return Err(s);
            }
        };

        swe_close();

        Ok(mod180(swe_degnorm(moon_posi - sun_posi)))
    };
    newton_iteration(jd, f)
}

/**
 * 计算从某年冬至开始连续15个农历月初一的儒略日
 * @param jds
 * 从冬至点所在月份开始，连续15个新月的儒略日
 */
pub fn get15_lunar_month_jds(jds: [f64; 15]) -> [LunarMonth; 15] {
    let mut first_day_jds: [LunarMonth; 15] = Default::default();

    for (index, jd) in jds.iter().enumerate() {
        let (y, m, d, hour): (i32, i32, i32, f64) = swe_revjul(*jd, Calendar::Gregorian);
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
        let mut n = (index + 11) % 12;
        if n == 0 {
            n = 12;
        }

        first_day_jds[index].num = n as u8;
        first_day_jds[index].jd = jd;
    }

    first_day_jds
}

/**
 * 根据节气计算是否有闰月
 * @param lunarMonth
 * 从前一年冬至开始的15个农历月的信息
 * @param jdsMiddleSolarTerm
 * 从前一年冬至开始的中气的儒略日,最后一中气是此年的冬至
 * 前一年冬至点所在农历月计为m_0
 * 此年冬至点所在农历月之前的一个农历月计为m_1
 * 从m_0数到m_1，如果有13个农历月，则置闰
 */
pub fn calc_leap_month(
    lunar_month: [LunarMonth; 15],
    jds_middle_solar_term: [f64; 13],
) -> [LunarMonth; 15] {
    // 找出区间[m_0, m_1]间的农历月
    // 只计数[m_0, m_1)之间的月数，
    // 此月数等于13，则置闰

    let n = lunar_month
        .iter()
        .filter(|x| x.jd <= jds_middle_solar_term[12])
        .count()
        - 1;

    if n == 12 {
        return lunar_month;
    }
    let mut lunar_month = lunar_month;
    for i in 0..lunar_month.len() - 1 {
        // 月中有中气:true，无中气:false
        let mut middle_solar_term = false;
        // len(jdsMiddleSolarTerm) - 1是因为排除今年的冬至点
        // jdsMiddleSolarTerm 的最后一个值即是今年的冬至点
        for j in 0..jds_middle_solar_term.len() - 1 {
            if lunar_month[i].jd < jds_middle_solar_term[j]
                && jds_middle_solar_term[j] < lunar_month[i + 1].jd
            {
                middle_solar_term = true;
                break;
            }
        }
        if !middle_solar_term {
            lunar_month[i].is_leap = true;
            for j in i..lunar_month.len() {
                lunar_month[j].num -= 1;
                if lunar_month[j].num == 0 {
                    lunar_month[j].num = 12;
                }
            }
            break;
        }
    }
    lunar_month
}
