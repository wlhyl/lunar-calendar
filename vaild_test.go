package lunarcalendar

import "testing"

/**
测试 func vaildDateTime()
*/
func TestVaildDateTime(t *testing.T) {
	t.Log("测试函数`vaildDateTime`用于检查时间是否合法")

	// 没有公元0年
	if err := vaildDateTime(0, 1, 1, 0, 0, 0); err == nil {
		t.Fatal("没有公元0年")
	}

	// 不正确的月份
	for _, v := range [...]int{-2, -1, 0, 13, 14} {
		if err := vaildDateTime(2021, v, 1, 0, 0, 0); err == nil {
			t.Fatalf("没有月份：%v", v)
		}
	}

	// 不正确的日
	for _, v := range [...]int{-2, -1, 0, 32, 33} {
		if err := vaildDateTime(2021, 1, v, 0, 0, 0); err == nil {
			t.Fatalf("没有日数：%v", v)
		}
	}

	// 不正确的时
	for _, v := range [...]int{-2, -1, 24, 25} {
		if err := vaildDateTime(2021, 1, 1, v, 0, 0); err == nil {
			t.Fatalf("没有小时数：%v", v)
		}
	}

	// 不正确的分
	for _, v := range [...]int{-2, -1, 60, 61} {
		if err := vaildDateTime(2021, 1, 1, 0, v, 0); err == nil {
			t.Fatalf("没有分钟数：%v", v)
		}
	}

	// 不正确的秒
	for _, v := range [...]int{-2, -1, 60, 61} {
		if err := vaildDateTime(2021, 1, 1, 0, 0, v); err == nil {
			t.Fatalf("没有秒数：%v", v)
		}
	}

	// 1582年10月5日-1582年10月14日不存在
	for d := 5; d < 15; d++ {
		if err := vaildDateTime(1582, 10, d, 0, 0, 0); err == nil {
			t.Fatalf("1582-10-%v 0:0:0 不是正确日期", d)
		}
	}

	// 2月29日
	if err := vaildDateTime(2021, 2, 29, 0, 0, 0); err == nil {
		t.Fatal("2021-2-29 0:0:0 没有此日期")
	}

	//4月31日
	if err := vaildDateTime(2021, 4, 31, 0, 0, 0); err == nil {
		t.Fatal("2021-4-31 0:0:0 没有此日期")
	}

	// 4月31日
	//能正确处理10月4日，10月15日
	if err := vaildDateTime(1582, 10, 4, 2, 5, 10); err != nil {
		t.Fatal("1582-10-4 2:5:10 是正确时间")
	}

	if err := vaildDateTime(1582, 10, 15, 2, 15, 10); err != nil {
		t.Fatal("1582-10-15 2:15:10 是正确时间")
	}

	// 测试正确的时间
	if err := vaildDateTime(2021, 4, 8, 0, 0, 0); err != nil {
		t.Fatal("2021-4-8 0:0:0 是正确时间")
	}
	if err := vaildDateTime(2021, 4, 2, 1, 1, 1); err != nil {
		t.Fatal("2021-4-2 1:1:1 是正确时间")
	}
	if err := vaildDateTime(2021, 4, 30, 23, 59, 59); err != nil {
		t.Fatal("2021-4-30 23:59:59 是正确时间")
	}
	if err := vaildDateTime(2021, 5, 31, 0, 0, 0); err != nil {
		t.Fatal("2021-5-31 0:0:0 是正确时间")
	}
	if err := vaildDateTime(2021, 2, 28, 0, 0, 0); err != nil {
		t.Fatalf("2021-2-28 0:0:0 是正确时间")
	}

	t.Log("函数`vaildDateTime`用于检查时间是否合法，正确。")

}
