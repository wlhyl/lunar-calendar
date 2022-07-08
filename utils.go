package lunarcalendar

import (
	"fmt"
	"math"
	"strings"

	"github.com/mshafiee/swephgo"
)

/**
 * 计算某一年冬至开始的连续25个节气
 * 第25个节气=下一年冬至
 * @param year
 * 冬至点所在年份
 */
func get25SolarTermJds(year int, ephePath string) ([25]float64, error) {
	var jds [25]float64

	// 计算冬至点太阳位置的函数
	// 冬至点太阳黄道经度:270
	//此函数实际是 y=sun位置(jd)-270
	// sunPoint := func(jd float64) (float64, error) {
	// 	swephgo.SetEphePath([]byte(ephePath))
	// 	xx := make([]float64, 6)
	// 	serr := make([]byte, 256)
	// 	iflgret := swephgo.CalcUt(jd, swephgo.SeSun, swephgo.SeflgSwieph, xx, serr)
	// 	swephgo.Close()

	// 	serrString := strings.TrimRight(string(serr), string(byte(0)))
	// 	if len(serrString) > 0 || iflgret < 0 {
	// 		return 0, fmt.Errorf("swe_calc_ut()错误。%s", serrString)
	// 	}

	// 	return mod180(xx[0] - 270), nil
	// }

	fx := func(x float64, angle float64) (float64, error) {
		f := func(jd float64) (float64, error) {
			swephgo.SetEphePath([]byte(ephePath))
			xx := make([]float64, 6)
			serr := make([]byte, 256)
			iflgret := swephgo.CalcUt(jd, swephgo.SeSun, swephgo.SeflgSwieph, xx, serr)
			swephgo.Close()

			serrString := strings.TrimRight(string(serr), string(byte(0)))
			if len(serrString) > 0 || iflgret < 0 {
				return 0, fmt.Errorf("swe_calc_ut()错误。%s", serrString)
			}

			return mod180(xx[0] - angle), nil
		}
		return newtonIteration(x, f)
	}

	// 计算前一年冬至点jd
	jd := swephgo.Julday(year, 12, 20, 0.0, swephgo.SeGregCal)
	if dongZhiJd, err := fx(jd, 270.0); err != nil {
		return jds, err
	} else {

		jds[0] = dongZhiJd
	}

	//计算从此年冬至到下一年冬至的25个节所的jd(utc) ,第25个节气=下一年冬至
	for i := 1; i <= 24; i++ {
		// 每个节气大约差15天，因此将前一节气的jd + 15作为迭代初值，jds[i-1] + 15
		var angle = 270 + i*15
		if angle >= 360 {
			angle -= 360
		}
		if jd, err := fx(jds[i-1]+15, float64(angle)); err != nil {
			return jds, err
		} else {
			jds[i] = jd
		}
	}
	return jds, nil
}

/**
 * 计算从某一年冬至开始的连续15个新月
 * @param jd
 * 冬至点的儒略日
 */
func get15NewMoonJDs(jd float64, ephePath string) ([15]float64, error) {
	var moonJds [15]float64

	// 如果冬至点在满月之后会得到下一个合朔
	shuoDongZhiJd, err := getNewMoonJD(jd, ephePath)
	if err != nil {
		return moonJds, err
	}
	if shuoDongZhiJd > jd {
		shuoDongZhiJd, err = getNewMoonJD(jd-29.53, ephePath)
		if err != nil {
			return moonJds, err
		}
	}
	moonJds[0] = shuoDongZhiJd
	for i := 1; i <= 14; i++ {
		moonJds[i], err = getNewMoonJD(moonJds[i-1]+29.53, ephePath)
		if err != nil {
			return moonJds, err
		}
	}
	return moonJds, err
}

/**
 * 计算给定jd所在农历月，日月合朔的jd
 * 如果jd在满月之后，迭代值为下一个合朔
 */
