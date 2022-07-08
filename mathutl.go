package lunarcalendar

import (
	"math"

	"github.com/mshafiee/swephgo"
)

/**
 * ModPi 把角度限制在[-180, 180]之间
 */
func mod180(r0 float64) float64 {
	var r = r0
	for r < -180 {
		r += 360
	}
	for r > 180 {
		r -= 360
	}
	return r
}

// 从儒略日得到东8区的日期
func getUT8DateTimeFromJd(jd float64) (int, int, int, int, int, float64) {
	var gregflag int = swephgo.SeGregCal
	if jd < 2299160.5 {
		gregflag = swephgo.SeJulCal
	}
	y := make([]int, 1)
	m := make([]int, 1)
	d := make([]int, 1)
	hour := make([]float64, 1)
	swephgo.Revjul(jd, gregflag, y, m, d, hour)

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

	return y8[0], m8[0], d8[0], h8[0], mi8[0], sec8[0]
}

/**
 * NewtonIteration 牛顿迭代法求解方程的根
 */
func newtonIteration(initValue float64, f func(float642 float64) (float64, error)) (float64, error) {
	var epsilon float64 = 1e-7
	var delta float64 = 5e-6
	var x float64 = 0.0
	var x0 = initValue

	for true {
		x = x0
		fx, err := f(x)
		if err != nil {
			return 0, err
		}
		// 导数
		fxDelta, err := f(x + delta)
		if err != nil {
			return 0, err
		}
		fpx := (fxDelta - fx) / delta
		x0 = x - fx/fpx
		if math.Abs(x0-x) <= epsilon {
			break
		}
	}
	return x, nil
}
