package lunarcalendar

import (
	"testing"
)

// 将2022-1-10 22:5:3转换为农历
func TestConvertToLunarCalendar2022_1_10_22_5_3(t *testing.T) {
	t.Log("测试公历转农历")
	t.Log("将2022-1-10 22:5:3转换为农历")
	year := 2022
	month := 1
	day := 10
	hour := 22
	minute := 5
	second := 3
	data, err := ConvertToLunarCalendar(year, month, day, hour, minute, second)
	if err != nil {
		t.Fatal(err)
	}

	if data.IsLeanYear {
		t.Fatalf("%v-%v-%v %v:%v:%v 不是闰年", year, month, day, hour, minute, second)
	}

	// 农历年，干支表示
	if s := data.LunarYear.Name(); s != "辛丑" {
		t.Fatalf("%v-%v-%v %v:%v:%v 是辛丑，而非%s", year, month, day, hour, minute, second, s)
	}

	// 农历月，以正月、二月、......、十月、冬月、腊月表示
	if s := data.LunarMonth; s != "腊月" {
		t.Log(s)
		t.Fatalf("%v-%v-%v %v:%v:%v 是腊月，而非%s", year, month, day, hour, minute, second, s)
	}

	//  农历日，以初一、初二、……、二十九、三十表示
	if s := data.LunarDay; s != "初八" {
		t.Fatalf("%v-%v-%v %v:%v:%v 是初八，而非%s", year, month, day, hour, minute, second, s)
	}

	// 农历年干支，按节气换年

	if s := data.LunarYearGanZhi.Name(); s != "辛丑" {
		t.Fatalf("%v-%v-%v %v:%v:%v 节气年干支是辛丑，而非%s", year, month, day, hour, minute, second, s)
	}

	// 农历月干支，按节气换月

	if s := data.LunarMonthGanZhi.Name(); s != "辛丑" {
		t.Fatalf("%v-%v-%v %v:%v:%v 月干支是辛丑，而非%s", year, month, day, hour, minute, second, s)
	}

	// 日干支
	if s := data.LunarDayGanZhi.Name(); s != "癸亥" {
		t.Fatalf("%v-%v-%v %v:%v:%v 日干支是癸亥，而非%s", year, month, day, hour, minute, second, s)
	}

	// 时干支
	if s := data.TimeGanZhi.Name(); s != "癸亥" {
		t.Fatalf("%v-%v-%v %v:%v:%v 时干支是癸亥，而非%s", year, month, day, hour, minute, second, s)
	}

	// 节
	solarTerm := data.SolarTermFirst
	if s := solarTerm.Name; s != "小寒" || solarTerm.Year != 2022 || solarTerm.Month != 1 || solarTerm.Day != 5 || solarTerm.Hour != 17 {
		t.Fatalf("%v-%v-%v %v:%v:%v 的节是`小寒 2022-1-5 17:13:54`，而非%s %v-%v-%v %v:%v:%v", year, month, day, hour, minute, second, s, solarTerm.Year, solarTerm.Month, solarTerm.Day, solarTerm.Hour, solarTerm.Minute, solarTerm.Second)
	}

	// 中气

	solarTerm = data.SolarTermSecond
	if s := solarTerm.Name; s != "大寒" || solarTerm.Year != 2022 || solarTerm.Month != 1 || solarTerm.Day != 20 || solarTerm.Hour != 10 {
		t.Fatalf("%v-%v-%v %v:%v:%v 的节是`大寒 2022-1-20 10:38:56`，而非%s %v-%v-%v %v:%v:%v", year, month, day, hour, minute, second, s, solarTerm.Year, solarTerm.Month, solarTerm.Day, solarTerm.Hour, solarTerm.Minute, solarTerm.Second)
	}
}