func getNewMoonJD(jd float64, ephePath string) (float64, error) {
	shuoJd, err := newtonIteration(jd, func(jd float64) (float64, error) {
		swephgo.SetEphePath([]byte(ephePath))

		//计算太阳黄道经度
		xx := make([]float64, 6)
		serr := make([]byte, 256)
		iflgret := swephgo.CalcUt(jd, swephgo.SeSun, swephgo.SeflgSwieph, xx, serr)

		serrString := strings.TrimRight(string(serr), string(byte(0)))
		if len(serrString) > 0 || iflgret < 0 {
			swephgo.Close()
			return 0, fmt.Errorf("swe_calc_ut()错误。%s", serrString)
		}

		sunPosi := xx[0]

		// 计算月亮黄道经度
		iflgret = swephgo.CalcUt(jd, swephgo.SeMoon, swephgo.SeflgSwieph, xx, serr)
		serrString = strings.TrimRight(string(serr), string(byte(0)))
		if len(serrString) > 0 || iflgret < 0 {
			swephgo.Close()
			return 0, fmt.Errorf("swe_calc_ut()错误。%s", serrString)
		}
		moonPosi := xx[0]
		swephgo.Close()

		return mod180(swephgo.Degnorm(moonPosi - sunPosi)), nil
	})

	if err != nil {
		return 0, err
	}

	return shuoJd, nil
}

/**
 * 计算从某年冬至开始连续15个农历月初一的儒略日
 * @param jds
 * 从冬至点所在月份开始，连续15个新月的儒略日
 */
func get15LunarMonthJds(jds [15]float64) [15]LunarMonth {
	var firstDayJds = [15]LunarMonth{}

	for index, jd := range jds {

		y := make([]int, 1)
		m := make([]int, 1)
		d := make([]int, 1)
		hour := make([]float64, 1)
		swephgo.Revjul(jd, swephgo.SeGregCal, y, m, d, hour)
		h := int(math.Floor(hour[0]))
		mi := int(math.Floor((hour[0] - float64(h)) * 60))
		sec := ((hour[0]-float64(h))*60 - float64(mi)) * 60

		// 将新月的jd换算到东八区
		y8 := make([]int, 1)
		m8 := make([]int, 1)
		d8 := make([]int, 1)
		h8 := make([]int, 1)
		mi8 := make([]int, 1)
		sec8 := make([]float64, 1)
		swephgo.UtcTimeZone(y[0], m[0], d[0], h, mi, sec, -8.0, y8, m8, d8, h8, mi8, sec8)
		// 以新月当天00:00:00为初一，计算儒略日
		swephgo.UtcTimeZone(y8[0], m8[0], d8[0], 0, 0, 0.0, 8.0, y8, m8, d8, h8, mi8, sec8)
		jd = swephgo.Julday(y8[0], m8[0], d8[0], float64(h8[0])+float64(mi8[0])/60.0+sec8[0]/3600.0, swephgo.SeGregCal)
		var n = (index + 11) % 12
		if n == 0 {
			n = 12
		}
		firstDayJds[index].num = n
		firstDayJds[index].jd = jd
	}
	return firstDayJds
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
func calcLeapMonth(lunarMonth [15]LunarMonth, jdsMiddleSolarTerm [13]float64) [15]LunarMonth {
	// 找出区间[m_0, m_1]间的农历月
	// 只计数[m_0, m_1)之间的月数，
	// 此月数等于13，则置闰
	var n = func(months [15]LunarMonth, jd float64) int {
		i := 0
		for i = 0; i < 15; i++ {
			if months[i].jd > jd {
				// 此时i=区间[m_0, m_1]之长度
				return i - 1
			}
		}
		// 以下这行，实际上是不会执行到的
		return 15
	}(lunarMonth, jdsMiddleSolarTerm[12])

	if n == 12 {
		return lunarMonth
	}

	for i := 0; i < len(lunarMonth)-1; i++ {
		// 月中有中气:true，无中气:false
		var middleSolarTerm = false
		// len(jdsMiddleSolarTerm) - 1是因为排除今年的冬至点
		// jdsMiddleSolarTerm 的最后一个值即是今年的冬至点
		for j := 0; j < len(jdsMiddleSolarTerm)-1; j++ {
			if lunarMonth[i].jd < jdsMiddleSolarTerm[j] && jdsMiddleSolarTerm[j] < lunarMonth[i+1].jd {
				middleSolarTerm = true
				break
			}
		}
		if !middleSolarTerm {
			lunarMonth[i].isLeap = true
			for j := i; j < len(lunarMonth); j++ {
				lunarMonth[j].num--
				if lunarMonth[j].num == 0 {
					lunarMonth[j].num += 12
				}
			}
			break
		}
	}
	return lunarMonth
}
