package lunarcalendar

import (
	"math"
	"testing"
)

func TestMod180(t *testing.T) {
	expected := 186.1 - 360
	d := 186.1
	actual := mod180(d)
	if math.Abs((expected-actual)*3600) >= 1 {
		t.Fatalf("mod180(%v)=%v, 而非%v", d, expected, actual)
	}
	expected = -186.1 + 360
	d = -186.1
	actual = mod180(d)
	if math.Abs((expected-actual)*3600) >= 1 {
		t.Fatalf("mod180(%v)=%v, 而非%v", d, expected, actual)
	}

	expected = 186.0 - 360
	d = 186.0 + 360*2
	actual = mod180(d)
	if math.Abs((expected-actual)*3600) >= 1 {
		t.Fatalf("mod180(%v)=%v, 而非%v", d, expected, actual)
	}

	expected = -186.0 + 360
	d = -186.0 - 360*2
	actual = mod180(d)
	if math.Abs((expected-actual)*3600) >= 1 {
		t.Fatalf("mod180(%v)=%v, 而非%v", d, expected, actual)
	}
}

func TestGetUT8DateTimeFromJd(t *testing.T) {
	// 2459760.5: 2022年6月30日 0:0:0
	//jd: 2022-6-30 22:31:48
	jd := 2459760.5 + (22-8+31/60.0+48/3600.0)/24.0
	y, m, d, h, mi, sec := getUT8DateTimeFromJd(jd)
	if y != 2022 || m != 6 || d != 30 || h != 22 || mi != 31 || (sec-48) >= 1 {
		t.Fatalf("从2022-6-30 22:31:48的儒略日得到%v-%v-%v %v:%v:%v", y, m, d, h, mi, sec)
	}
}

func TestNewtonIteration(t *testing.T) {
	t.Log("测试牛顿迭代")
	t.Log("fx=x^2+x-1 在[0,1]上的根")
	fx := func(x float64) (float64, error) {
		return x*x + x - 1, nil
	}

	y, err := newtonIteration(0, fx)
	if err != nil {
		t.Fatal(err)
	}
	x := (-1 + math.Sqrt(5)) / 2.0
	var epsilon float64 = 1e-7
	if d := math.Abs(x - y); d >= epsilon {
		t.Fatalf("fx=x^2+x-1 在[0,1]上的根求解失败, 误差%v>=%v", d, epsilon)
	}
}