// 将2022-2-3 22:5:3转换为农历
func TestConvertToLunarCalendar2022_2_3_22_5_3(t *testing.T) {
	t.Log("测试公历转农历")
	t.Log("将2022-3-3 22:5:3转换为农历")
	year := 2022
	month := 2
	day := 3
	hour := 22
	minute := 5
	second := 3
	data, err := ConvertToLunarCalendar(year, month, day, hour, minute, second)
	if err != nil {
		t.Fatal(err)
	}

	if data.IsLeanYear {
		t.Fatalf("%v-%v-%v %v:%v:%v 不是闰年", year, month, day, hour, minute, second)
	}

	// 农历年，干支表示
	if s := data.LunarYear.Name(); s != "壬寅" {
		t.Fatalf("%v-%v-%v %v:%v:%v 是辛丑，而非%s", year, month, day, hour, minute, second, s)
	}

	// 农历月，以正月、二月、......、十月、冬月、腊月表示
	if s := data.LunarMonth; s != "正月" {
		t.Log(s)
		t.Fatalf("%v-%v-%v %v:%v:%v 是正月，而非%s", year, month, day, hour, minute, second, s)
	}

	//  农历日，以初一、初二、……、二十九、三十表示
	if s := data.LunarDay; s != "初三" {
		t.Fatalf("%v-%v-%v %v:%v:%v 是初三，而非%s", year, month, day, hour, minute, second, s)
	}

	// 农历年干支，按节气换年

	if s := data.LunarYearGanZhi.Name(); s != "辛丑" {
		t.Fatalf("%v-%v-%v %v:%v:%v 节气年干支是辛丑，而非%s", year, month, day, hour, minute, second, s)
	}

	// 农历月干支，按节气换月

	if s := data.LunarMonthGanZhi.Name(); s != "辛丑" {
		t.Fatalf("%v-%v-%v %v:%v:%v 月干支是辛丑，而非%s", year, month, day, hour, minute, second, s)
	}

	// 日干支
	if s := data.LunarDayGanZhi.Name(); s != "丁亥" {
		t.Fatalf("%v-%v-%v %v:%v:%v 日干支是丁亥，而非%s", year, month, day, hour, minute, second, s)
	}

	// 时干支
	if s := data.TimeGanZhi.Name(); s != "辛亥" {
		t.Fatalf("%v-%v-%v %v:%v:%v 时干支是辛亥，而非%s", year, month, day, hour, minute, second, s)
	}

	// 节
	solarTerm := data.SolarTermFirst
	if s := solarTerm.Name; s != "小寒" || solarTerm.Year != 2022 || solarTerm.Month != 1 || solarTerm.Day != 5 || solarTerm.Hour != 17 {
		t.Fatalf("%v-%v-%v %v:%v:%v 的节是`小寒 2022-1-5 17:13:54`，而非%s %v-%v-%v %v:%v:%v", year, month, day, hour, minute, second, s, solarTerm.Year, solarTerm.Month, solarTerm.Day, solarTerm.Hour, solarTerm.Minute, solarTerm.Second)
	}

	// 中气

	solarTerm = data.SolarTermSecond
	if s := solarTerm.Name; s != "大寒" || solarTerm.Year != 2022 || solarTerm.Month != 1 || solarTerm.Day != 20 || solarTerm.Hour != 10 {
		t.Fatalf("%v-%v-%v %v:%v:%v 的节是`大寒 2022-1-20 10:38:56`，而非%s %v-%v-%v %v:%v:%v", year, month, day, hour, minute, second, s, solarTerm.Year, solarTerm.Month, solarTerm.Day, solarTerm.Hour, solarTerm.Minute, solarTerm.Second)
	}
}

// 将2022-3-10 11:5:3转换为农历
func TestConvertToLunarCalendar2022_3_10_11_5_3(t *testing.T) {
	t.Log("测试公历转农历")
	t.Log("将2022-3-10 11:5:3转换为农历")
	year := 2022
	month := 3
	day := 10
	hour := 11
	minute := 5
	second := 3
	data, err := ConvertToLunarCalendar(year, month, day, hour, minute, second)
	if err != nil {
		t.Fatal(err)
	}

	if data.IsLeanYear {
		t.Fatalf("%v-%v-%v %v:%v:%v 不是闰年", year, month, day, hour, minute, second)
	}

	// 农历年，干支表示
	if s := data.LunarYear.Name(); s != "壬寅" {
		t.Fatalf("%v-%v-%v %v:%v:%v 是壬寅年，而非%s", year, month, day, hour, minute, second, s)
	}

	// 农历月，以正月、二月、......、十月、冬月、腊月表示
	if s := data.LunarMonth; s != "二月" {
		t.Log(s)
		t.Fatalf("%v-%v-%v %v:%v:%v 是二月，而非%s", year, month, day, hour, minute, second, s)
	}

	//  农历日，以初一、初二、……、二十九、三十表示
	if s := data.LunarDay; s != "初八" {
		t.Fatalf("%v-%v-%v %v:%v:%v 是初八，而非%s", year, month, day, hour, minute, second, s)
	}

	// 农历年干支，按节气换年

	if s := data.LunarYearGanZhi.Name(); s != "壬寅" {
		t.Fatalf("%v-%v-%v %v:%v:%v 节气年干支是壬寅，而非%s", year, month, day, hour, minute, second, s)
	}

	// 农历月干支，按节气换月

	if s := data.LunarMonthGanZhi.Name(); s != "癸卯" {
		t.Fatalf("%v-%v-%v %v:%v:%v 是癸卯月，而非%s", year, month, day, hour, minute, second, s)
	}

	// 日干支
	if s := data.LunarDayGanZhi.Name(); s != "壬戌" {
		t.Fatalf("%v-%v-%v %v:%v:%v 是壬戌日，而非%s", year, month, day, hour, minute, second, s)
	}

	// 时干支
	if s := data.TimeGanZhi.Name(); s != "丙午" {
		t.Fatalf("%v-%v-%v %v:%v:%v 丙午时，而非%s", year, month, day, hour, minute, second, s)
	}

	// 节
	solarTerm := data.SolarTermFirst
	if s := solarTerm.Name; s != "惊蛰" || solarTerm.Year != 2022 || solarTerm.Month != 3 || solarTerm.Day != 5 || solarTerm.Hour != 22 {
		t.Fatalf("%v-%v-%v %v:%v:%v 的节是`小寒 2022-3-5 22:43:34`，而非%s %v-%v-%v %v:%v:%v", year, month, day, hour, minute, second, s, solarTerm.Year, solarTerm.Month, solarTerm.Day, solarTerm.Hour, solarTerm.Minute, solarTerm.Second)
	}

	// 中气

	solarTerm = data.SolarTermSecond
	if s := solarTerm.Name; s != "春分" || solarTerm.Year != 2022 || solarTerm.Month != 3 || solarTerm.Day != 20 || solarTerm.Hour != 23 {
		t.Fatalf("%v-%v-%v %v:%v:%v 的节是`大寒 2022-3-20 23:33:15`，而非%s %v-%v-%v %v:%v:%v", year, month, day, hour, minute, second, s, solarTerm.Year, solarTerm.Month, solarTerm.Day, solarTerm.Hour, solarTerm.Minute, solarTerm.Second)
	}
}

