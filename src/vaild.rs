use swe::{swe_date_conversion, Calendar};

/**
检查时间是否合法
*/
pub fn vaild_date_time(
    year: i32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
) -> Result<(), String> {
    if year == 0 {
        return Err(format!("no {} year", year));
    }
    if month < 1 || month > 12 {
        return Err("mont muster > 0 and < 13".to_owned());
    }
    if day < 1 || day > 31 {
        return Err("days muster > 0 and < 31".to_owned());
    }

    if hour > 23 {
        return Err("hours muster > 0 and < 24".to_owned());
    }

    if minute > 59 {
        return Err("minutes muster > 0 and < 60".to_owned());
    }

    if second > 59 {
        return Err("second muster > 0 and < 60".to_owned());
    }

    if year == 1582 && month == 10 && day > 4 && day < 15 {
        return Err(format!(
            "{}-{}-{} {}:{}:{} 没有此日期",
            year, month, day, hour, minute, second
        ));
    }

    // 计算儒略日，并判断时间是否合法
    // 1582年10月15日00:00:00起为格里高利历
    let mut calendar = Calendar::Gregorian;
    if year < 1582 {
        calendar = Calendar::Julian
    }
    if year == 1582 && month < 10 {
        calendar = Calendar::Julian
    }
    if year == 1582 && month == 10 && day < 15 {
        calendar = Calendar::Julian
    }

    //这一步仅用作判断时间的正确性
    //假定此时间是格林尼治时间
    // swe_date_conversio通过swe_julday计算jd，
    // 再以swe_revjul反算，作比较，判断时间是否正确
    // 但swe_julday与swe_revjul会将闰秒视作下一秒，
    //即2016-12-31 23:59:60视作2017-1-1 00:00:00
    // 这两个函数使用的是日期与jd的转换公式，不能处理闰秒
    let dhour: f64 = hour as f64 + minute as f64 / 60.0 + second as f64 / 3600.0;
    let year = if year < 0 { year + 1 } else { year };
    if swe_date_conversion(year, month.into(), day.into(), dhour.into(), calendar).is_ok() {
        Ok(())
    } else {
        Err(format!(
            "{}-{}-{} {}:{}:{} 没有此日期",
            year, month, day, hour, minute, second
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::vaild_date_time;

    #[test]
    fn test_vaild_date_time() {
        // 正确日期时间
        assert!(vaild_date_time(2023, 1, 1, 0, 0, 0).is_ok());

        // 不正确年
        assert!(vaild_date_time(0, 1, 1, 0, 0, 0).is_err());
        // 不正确月
        assert!(vaild_date_time(2023, 13, 1, 0, 0, 0).is_err());
        // 不正确日
        assert!(vaild_date_time(2023, 1, 32, 0, 0, 0).is_err());
        // 不正确时
        assert!(vaild_date_time(2023, 1, 1, 24, 0, 0).is_err());
        //不正确分
        assert!(vaild_date_time(2023, 1, 1, 0, 60, 0).is_err());
        //不正确秒
        assert!(vaild_date_time(2023, 1, 1, 0, 0, 60).is_err());

        // 闰年
        assert!(vaild_date_time(2023, 2, 29, 0, 0, 0).is_err());

        //不存在的年份
        for day in 5..15 {
            assert!(vaild_date_time(1582, 10, day, 0, 0, 0).is_err());
        }
    }
}
