package lunarcalendar

import (
	"fmt"
	"github.com/mshafiee/swephgo"
)

/**
检查时间是否合法
*/
func vaildDateTime(year, month, day, hour, minute, second int) error {
	if year == 0 {
		return fmt.Errorf("no %v year", year)
	}
	if month < 1 || month > 12 {
		return fmt.Errorf("mont muster > 0 and < 13")
	}
	if day < 1 || day > 31 {
		return fmt.Errorf("days muster > 0 and < 31")
	}

	if hour < 0 || hour > 23 {
		return fmt.Errorf("hours muster > 0 and < 24")
	}

	if minute < 0 || minute > 59 {
		return fmt.Errorf("minutes muster > 0 and < 60")
	}

	if second < 0 || second > 59 {
		return fmt.Errorf("second muster > 0 and < 60")
	}

	if year == 1582 &&
		month == 10 &&
		day > 4 &&
		day < 15 {
		return fmt.Errorf("%v-%v-%v %v:%v:%v 没有此日期", year, month, day, hour, minute, second)
	}

	// 计算儒略日，并判断时间是否合法
	// 1582年10月15日00:00:00起为格里高利历
	var calendar byte = 'g'
	if year < 1582 {
		calendar = 'j'
	}
	if year == 1582 && month < 10 {
		calendar = 'j'
	}
	if year == 1582 && month == 10 && day < 15 {
		calendar = 'j'
	}

	//这一步仅用作判断时间的正确性
	//假定此时间是格林尼治时间
	// swe_date_conversio通过swe_julday计算jd，
	// 再以swe_revjul反算，作比较，判断时间是否正确
	// 但swe_julday与swe_revjul会将闰秒视作下一秒，
	//即2016-12-31 23:59:60视作2017-1-1 00:00:00
	// 这两个函数使用的是日期与jd的转换公式，不能处理闰秒
	dhour := float64(hour) + float64(minute)/60.0 + float64(second)/3600.0
	dyear := year
	if year < 0 {
		dyear = year
	}
	tjd := make([]float64, 1)
	if err := swephgo.DateConversion(dyear, month, day, dhour, calendar, tjd); err == swephgo.Err {
		return fmt.Errorf("%v-%v-%v %v:%v:%v 没有此日期", year, month, day, hour, minute, second)

	}
	return nil
}
