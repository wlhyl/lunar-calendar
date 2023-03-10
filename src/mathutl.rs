use swe::{swe_revjul, swe_utc_time_zone, Calendar};

/**
 * ModPi 把角度限制在[-180, 180]之间
 */
pub fn mod180(r0: f64) -> f64 {
    let mut r = r0;
    while r < -180.0 {
        r += 360.0;
    }
    while r > 180.0 {
        r -= 360.0;
    }
    r
}
// 从儒略日得到东8区的日期
pub fn get_ut8_date_time_from_jd(jd: f64) -> (i32, u8, u8, u8, u8, f64) {
    let mut gregflag = Calendar::Gregorian;
    if jd < 2299160.5 {
        gregflag = Calendar::Julian;
    }

    let (y, m, d, hour) = swe_revjul(jd, gregflag);

    let h = hour.floor() as i32;
    let mi = ((hour - h as f64) * 60.0).floor() as i32;
    let sec = ((hour - h as f64) * 60.0 - mi as f64) * 60.0;

    // 将新月的jd换算到东八区
    let (y8, m8, d8, h8, mi8, sec8) = swe_utc_time_zone(y, m, d, h, mi, sec, -8.0);

    (
        y8,
        m8.try_into().unwrap(),
        d8.try_into().unwrap(),
        h8.try_into().unwrap(),
        mi8.try_into().unwrap(),
        sec8,
    )
}
/**
 * NewtonIteration 牛顿迭代法求解方程的根
 */
pub fn newton_iteration<F>(init_value: f64, f: F) -> Result<f64, String>
where
    F: Fn(f64) -> Result<f64, String>,
{
    let epsilon = 1e-7;
    let delta = 5e-6;
    let mut x = 0.0;
    let mut x0 = init_value;

    for _i in 0..1000 {
        x = x0;
        let fx = f(x)?;

        // 导数
        let fx_delta = f(x + delta)?;

        let fpx = (fx_delta - fx) / delta;
        x0 = x - fx / fpx;
        if (x0 - x).abs() <= epsilon {
            break;
        }
    }
    if (x0 - x).abs() <= epsilon {
        Ok(x)
    } else {
        Err("1000次迭代，求解失败，调整初值重试".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::{get_ut8_date_time_from_jd, mod180, newton_iteration};

    #[test]
    fn test_mod180() {
        let expected = 186.1 - 360.0;
        let d = 186.1;
        let actual = mod180(d);
        // 精度为1秒
        assert!(
            ((expected - actual) * 3600.0).abs() < 1.0,
            "mod180({})={}, 而非{}",
            d,
            expected,
            actual
        );

        let expected = -186.1 + 360.0;
        let d = -186.1;
        let actual = mod180(d);
        assert!(
            ((expected - actual) * 3600.0).abs() < 1.0,
            "mod180({})={}, 而非{}",
            d,
            expected,
            actual
        );

        let expected = 186.0 - 360.0;
        let d = 186.0 + 360.0 * 2.0;
        let actual = mod180(d);
        assert!(
            ((expected - actual) * 3600.0).abs() < 1.0,
            "mod180({})={}, 而非{}",
            d,
            expected,
            actual
        );

        let expected = -186.0 + 360.0;
        let d = -186.0 - 360.0 * 2.0;
        let actual = mod180(d);
        assert!(
            ((expected - actual) * 3600.0).abs() < 1.0,
            "mod180({})={}, 而非{}",
            d,
            expected,
            actual
        );
    }

    #[test]
    fn test_get_ut8_date_time_from_jd() {
        // 2459760.5: 2022年6月30日 0:0:0
        //jd: 2022-6-30 22:31:48
        let jd = 2459760.5 + (22.0 - 8.0 + 31.0 / 60.0 + 48.0 / 3600.0) / 24.0;
        let (y, m, d, h, mi, sec) = get_ut8_date_time_from_jd(jd);
        assert!(
            y == 2022 && m == 6 && d == 30 && h == 22 && mi == 31 && (sec - 48.0) < 1.0,
            "从2022-6-30 22:31:48的儒略日得到{}-{}-{} {}:{}:{}",
            y,
            m,
            d,
            h,
            mi,
            sec
        );
    }
    // 测试牛顿迭代
    #[test]
    fn test_newton_iteration() {
        // fx=x^2+x-1 在[0,1]上的根
        let fx = |x| Ok(x * x + x - 1.0);

        let y = newton_iteration(0.0, fx);
        assert!(y.is_ok());

        let x = (-1.0 + 5.0_f64.sqrt()) / 2.0;
        let epsilon = 1e-7;
        let d = (x - y.unwrap()).abs();
        assert!(
            d < epsilon,
            "fx=x^2+x-1 在[0,1]上的根求解失败, 误差{}>={}",
            d,
            epsilon
        );
    }
}
