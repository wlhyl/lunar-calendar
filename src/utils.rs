use swe::{swe_calc_ut, swe_close, swe_degnorm, swe_julday, swe_set_ephe_path, Body, Calendar};

use crate::mathutl::{mod180, newton_iteration};

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

        let xx: Result<[f64; 6], String> = swe_calc_ut(jd, &Body::SeSun, Default::default());
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

        let sun_posi = match swe_calc_ut(jd, &Body::SeSun, Default::default()) {
            Ok(xx) => xx[0],
            Err(s) => {
                swe_close();
                return Err(s);
            }
        };

        // 计算月亮黄道经度
        let moon_posi = match swe_calc_ut(jd, &Body::SeMoon, Default::default()) {
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