// 将2020-6-10 11:5:3转换为农历，此年闰四月
func TestConvertToLunarCalendar2020_6_10_11_5_3(t *testing.T) {
	t.Log("测试公历转农历")
	t.Log("将2020-6-10 11:5:3转换为农历，此年闰四月")
	year := 2020
	month := 6
	day := 10
	hour := 11
	minute := 5
	second := 3
	data, err := ConvertToLunarCalendar(year, month, day, hour, minute, second)
	if err != nil {
		t.Fatal(err)
	}

	if !data.IsLeanYear {
		t.Fatalf("%v-%v-%v %v:%v:%v 是闰年", year, month, day, hour, minute, second)
	}

	// 农历年，干支表示
	if s := data.LunarYear.Name(); s != "庚子" {
		t.Fatalf("%v-%v-%v %v:%v:%v 是庚子年，而非%s", year, month, day, hour, minute, second, s)
	}

	// 农历月，以正月、二月、......、十月、冬月、腊月表示
	if s := data.LunarMonth; s != "闰四月" {
		t.Log(s)
		t.Fatalf("%v-%v-%v %v:%v:%v 是闰四月，而非%s", year, month, day, hour, minute, second, s)
	}

	//  农历日，以初一、初二、……、二十九、三十表示
	if s := data.LunarDay; s != "十九" {
		t.Fatalf("%v-%v-%v %v:%v:%v 是十九，而非%s", year, month, day, hour, minute, second, s)
	}

	// 农历年干支，按节气换年

	if s := data.LunarYearGanZhi.Name(); s != "庚子" {
		t.Fatalf("%v-%v-%v %v:%v:%v 节气年干支是庚子，而非%s", year, month, day, hour, minute, second, s)
	}

	// 农历月干支，按节气换月

	if s := data.LunarMonthGanZhi.Name(); s != "壬午" {
		t.Fatalf("%v-%v-%v %v:%v:%v 是壬午月，而非%s", year, month, day, hour, minute, second, s)
	}

	// 日干支
	if s := data.LunarDayGanZhi.Name(); s != "甲申" {
		t.Fatalf("%v-%v-%v %v:%v:%v 是甲申日，而非%s", year, month, day, hour, minute, second, s)
	}

	// 时干支
	if s := data.TimeGanZhi.Name(); s != "庚午" {
		t.Fatalf("%v-%v-%v %v:%v:%v 庚午时，而非%s", year, month, day, hour, minute, second, s)
	}

	// 节
	solarTerm := data.SolarTermFirst
	if s := solarTerm.Name; s != "芒种" || solarTerm.Year != 2020 || solarTerm.Month != 6 || solarTerm.Day != 5 || solarTerm.Hour != 12 {
		t.Fatalf("%v-%v-%v %v:%v:%v 的节是`芒种 2020-6-5 12:58:18`，而非%s %v-%v-%v %v:%v:%v", year, month, day, hour, minute, second, s, solarTerm.Year, solarTerm.Month, solarTerm.Day, solarTerm.Hour, solarTerm.Minute, solarTerm.Second)
	}

	// 中气

	solarTerm = data.SolarTermSecond
	if s := solarTerm.Name; s != "夏至" || solarTerm.Year != 2020 || solarTerm.Month != 6 || solarTerm.Day != 21 || solarTerm.Hour != 5 {
		t.Fatalf("%v-%v-%v %v:%v:%v 的节是`夏至 2020-6-21 5:43:33`，而非%s %v-%v-%v %v:%v:%v", year, month, day, hour, minute, second, s, solarTerm.Year, solarTerm.Month, solarTerm.Day, solarTerm.Hour, solarTerm.Minute, solarTerm.Second)
	}
}
