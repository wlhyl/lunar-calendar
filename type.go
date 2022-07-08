package lunarcalendar

import "github.com/wlhyl/ganzhiwuxin"

// 农历月信息
type LunarMonth struct {
	num       int
	jd        float64
	monthName string
	isLeap    bool
}

type LunarCalendar struct {

	/**
	 * 闰年:true
	 */
	IsLeanYear bool

	/**
	 * 农历年，干支表示
	 */
	LunarYear ganzhiwuxin.GanZhi

	/**
	 * 农历月，以正月、二月、......、十月、冬月、腊月表示
	 */
	LunarMonth string

	/**
	 * 农历日，以初一、初二、……、二十九、三十表示
	 */
	LunarDay string

	/**
	 * 农历年干支，按节气换年
	 */
	LunarYearGanZhi ganzhiwuxin.GanZhi

	/**
	 * 农历月干支，按节气换月
	 */
	LunarMonthGanZhi ganzhiwuxin.GanZhi

	/**
	 * 日干支
	 */
	LunarDayGanZhi ganzhiwuxin.GanZhi

	/**
	 * 时干支
	 */
	TimeGanZhi ganzhiwuxin.GanZhi

	/**
	 * 节
	 */
	SolarTermFirst SolarTerm

	/**
	 * 中气
	 */
	SolarTermSecond SolarTerm
}

// 节气
type SolarTerm struct {
	Name   string
	Year   int
	Month  int
	Day    int
	Hour   int
	Minute int
	Second int
}
