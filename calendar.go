// Package lunarcalendar
//公历日期转农历日期
package lunarcalendar

import (
	"fmt"
	"math"
	"os"
	"strings"

	"github.com/mshafiee/swephgo"
	"github.com/wlhyl/ganzhiwuxin"
)

/**
从公历日期得到农历日期
*/
func ConvertToLunarCalendar(year, month, day, hour, minute, second int) (LunarCalendar, error) {
	var lunarCalendar LunarCalendar

	ephePath := os.Getenv("EPHE_PATH")
	if ephePath == "" {
		return lunarCalendar, fmt.Errorf("EPHE_PATH must be specified")
	}

	if err := vaildDateTime(year, month, day, hour, minute, second); err != nil {
		return lunarCalendar, err
	}

	// 从前一年冬至到此年冬至间的25个节气，第25个节气=此年冬至
	solarTermJds, err := get25SolarTermJds(year-1, ephePath)
	if err != nil {
		return lunarCalendar, err
	}

	// 从前一年冬至所在农历开始的15个新月的jd
	newMoonJDs, err := get15NewMoonJDs(solarTermJds[0], ephePath)
	if err != nil {
		return lunarCalendar, err
	}

	// 从前一年冬至所在农历月开始的15个农历月的初一的儒略日，以东八区时间为准
	lunarMonths := get15LunarMonthJds(newMoonJDs)

	// 计算闰月，如果有闰月，修正月的num
	lunarMonths = calcLeapMonth(lunarMonths,
		func(jds [25]float64) [13]float64 {
			var j [13]float64
			for index, v := range jds {
				i := index % 2
				if i == 0 {
					j[index/2] = v
				}
			}
			return j
		}(solarTermJds))

	// 设置闰年
	func() {
		for _, m := range lunarMonths {
			if m.isLeap {
				lunarCalendar.IsLeanYear = true
				return
			}
		}
		lunarCalendar.IsLeanYear = false
	}()

	// 得到月名
	for i, month := range lunarMonths {
		if month.isLeap {
			lunarMonths[i].monthName = fmt.Sprintf("闰%s月", monthNames[month.num-1])
		} else {
			lunarMonths[i].monthName = fmt.Sprintf("%s月", monthNames[month.num-1])
		}
	}

	/*
	   将公历转换为农历
	   为方便计算，可以取20:00:00，utc此时为12:00:00
	   此处默认计算00:00:00
	*/
	// 计算农历月和农历日
	// 1582年10月15日00:00:00起为格里高利历
	var calendar int32 = swephgo.SeGregCal
	if year < 1582 {
		calendar = swephgo.SeJulCal
	}
	if year == 1582 && month < 10 {
		calendar = swephgo.SeJulCal
	}
	if year == 1582 && month == 10 && day < 15 {
		calendar = swephgo.SeJulCal
	}
	cyear := make([]int, 1)
	cmonth := make([]int, 1)
	cday := make([]int, 1)
	chour := make([]int, 1)
	cminute := make([]int, 1)
	csecond := make([]float64, 1)
	swephgo.UtcTimeZone(year, month, day, hour, minute, float64(second), 8.0,
		cyear, cmonth, cday, chour, cminute, csecond)
	var currentJd = swephgo.Julday(
		cyear[0], cmonth[0], cday[0],
		float64(chour[0])+float64(cminute[0])/60.0+csecond[0]/3600.0,
		calendar)

	// 找出当前日期所在农历月
	var n = 0
	for i := 0; i < len(lunarMonths); i++ {
		if lunarMonths[i].jd <= currentJd && currentJd < lunarMonths[i+1].jd {
			n = i
			break
		}
	}
	lunarCalendar.LunarMonth = lunarMonths[n].monthName
	var days = int(math.Floor(currentJd - lunarMonths[n].jd))
	lunarCalendar.LunarDay = dayNames[days]

	// 计算年
	// 根据2017年国标，农历年用干支表示
	// firstLunarMonth： 农历正月
	var firstLunarMonth = func() LunarMonth {
		for _, month := range lunarMonths {
			if month.num == 1 && !month.isLeap {
				return month
			}
		}
		return lunarMonths[0]
	}()

	甲, err := ganzhiwuxin.NewTianGan("甲")
	if err != nil {
		return lunarCalendar, err
	}
	子, err := ganzhiwuxin.NewDiZhi("子")
	if err != nil {
		return lunarCalendar, err
	}
	甲子, err := ganzhiwuxin.NewGanZhi(甲, 子)
	if err != nil {
		return lunarCalendar, err
	}

	// 计算农历年
	if currentJd < firstLunarMonth.jd {
		lunarCalendar.LunarYear = 甲子.Plus(year - 1 - 1864)
	} else {
		lunarCalendar.LunarYear = 甲子.Plus(year - 1864)
	}

	// 计算农历日干支
	// 计算日柱, 以2017年4月7日，甲子日为起点
	lunarCalendar.LunarDayGanZhi = func() ganzhiwuxin.GanZhi {
		d0 := currentJd - swephgo.Julday(2017, 4, 6, 16.0, swephgo.SeGregCal)
		d0 = math.Floor(d0)
		d := int(d0)
		return 甲子.Plus(d)
	}()

	// 年干支，以立春换年
	// solarTermJds[3]是立春
	if currentJd < solarTermJds[3] {
		lunarCalendar.LunarYearGanZhi = 甲子.Plus(year - 1 - 1864)
	} else {
		lunarCalendar.LunarYearGanZhi = 甲子.Plus(year - 1864)

	}

	// 计算月柱
	// 默认为00:00:00所在的月柱，立春换月柱
	// 大雪的黄经=255度
	// monthNum在计算节气时会用到
	var monthNum int
	lunarCalendar.LunarMonthGanZhi, err = func() (ganzhiwuxin.GanZhi, error) {
		swephgo.SetEphePath([]byte(ephePath))
		xx := make([]float64, 6)
		serr := make([]byte, 256)

		iflgret := swephgo.CalcUt(currentJd, swephgo.SeSun, swephgo.SeflgSwieph, xx, serr)
		swephgo.Close()
		serrString := strings.TrimRight(string(serr), string(byte(0)))
		if len(serrString) > 0 || iflgret < 0 {
			return ganzhiwuxin.GanZhi{}, fmt.Errorf("swe_calc_ut()错误。%s", serrString)
		}
		monthNum = int(math.Floor(swephgo.Degnorm(xx[0]-255) / 30))

		monthDiZhi := 子.Plus(monthNum)

		// 求月柱，按节气换年，不能使用农历正月初一换年，如果 2017年1月7日，节气年、农历年都是丙申，
		// 不能以monthNum < 2，将农历的丙申 - 1

		// this.lunarMonthGanZhi =
		yearGan := lunarCalendar.LunarYearGanZhi.Gan()
		// 寅 = 子 + 2
		寅 := 子.Plus(2)
		if yearGan.Equals(甲) || yearGan.Equals(甲.Plus(5)) {
			// 丙 = 甲 + 2
			n := monthDiZhi.Minus(寅)
			g := 甲.Plus(2).Plus(n)
			return ganzhiwuxin.NewGanZhi(g, monthDiZhi)

		}
		if yearGan.Equals(甲.Plus(1)) || yearGan.Equals(甲.Plus(6)) {
			// 戊 = 甲 + 4
			n := monthDiZhi.Minus(寅)
			g := 甲.Plus(4).Plus(n)
			return ganzhiwuxin.NewGanZhi(g, monthDiZhi)
		}
		if yearGan.Equals(甲.Plus(2)) || yearGan.Equals(甲.Plus(7)) {
			// 庚 = 甲 + 6
			n := monthDiZhi.Minus(寅)
			g := 甲.Plus(6).Plus(n)
			return ganzhiwuxin.NewGanZhi(g, monthDiZhi)
		}
		if yearGan.Equals(甲.Plus(3)) || yearGan.Equals(甲.Plus(8)) {
			// 壬 = 甲 + 8
			n := monthDiZhi.Minus(寅)
			g := 甲.Plus(8).Plus(n)
			return ganzhiwuxin.NewGanZhi(g, monthDiZhi)
		}

		// 甲 = 甲 + 0
		n := monthDiZhi.Minus(寅)
		g := 甲.Plus(n)
		return ganzhiwuxin.NewGanZhi(g, monthDiZhi)
	}()
	if err != nil {
		return lunarCalendar, err
	}

	// 计算时柱, (hour + 1) / 2 = 时辰数-1, 0点子时=1,丑时=2,辰时=3... 亥时=11,23点=12
	lunarCalendar.TimeGanZhi = func() ganzhiwuxin.GanZhi {
		n := (hour + 1) / 2

		丙子 := 甲子.Plus(12)
		戊子 := 丙子.Plus(12)
		庚子 := 戊子.Plus(12)
		壬子 := 庚子.Plus(12)

		dayGan := lunarCalendar.LunarDayGanZhi.Gan()

		if dayGan.Equals(甲) || dayGan.Equals(甲.Plus(5)) {
			return 甲子.Plus(n)
		}

		if dayGan.Equals(甲.Plus(1)) || dayGan.Equals(甲.Plus(6)) {
			return 丙子.Plus(n)
		}
		if dayGan.Equals(甲.Plus(2)) || dayGan.Equals(甲.Plus(7)) {
			return 戊子.Plus(n)
		}

		if dayGan.Equals(甲.Plus(3)) || dayGan.Equals(甲.Plus(8)) {
			return 庚子.Plus(n)
		}

		return 壬子.Plus(n)

	}()

	// 计算此日期所在的节气
	solarTermJd0, err := newtonIteration(currentJd, func(jd float64) (float64, error) {
		swephgo.SetEphePath([]byte(ephePath))
		xx := make([]float64, 6)
		serr := make([]byte, 256)

		iflgret := swephgo.CalcUt(jd, swephgo.SeSun, swephgo.SeflgSwieph, xx, serr)
		swephgo.Close()
		serrString := strings.TrimRight(string(serr), string(byte(0)))
		if len(serrString) > 0 || iflgret < 0 {
			return 0, fmt.Errorf("swe_calc_ut()错误。%s", serrString)
		}
		return mod180(xx[0] - swephgo.Degnorm(float64(monthNum*30+255.0))), nil
	})

	y8, m8, d8, h8, mi8, sec8 := getUT8DateTimeFromJd(solarTermJd0)
	lunarCalendar.SolarTermFirst = SolarTerm{
		SolarTermNames[monthNum*2],
		y8, m8, d8, h8, mi8, int(math.Floor(sec8)),
	}

	solarTermJd1, err := newtonIteration(solarTermJd0+15, func(jd float64) (float64, error) {
		swephgo.SetEphePath([]byte(ephePath))
		xx := make([]float64, 6)
		serr := make([]byte, 256)

		iflgret := swephgo.CalcUt(jd, swephgo.SeSun, swephgo.SeflgSwieph, xx, serr)
		swephgo.Close()
		serrString := strings.TrimRight(string(serr), string(byte(0)))
		if len(serrString) > 0 || iflgret < 0 {
			return 0, fmt.Errorf("swe_calc_ut()错误。%s", serrString)
		}
		return mod180(xx[0] - swephgo.Degnorm(float64(monthNum*30+255+15))), nil
	})

	y8, m8, d8, h8, mi8, sec8 = getUT8DateTimeFromJd(solarTermJd1)
	lunarCalendar.SolarTermSecond = SolarTerm{
		SolarTermNames[monthNum*2+1],
		y8, m8, d8, h8, mi8, int(math.Floor(sec8)),
	}
	return lunarCalendar, nil
}
