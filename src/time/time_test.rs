// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::time::time::{self, Month, Weekday};

// import (
// 	"bytes"
// 	"encoding/gob"
// 	"encoding/json"
// 	"fmt"
// 	"math"
// 	"math/big"
// 	"math/rand"
// 	"os"
// 	"runtime"
// 	"strings"
// 	"sync"
// 	"testing"
// 	"testing/quick"
// 	. "time"
// )

// // We should be in PST/PDT, but if the time zone files are missing we
// // won't be. The purpose of this test is to at least explain why some of
// // the subsequent tests fail.
// fn TestZoneData() {
// 	lt := Now()
// 	// PST is 8 hours west, PDT is 7 hours west. We could use the name but it's not unique.
// 	if name, off := lt.Zone(); off != -8*60*60 && off != -7*60*60 {
// 		t.Errorf("Unable to find US Pacific time zone data for testing; time zone is %q offset {}", name, off)
// 		t.Error("Likely problem: the time zone files have not been installed.")
// 	}
// }

// ParsedTime is the struct representing a parsed time value.
struct ParsedTime {
    year: isize,
    month: usize, // Month
    day: usize,
    hour: usize,
    minute: usize,
    second: usize,
    nanosecond: usize, // Fractional second.
    weekday: usize,    // Weekday

                       // 	ZoneOffset           isize    // seconds east of UTC, e.g. -7*60*60 for -0700
                       // 	Zone                 string // e.g., "MST"
}

struct TimeTest {
    seconds: i64,
    golden: ParsedTime,
}

impl TimeTest {
    const fn new(
        seconds: i64,
        year: isize,
        month: Month,
        day: usize,
        hour: usize,
        minute: usize,
        second: usize,
        nanosecond: usize,
        weekday: Weekday,
    ) -> Self {
        TimeTest {
            seconds,
            golden: ParsedTime {
                year,
                month: month as usize,
                day,
                hour,
                minute,
                second,
                nanosecond,
                weekday: weekday as usize,
            },
        }
    }
}

const UTC_TESTS: &[TimeTest] = &[
    TimeTest::new(
        0,
        1970,
        Month::January,
        1,
        0,
        0,
        0,
        0,
        Weekday::Thursday, /* , 0, "UTC"*/
    ),
    TimeTest::new(
        1221681866,
        2008,
        Month::September,
        17,
        20,
        4,
        26,
        0,
        Weekday::Wednesday, /* , 0, "UTC"*/
    ),
    TimeTest::new(
        -1221681866,
        1931,
        Month::April,
        16,
        3,
        55,
        34,
        0,
        Weekday::Thursday, /* , 0, "UTC"*/
    ),
    TimeTest::new(
        -11644473600,
        1601,
        Month::January,
        1,
        0,
        0,
        0,
        0,
        Weekday::Monday, /* , 0, "UTC"*/
    ),
    TimeTest::new(
        599529660,
        1988,
        Month::December,
        31,
        0,
        1,
        0,
        0,
        Weekday::Saturday, /* , 0, "UTC"*/
    ),
    TimeTest::new(
        978220860,
        2000,
        Month::December,
        31,
        0,
        1,
        0,
        0,
        Weekday::Sunday, /* , 0, "UTC"*/
    ),
];

const NANO_UTC_TESTS: &[TimeTest] = &[
    TimeTest::new(
        0,
        1970,
        Month::January,
        1,
        0,
        0,
        0,
        100_000_000,
        Weekday::Thursday, /* 0, "UTC" */
    ),
    TimeTest::new(
        1221681866,
        2008,
        Month::September,
        17,
        20,
        4,
        26,
        200_000_000,
        Weekday::Wednesday, /* 0, "UTC" */
    ),
];

// var localtests = []TimeTest{
// 	{0, ParsedTime{1969, December, 31, 16, 0, 0, 0, Wednesday, -8 * 60 * 60, "PST"}},
// 	{1221681866, ParsedTime{2008, September, 17, 13, 4, 26, 0, Wednesday, -7 * 60 * 60, "PDT"}},
// 	{2159200800, ParsedTime{2038, June, 3, 11, 0, 0, 0, Thursday, -7 * 60 * 60, "PDT"}},
// 	{2152173599, ParsedTime{2038, March, 14, 1, 59, 59, 0, Sunday, -8 * 60 * 60, "PST"}},
// 	{2152173600, ParsedTime{2038, March, 14, 3, 0, 0, 0, Sunday, -7 * 60 * 60, "PDT"}},
// 	{2152173601, ParsedTime{2038, March, 14, 3, 0, 1, 0, Sunday, -7 * 60 * 60, "PDT"}},
// 	{2172733199, ParsedTime{2038, November, 7, 1, 59, 59, 0, Sunday, -7 * 60 * 60, "PDT"}},
// 	{2172733200, ParsedTime{2038, November, 7, 1, 0, 0, 0, Sunday, -8 * 60 * 60, "PST"}},
// 	{2172733201, ParsedTime{2038, November, 7, 1, 0, 1, 0, Sunday, -8 * 60 * 60, "PST"}},
// }

// var nanolocaltests = []TimeTest{
// 	{0, ParsedTime{1969, December, 31, 16, 0, 0, 1e8, Wednesday, -8 * 60 * 60, "PST"}},
// 	{1221681866, ParsedTime{2008, September, 17, 13, 4, 26, 3e8, Wednesday, -7 * 60 * 60, "PDT"}},
// }

fn same(t: &time::Time, u: &ParsedTime) -> bool {
    // Check aggregates.
    let ymd = t.date();
    let hms = t.clock();
    // let (name, offset) = t.Zone();
    if ymd.year != u.year
        || ymd.month != u.month
        || ymd.day != u.day
        || hms.hour != u.hour
        || hms.min != u.minute
        || hms.sec != u.second
    // || name != u.Zone
    // || offset != u.ZoneOffset
    {
        return false;
    }
    // Check individual entries.
    t.year() == u.year
        && t.month() == u.month
        && t.day() == u.day
        && t.hour() == u.hour as u8
        && t.minute() == u.minute as u8
        && t.second() == u.second as u8
        && t.nanosecond() == u.nanosecond
        && t.weekday() == u.weekday
}

#[test]
fn test_seconds_to_utc() {
    for test in UTC_TESTS {
        let sec = test.seconds;
        let golden = &test.golden;
        let tm = time::unix(sec, 0)/* .UTC()*/;
        let newsec = tm.unix();
        assert_eq!(sec, newsec);
        assert!(same(&tm, golden));
    }
}

#[test]
fn test_nanoseconds_to_utc() {
    for test in NANO_UTC_TESTS {
        let golden = &test.golden;
        let nsec = test.seconds * 1_000_000_000 + golden.nanosecond as i64;
        let tm = time::unix(0, nsec)/* .UTC() */;
        let newnsec = tm.unix() * 1_000_000_000 + tm.nanosecond() as i64;
        assert_eq!(nsec, newnsec);
        assert!(same(&tm, golden));
    }
}

// fn TestSecondsToLocalTime() {
// 	for test in localtests {
// 		sec := test.seconds
// 		golden := &test.golden
// 		tm := unix(sec, 0)
// 		newsec := tm.unix()
// 		if newsec != sec {
// 			t.Errorf("SecondsToLocalTime({}).Seconds() = {}", sec, newsec)
// 		}
// 		if !same(tm, golden) {
// 			t.Errorf("SecondsToLocalTime({}):", sec)
// 			t.Errorf("  want=%+v", *golden)
// 			t.Errorf("  have=%+v", tm.Format(RFC3339+" MST"))
// 		}
// 	}
// }

// fn TestNanosecondsToLocalTime() {
// 	for test in nanolocaltests {
// 		golden := &test.golden
// 		nsec := test.seconds*1e9 + i64(golden.nanosecond)
// 		tm := unix(0, nsec)
// 		newnsec := tm.unix()*1e9 + i64(tm.nanosecond())
// 		if newnsec != nsec {
// 			t.Errorf("NanosecondsToLocalTime({}).Seconds() = {}", nsec, newnsec)
// 		}
// 		if !same(tm, golden) {
// 			t.Errorf("NanosecondsToLocalTime({}):", nsec)
// 			t.Errorf("  want=%+v", *golden)
// 			t.Errorf("  have=%+v", tm.Format(RFC3339+" MST"))
// 		}
// 	}
// }

// fn TestSecondsToUTCAndBack() {
// 	f := fn(sec i64) bool { return unix(sec, 0).UTC().unix() == sec }
// 	f32 := fn(sec int32) bool { return f(i64(sec)) }
// 	cfg := &quick.Config{MaxCount: 10000}

// 	// Try a reasonable date first, then the huge ones.
// 	if err := quick.Check(f32, cfg); err != nil {
// 		t.Fatal(err)
// 	}
// 	if err := quick.Check(f, cfg); err != nil {
// 		t.Fatal(err)
// 	}
// }

// fn TestNanosecondsToUTCAndBack() {
// 	f := fn(nsec i64) bool {
// 		t := unix(0, nsec).UTC()
// 		ns := t.unix()*1e9 + i64(t.nanosecond())
// 		return ns == nsec
// 	}
// 	f32 := fn(nsec int32) bool { return f(i64(nsec)) }
// 	cfg := &quick.Config{MaxCount: 10000}

// 	// Try a small date first, then the large ones. (The span is only a few hundred years
// 	// for nanoseconds in an i64.)
// 	if err := quick.Check(f32, cfg); err != nil {
// 		t.Fatal(err)
// 	}
// 	if err := quick.Check(f, cfg); err != nil {
// 		t.Fatal(err)
// 	}
// }

// fn TestUnixMilli() {
// 	f := fn(msec i64) bool {
// 		t := UnixMilli(msec)
// 		return t.UnixMilli() == msec
// 	}
// 	cfg := &quick.Config{MaxCount: 10000}
// 	if err := quick.Check(f, cfg); err != nil {
// 		t.Fatal(err)
// 	}
// }

// fn TestUnixMicro() {
// 	f := fn(usec i64) bool {
// 		t := UnixMicro(usec)
// 		return t.UnixMicro() == usec
// 	}
// 	cfg := &quick.Config{MaxCount: 10000}
// 	if err := quick.Check(f, cfg); err != nil {
// 		t.Fatal(err)
// 	}
// }

// // The time routines provide no way to get absolute time
// // (seconds since zero), but we need it to compute the right
// // answer for bizarre roundings like "to the nearest 3 ns".
// // Compute as t - year1 = (t - 1970) + (1970 - 2001) + (2001 - 1).
// // t - 1970 is returned by unix and Nanosecond.
// // 1970 - 2001 is -(31*365+8)*86400 = -978307200 seconds.
// // 2001 - 1 is 2000*365.2425*86400 = 63113904000 seconds.
// const unixToZero = -978307200 + 63113904000

// // abs returns the absolute time stored in t, as seconds and nanoseconds.
// fn abs(t Time) (sec, nsec i64) {
// 	unix := t.unix()
// 	nano := t.nanosecond()
// 	return unix + unixToZero, i64(nano)
// }

// // absString returns abs as a decimal string.
// fn absString(t Time) string {
// 	sec, nsec := abs(t)
// 	if sec < 0 {
// 		sec = -sec
// 		nsec = -nsec
// 		if nsec < 0 {
// 			nsec += 1e9
// 			sec--
// 		}
// 		return fmt.Sprintf("-{}%09d", sec, nsec)
// 	}
// 	return fmt.Sprintf("{}%09d", sec, nsec)
// }

// var truncateRoundTests = []struct {
// 	t Time
// 	d Duration
// }{
// 	{Date(-1, January, 1, 12, 15, 30, 5e8, UTC), 3},
// 	{Date(-1, January, 1, 12, 15, 31, 5e8, UTC), 3},
// 	{Date(2012, January, 1, 12, 15, 30, 5e8, UTC), Second},
// 	{Date(2012, January, 1, 12, 15, 31, 5e8, UTC), Second},
// 	{unix(-19012425939, 649146258), 7435029458905025217}, // 5.8*d rounds to 6*d, but .8*d+.8*d < 0 < d
// }

// fn TestTruncateRound() {
// 	var (
// 		bsec  = new(big.Int)
// 		bnsec = new(big.Int)
// 		bd    = new(big.Int)
// 		bt    = new(big.Int)
// 		br    = new(big.Int)
// 		bq    = new(big.Int)
// 		b1e9  = new(big.Int)
// 	)

// 	b1e9.SetInt64(1e9)

// 	testOne := fn(ti, tns, di i64) bool {
// 		t.Helper()

// 		t0 := unix(ti, i64(tns)).UTC()
// 		d := Duration(di)
// 		if d < 0 {
// 			d = -d
// 		}
// 		if d <= 0 {
// 			d = 1
// 		}

// 		// Compute bt = absolute nanoseconds.
// 		sec, nsec := abs(t0)
// 		bsec.SetInt64(sec)
// 		bnsec.SetInt64(nsec)
// 		bt.Mul(bsec, b1e9)
// 		bt.Add(bt, bnsec)

// 		// Compute quotient and remainder mod d.
// 		bd.SetInt64(i64(d))
// 		bq.DivMod(bt, bd, br)

// 		// To truncate, subtract remainder.
// 		// br is < d, so it fits in an i64.
// 		r := br.Int64()
// 		t1 := t0.Add(-Duration(r))

// 		// Check that time.Truncate works.
// 		if trunc := t0.Truncate(d); trunc != t1 {
// 			t.Errorf("Time.Truncate(%s, %s) = %s, want %s\n"+
// 				"{} trunc {} =\n{} want\n{}",
// 				t0.Format(RFC3339Nano), d, trunc, t1.Format(RFC3339Nano),
// 				absString(t0), i64(d), absString(trunc), absString(t1))
// 			return false
// 		}

// 		// To round, add d back if remainder r > d/2 or r == exactly d/2.
// 		// The commented out code would round half to even instead of up,
// 		// but that makes it time-zone dependent, which is a bit strange.
// 		if r > i64(d)/2 || r+r == i64(d) /*&& bq.Bit(0) == 1*/ {
// 			t1 = t1.Add(Duration(d))
// 		}

// 		// Check that time.Round works.
// 		if rnd := t0.Round(d); rnd != t1 {
// 			t.Errorf("Time.Round(%s, %s) = %s, want %s\n"+
// 				"{} round {} =\n{} want\n{}",
// 				t0.Format(RFC3339Nano), d, rnd, t1.Format(RFC3339Nano),
// 				absString(t0), i64(d), absString(rnd), absString(t1))
// 			return false
// 		}
// 		return true
// 	}

// 	// manual test cases
// 	for _, tt := range truncateRoundTests {
// 		testOne(tt.t.unix(), i64(tt.t.nanosecond()), i64(tt.d))
// 	}

// 	// exhaustive near 0
// 	for i := 0; i < 100; i += 1 {
// 		for j := 1; j < 100; j++ {
// 			testOne(unixToZero, i64(i), i64(j))
// 			testOne(unixToZero, -i64(i), i64(j))
// 			if t.Failed() {
// 				return
// 			}
// 		}
// 	}

// 	if t.Failed() {
// 		return
// 	}

// 	// randomly generated test cases
// 	cfg := &quick.Config{MaxCount: 100000}
// 	if testing.Short() {
// 		cfg.MaxCount = 1000
// 	}

// 	// divisors of Second
// 	f1 := fn(ti i64, tns int32, logdi int32) bool {
// 		d := Duration(1)
// 		a, b := uint(logdi%9), (logdi>>16)%9
// 		d <<= a
// 		for i := 0; i < isize(b); i += 1 {
// 			d *= 5
// 		}

// 		// Make room for unix ↔ internal conversion.
// 		// We don't care about behavior too close to ± 2^63 unix seconds.
// 		// It is full of wraparounds but will never happen in a reasonable program.
// 		// (Or maybe not? See go.dev/issue/20678. In any event, they're not handled today.)
// 		ti >>= 1

// 		return testOne(ti, i64(tns), i64(d))
// 	}
// 	quick.Check(f1, cfg)

// 	// multiples of Second
// 	f2 := fn(ti i64, tns int32, di int32) bool {
// 		d := Duration(di) * Second
// 		if d < 0 {
// 			d = -d
// 		}
// 		ti >>= 1 // see comment in f1
// 		return testOne(ti, i64(tns), i64(d))
// 	}
// 	quick.Check(f2, cfg)

// 	// halfway cases
// 	f3 := fn(tns, di i64) bool {
// 		di &= 0xfffffffe
// 		if di == 0 {
// 			di = 2
// 		}
// 		tns -= tns % di
// 		if tns < 0 {
// 			tns += di / 2
// 		} else {
// 			tns -= di / 2
// 		}
// 		return testOne(0, tns, di)
// 	}
// 	quick.Check(f3, cfg)

// 	// full generality
// 	f4 := fn(ti i64, tns int32, di i64) bool {
// 		ti >>= 1 // see comment in f1
// 		return testOne(ti, i64(tns), di)
// 	}
// 	quick.Check(f4, cfg)
// }

// type ISOWeekTest struct {
// 	year       isize // year
// 	month, day isize // month and day
// 	yex        isize // expected year
// 	wex        isize // expected week
// }

// var isoWeekTests = []ISOWeekTest{
// 	{1981, 1, 1, 1981, 1}, {1982, 1, 1, 1981, 53}, {1983, 1, 1, 1982, 52},
// 	{1984, 1, 1, 1983, 52}, {1985, 1, 1, 1985, 1}, {1986, 1, 1, 1986, 1},
// 	{1987, 1, 1, 1987, 1}, {1988, 1, 1, 1987, 53}, {1989, 1, 1, 1988, 52},
// 	{1990, 1, 1, 1990, 1}, {1991, 1, 1, 1991, 1}, {1992, 1, 1, 1992, 1},
// 	{1993, 1, 1, 1992, 53}, {1994, 1, 1, 1993, 52}, {1995, 1, 2, 1995, 1},
// 	{1996, 1, 1, 1996, 1}, {1996, 1, 7, 1996, 1}, {1996, 1, 8, 1996, 2},
// 	{1997, 1, 1, 1997, 1}, {1998, 1, 1, 1998, 1}, {1999, 1, 1, 1998, 53},
// 	{2000, 1, 1, 1999, 52}, {2001, 1, 1, 2001, 1}, {2002, 1, 1, 2002, 1},
// 	{2003, 1, 1, 2003, 1}, {2004, 1, 1, 2004, 1}, {2005, 1, 1, 2004, 53},
// 	{2006, 1, 1, 2005, 52}, {2007, 1, 1, 2007, 1}, {2008, 1, 1, 2008, 1},
// 	{2009, 1, 1, 2009, 1}, {2010, 1, 1, 2009, 53}, {2010, 1, 1, 2009, 53},
// 	{2011, 1, 1, 2010, 52}, {2011, 1, 2, 2010, 52}, {2011, 1, 3, 2011, 1},
// 	{2011, 1, 4, 2011, 1}, {2011, 1, 5, 2011, 1}, {2011, 1, 6, 2011, 1},
// 	{2011, 1, 7, 2011, 1}, {2011, 1, 8, 2011, 1}, {2011, 1, 9, 2011, 1},
// 	{2011, 1, 10, 2011, 2}, {2011, 1, 11, 2011, 2}, {2011, 6, 12, 2011, 23},
// 	{2011, 6, 13, 2011, 24}, {2011, 12, 25, 2011, 51}, {2011, 12, 26, 2011, 52},
// 	{2011, 12, 27, 2011, 52}, {2011, 12, 28, 2011, 52}, {2011, 12, 29, 2011, 52},
// 	{2011, 12, 30, 2011, 52}, {2011, 12, 31, 2011, 52}, {1995, 1, 1, 1994, 52},
// 	{2012, 1, 1, 2011, 52}, {2012, 1, 2, 2012, 1}, {2012, 1, 8, 2012, 1},
// 	{2012, 1, 9, 2012, 2}, {2012, 12, 23, 2012, 51}, {2012, 12, 24, 2012, 52},
// 	{2012, 12, 30, 2012, 52}, {2012, 12, 31, 2013, 1}, {2013, 1, 1, 2013, 1},
// 	{2013, 1, 6, 2013, 1}, {2013, 1, 7, 2013, 2}, {2013, 12, 22, 2013, 51},
// 	{2013, 12, 23, 2013, 52}, {2013, 12, 29, 2013, 52}, {2013, 12, 30, 2014, 1},
// 	{2014, 1, 1, 2014, 1}, {2014, 1, 5, 2014, 1}, {2014, 1, 6, 2014, 2},
// 	{2015, 1, 1, 2015, 1}, {2016, 1, 1, 2015, 53}, {2017, 1, 1, 2016, 52},
// 	{2018, 1, 1, 2018, 1}, {2019, 1, 1, 2019, 1}, {2020, 1, 1, 2020, 1},
// 	{2021, 1, 1, 2020, 53}, {2022, 1, 1, 2021, 52}, {2023, 1, 1, 2022, 52},
// 	{2024, 1, 1, 2024, 1}, {2025, 1, 1, 2025, 1}, {2026, 1, 1, 2026, 1},
// 	{2027, 1, 1, 2026, 53}, {2028, 1, 1, 2027, 52}, {2029, 1, 1, 2029, 1},
// 	{2030, 1, 1, 2030, 1}, {2031, 1, 1, 2031, 1}, {2032, 1, 1, 2032, 1},
// 	{2033, 1, 1, 2032, 53}, {2034, 1, 1, 2033, 52}, {2035, 1, 1, 2035, 1},
// 	{2036, 1, 1, 2036, 1}, {2037, 1, 1, 2037, 1}, {2038, 1, 1, 2037, 53},
// 	{2039, 1, 1, 2038, 52}, {2040, 1, 1, 2039, 52},
// }

// fn TestISOWeek() {
// 	// Selected dates and corner cases
// 	for _, wt := range isoWeekTests {
// 		dt := Date(wt.year, Month(wt.month), wt.day, 0, 0, 0, 0, UTC)
// 		y, w := dt.ISOWeek()
// 		if w != wt.wex || y != wt.yex {
// 			t.Errorf("got {}/{}; expected {}/{} for {}-%02d-%02d",
// 				y, w, wt.yex, wt.wex, wt.year, wt.month, wt.day)
// 		}
// 	}

// 	// The only real invariant: Jan 04 is in week 1
// 	for year := 1950; year < 2100; year++ {
// 		if y, w := Date(year, January, 4, 0, 0, 0, 0, UTC).ISOWeek(); y != year || w != 1 {
// 			t.Errorf("got {}/{}; expected {}/1 for Jan 04", y, w, year)
// 		}
// 	}
// }

#[derive(Debug)]
struct YearDayTest {
    year: isize,
    month: usize,
    day: usize,
    yday: usize,
}

impl YearDayTest {
    const fn new(year: isize, month: usize, day: usize, yday: usize) -> Self {
        Self {
            year,
            month,
            day,
            yday,
        }
    }
}

// Test YearDay in several different scenarios
// and corner cases
const YEAR_DAY_TESTS: &[YearDayTest] = &[
    // Non-leap-year tests
    YearDayTest::new(2007, 1, 1, 1),
    YearDayTest::new(2007, 1, 15, 15),
    YearDayTest::new(2007, 2, 1, 32),
    YearDayTest::new(2007, 2, 15, 46),
    YearDayTest::new(2007, 3, 1, 60),
    YearDayTest::new(2007, 3, 15, 74),
    YearDayTest::new(2007, 4, 1, 91),
    YearDayTest::new(2007, 12, 31, 365),
    // Leap-year tests
    YearDayTest::new(2008, 1, 1, 1),
    YearDayTest::new(2008, 1, 15, 15),
    YearDayTest::new(2008, 2, 1, 32),
    YearDayTest::new(2008, 2, 15, 46),
    YearDayTest::new(2008, 3, 1, 61),
    YearDayTest::new(2008, 3, 15, 75),
    YearDayTest::new(2008, 4, 1, 92),
    YearDayTest::new(2008, 12, 31, 366),
    // Looks like leap-year (but isn't) tests
    YearDayTest::new(1900, 1, 1, 1),
    YearDayTest::new(1900, 1, 15, 15),
    YearDayTest::new(1900, 2, 1, 32),
    YearDayTest::new(1900, 2, 15, 46),
    YearDayTest::new(1900, 3, 1, 60),
    YearDayTest::new(1900, 3, 15, 74),
    YearDayTest::new(1900, 4, 1, 91),
    YearDayTest::new(1900, 12, 31, 365),
    // Year one tests (non-leap)
    YearDayTest::new(1, 1, 1, 1),
    YearDayTest::new(1, 1, 15, 15),
    YearDayTest::new(1, 2, 1, 32),
    YearDayTest::new(1, 2, 15, 46),
    YearDayTest::new(1, 3, 1, 60),
    YearDayTest::new(1, 3, 15, 74),
    YearDayTest::new(1, 4, 1, 91),
    YearDayTest::new(1, 12, 31, 365),
    // Year minus one tests (non-leap)
    YearDayTest::new(-1, 1, 1, 1),
    YearDayTest::new(-1, 1, 15, 15),
    YearDayTest::new(-1, 2, 1, 32),
    YearDayTest::new(-1, 2, 15, 46),
    YearDayTest::new(-1, 3, 1, 60),
    YearDayTest::new(-1, 3, 15, 74),
    YearDayTest::new(-1, 4, 1, 91),
    YearDayTest::new(-1, 12, 31, 365),
    // 400 BC tests (leap-year)
    YearDayTest::new(-400, 1, 1, 1),
    YearDayTest::new(-400, 1, 15, 15),
    YearDayTest::new(-400, 2, 1, 32),
    YearDayTest::new(-400, 2, 15, 46),
    YearDayTest::new(-400, 3, 1, 61),
    YearDayTest::new(-400, 3, 15, 75),
    YearDayTest::new(-400, 4, 1, 92),
    YearDayTest::new(-400, 12, 31, 366),
    // Special Cases
    // Gregorian calendar change (no effect)
    YearDayTest::new(1582, 10, 4, 277),
    YearDayTest::new(1582, 10, 15, 288),
];

#[test]
fn test_year_day() {
    // 	for i, loc := range yearDayLocations {
    for (index, test) in YEAR_DAY_TESTS.iter().enumerate() {
        let dt = time::date(
            test.year,
            test.month as isize,
            test.day as isize,
            0,
            0,
            0,
            0,
        );
        // let msg = format!();
        assert_eq!(
            test.yday,
            dt.year_day(),
            "failed test {}: {:?}",
            index,
            test
        );

        // 			if ydt.year < 0 || ydt.year > 9999 {
        // 				continue
        // 			}
        // 			f := fmt.Sprintf("%04d-%02d-%02d %03d %+.2d00",
        // 				ydt.year, ydt.month, ydt.day, ydt.yday, (i-2)*4)
        // 			dt1, err := Parse("2006-01-02 002 -0700", f)
        // 			if err != nil {
        // 				t.Errorf(`Parse("2006-01-02 002 -0700", %q): {}`, f, err)
        // 				continue
        // 			}
        // 			if !dt1.Equal(dt) {
        // 				t.Errorf(`Parse("2006-01-02 002 -0700", %q) = {}, want {}`, f, dt1, dt)
        // 			}
    }
    // }
}

// var durationTests = []struct {
// 	str string
// 	d   Duration
// }{
// 	{"0s", 0},
// 	{"1ns", 1 * Nanosecond},
// 	{"1.1µs", 1100 * Nanosecond},
// 	{"2.2ms", 2200 * Microsecond},
// 	{"3.3s", 3300 * Millisecond},
// 	{"4m5s", 4*Minute + 5*Second},
// 	{"4m5.001s", 4*Minute + 5001*Millisecond},
// 	{"5h6m7.001s", 5*Hour + 6*Minute + 7001*Millisecond},
// 	{"8m0.000000001s", 8*Minute + 1*Nanosecond},
// 	{"2562047h47m16.854775807s", 1<<63 - 1},
// 	{"-2562047h47m16.854775808s", -1 << 63},
// }

// fn TestDurationString() {
// 	for _, tt := range durationTests {
// 		if str := tt.d.String(); str != tt.str {
// 			t.Errorf("Duration({}).String() = %s, want %s", i64(tt.d), str, tt.str)
// 		}
// 		if tt.d > 0 {
// 			if str := (-tt.d).String(); str != "-"+tt.str {
// 				t.Errorf("Duration({}).String() = %s, want %s", i64(-tt.d), str, "-"+tt.str)
// 			}
// 		}
// 	}
// }

// var dateTests = []struct {
// 	year, month, day, hour, min, sec, nsec isize
// 	z                                      *Location
// 	unix                                   i64
// }{
// 	{2011, 11, 6, 1, 0, 0, 0, Local, 1320566400},   // 1:00:00 PDT
// 	{2011, 11, 6, 1, 59, 59, 0, Local, 1320569999}, // 1:59:59 PDT
// 	{2011, 11, 6, 2, 0, 0, 0, Local, 1320573600},   // 2:00:00 PST

// 	{2011, 3, 13, 1, 0, 0, 0, Local, 1300006800},   // 1:00:00 PST
// 	{2011, 3, 13, 1, 59, 59, 0, Local, 1300010399}, // 1:59:59 PST
// 	{2011, 3, 13, 3, 0, 0, 0, Local, 1300010400},   // 3:00:00 PDT
// 	{2011, 3, 13, 2, 30, 0, 0, Local, 1300008600},  // 2:30:00 PDT ≡ 1:30 PST
// 	{2012, 12, 24, 0, 0, 0, 0, Local, 1356336000},  // Leap year

// 	// Many names for Fri Nov 18 7:56:35 PST 2011
// 	{2011, 11, 18, 7, 56, 35, 0, Local, 1321631795},                 // Nov 18 7:56:35
// 	{2011, 11, 19, -17, 56, 35, 0, Local, 1321631795},               // Nov 19 -17:56:35
// 	{2011, 11, 17, 31, 56, 35, 0, Local, 1321631795},                // Nov 17 31:56:35
// 	{2011, 11, 18, 6, 116, 35, 0, Local, 1321631795},                // Nov 18 6:116:35
// 	{2011, 10, 49, 7, 56, 35, 0, Local, 1321631795},                 // Oct 49 7:56:35
// 	{2011, 11, 18, 7, 55, 95, 0, Local, 1321631795},                 // Nov 18 7:55:95
// 	{2011, 11, 18, 7, 56, 34, 1e9, Local, 1321631795},               // Nov 18 7:56:34 + 10⁹ns
// 	{2011, 12, -12, 7, 56, 35, 0, Local, 1321631795},                // Dec -21 7:56:35
// 	{2012, 1, -43, 7, 56, 35, 0, Local, 1321631795},                 // Jan -52 7:56:35 2012
// 	{2012, isize(January - 2), 18, 7, 56, 35, 0, Local, 1321631795},   // (Jan-2) 18 7:56:35 2012
// 	{2010, isize(December + 11), 18, 7, 56, 35, 0, Local, 1321631795}, // (Dec+11) 18 7:56:35 2010
// }

// fn TestDate() {
// 	for _, tt := range dateTests {
// 		time := Date(tt.year, Month(tt.month), tt.day, tt.hour, tt.min, tt.sec, tt.nsec, tt.z)
// 		want := unix(tt.unix, 0)
// 		if !time.Equal(want) {
// 			t.Errorf("Date({}, {}, {}, {}, {}, {}, {}, %s) = {}, want {}",
// 				tt.year, tt.month, tt.day, tt.hour, tt.min, tt.sec, tt.nsec, tt.z,
// 				time, want)
// 		}
// 	}
// }

// // Several ways of getting from
// // Fri Nov 18 7:56:35 PST 2011
// // to
// // Thu Mar 19 7:56:35 PST 2016
// var addDateTests = []struct {
// 	years, months, days isize
// }{
// 	{4, 4, 1},
// 	{3, 16, 1},
// 	{3, 15, 30},
// 	{5, -6, -18 - 30 - 12},
// }

// fn TestAddDate() {
// 	t0 := Date(2011, 11, 18, 7, 56, 35, 0, UTC)
// 	t1 := Date(2016, 3, 19, 7, 56, 35, 0, UTC)
// 	for _, at := range addDateTests {
// 		time := t0.AddDate(at.years, at.months, at.days)
// 		if !time.Equal(t1) {
// 			t.Errorf("AddDate({}, {}, {}) = {}, want {}",
// 				at.years, at.months, at.days,
// 				time, t1)
// 		}
// 	}
// }

#[test]
fn test_days_in() {
    struct Test {
        year: isize,
        month: usize,
        di: usize,
    }
    const DAYS_IN_TESTS: &[Test] = &[
        // 	year, month, di isize
        Test {
            year: 2011,
            month: 1,
            di: 31,
        }, // January, first month, 31 days
        Test {
            year: 2011,
            month: 2,
            di: 28,
        }, // February, non-leap year, 28 days
        Test {
            year: 2012,
            month: 2,
            di: 29,
        }, // February, leap year, 29 days
        Test {
            year: 2011,
            month: 6,
            di: 30,
        }, // June, 30 days
        Test {
            year: 2011,
            month: 12,
            di: 31,
        }, // December, last month, 31 days
    ];

    for tt in DAYS_IN_TESTS {
        let di = time::days_in(tt.month, tt.year);
        assert_eq!(tt.di, di);
    }
}

// fn TestAddToExactSecond() {
// 	// Add an amount to the current time to round it up to the next exact second.
// 	// This test checks that the nsec field still lies within the range [0, 999999999].
// 	t1 := Now()
// 	t2 := t1.Add(Second - Duration(t1.nanosecond()))
// 	sec := (t1.second() + 1) % 60
// 	if t2.second() != sec || t2.nanosecond() != 0 {
// 		t.Errorf("sec = {}, nsec = {}, want sec = {}, nsec = 0", t2.second(), t2.nanosecond(), sec)
// 	}
// }

// fn equalTimeAndZone(a, b Time) bool {
// 	aname, aoffset := a.Zone()
// 	bname, boffset := b.Zone()
// 	return a.Equal(b) && aoffset == boffset && aname == bname
// }

// var gobTests = []Time{
// 	Date(0, 1, 2, 3, 4, 5, 6, UTC),
// 	Date(7, 8, 9, 10, 11, 12, 13, FixedZone("", 0)),
// 	unix(81985467080890095, 0x76543210), // Time.sec: 0x0123456789ABCDEF
// 	{},                                  // nil location
// 	Date(1, 2, 3, 4, 5, 6, 7, FixedZone("", 32767*60)),
// 	Date(1, 2, 3, 4, 5, 6, 7, FixedZone("", -32768*60)),
// }

// fn TestTimeGob() {
// 	let mut b = bytes::Buffer::new();
// 	enc := gob.NewEncoder(&b)
// 	dec := gob.NewDecoder(&b)
// 	for _, tt := range gobTests {
// 		var gobtt Time
// 		if err := enc.Encode(&tt); err != nil {
// 			t.Errorf("{} gob Encode error = %q, want nil", tt, err)
// 		} else if err := dec.Decode(&gobtt); err != nil {
// 			t.Errorf("{} gob Decode error = %q, want nil", tt, err)
// 		} else if !equalTimeAndZone(gobtt, tt) {
// 			t.Errorf("Decoded time = {}, want {}", gobtt, tt)
// 		}
// 		b.reset()
// 	}
// }

// var invalidEncodingTests = []struct {
// 	bytes [u8]
// 	want  string
// }{
// 	{[u8]{}, "Time.UnmarshalBinary: no data"},
// 	{[u8]{0, 2, 3}, "Time.UnmarshalBinary: unsupported version"},
// 	{[u8]{1, 2, 3}, "Time.UnmarshalBinary: invalid length"},
// }

// fn TestInvalidTimeGob() {
// 	for _, tt := range invalidEncodingTests {
// 		var ignored Time
// 		err := ignored.GobDecode(tt.bytes)
// 		if err == nil || err.Error() != tt.want {
// 			t.Errorf("time.GobDecode(%#v) error = {}, want {}", tt.bytes, err, tt.want)
// 		}
// 		err = ignored.UnmarshalBinary(tt.bytes)
// 		if err == nil || err.Error() != tt.want {
// 			t.Errorf("time.UnmarshalBinary(%#v) error = {}, want {}", tt.bytes, err, tt.want)
// 		}
// 	}
// }

// var notEncodableTimes = []struct {
// 	time Time
// 	want string
// }{
// 	{Date(0, 1, 2, 3, 4, 5, 6, FixedZone("", -1*60)), "Time.MarshalBinary: unexpected zone offset"},
// 	{Date(0, 1, 2, 3, 4, 5, 6, FixedZone("", -32769*60)), "Time.MarshalBinary: unexpected zone offset"},
// 	{Date(0, 1, 2, 3, 4, 5, 6, FixedZone("", 32768*60)), "Time.MarshalBinary: unexpected zone offset"},
// }

// fn TestNotGobEncodableTime() {
// 	for _, tt := range notEncodableTimes {
// 		_, err := tt.time.GobEncode()
// 		if err == nil || err.Error() != tt.want {
// 			t.Errorf("{} GobEncode error = {}, want {}", tt.time, err, tt.want)
// 		}
// 		_, err = tt.time.MarshalBinary()
// 		if err == nil || err.Error() != tt.want {
// 			t.Errorf("{} MarshalBinary error = {}, want {}", tt.time, err, tt.want)
// 		}
// 	}
// }

// var jsonTests = []struct {
// 	time Time
// 	json string
// }{
// 	{Date(9999, 4, 12, 23, 20, 50, 520*1e6, UTC), `"9999-04-12T23:20:50.52Z"`},
// 	{Date(1996, 12, 19, 16, 39, 57, 0, Local), `"1996-12-19T16:39:57-08:00"`},
// 	{Date(0, 1, 1, 0, 0, 0, 1, FixedZone("", 1*60)), `"0000-01-01T00:00:00.000000001+00:01"`},
// 	{Date(2020, 1, 1, 0, 0, 0, 0, FixedZone("", 23*60*60+59*60)), `"2020-01-01T00:00:00+23:59"`},
// }

// fn TestTimeJSON() {
// 	for _, tt := range jsonTests {
// 		var jsonTime Time

// 		if jsonBytes, err := json.Marshal(tt.time); err != nil {
// 			t.Errorf("{} json.Marshal error = {}, want nil", tt.time, err)
// 		} else if string(jsonBytes) != tt.json {
// 			t.Errorf("{} JSON = %#q, want %#q", tt.time, string(jsonBytes), tt.json)
// 		} else if err = json.Unmarshal(jsonBytes, &jsonTime); err != nil {
// 			t.Errorf("{} json.Unmarshal error = {}, want nil", tt.time, err)
// 		} else if !equalTimeAndZone(jsonTime, tt.time) {
// 			t.Errorf("Unmarshaled time = {}, want {}", jsonTime, tt.time)
// 		}
// 	}
// }

// fn TestUnmarshalInvalidTimes() {
// 	tests := []struct {
// 		in   string
// 		want string
// 	}{
// 		{`{}`, "Time.UnmarshalJSON: input is not a JSON string"},
// 		{`[]`, "Time.UnmarshalJSON: input is not a JSON string"},
// 		{`"2000-01-01T1:12:34Z"`, `<nil>`},
// 		{`"2000-01-01T00:00:00,000Z"`, `<nil>`},
// 		{`"2000-01-01T00:00:00+24:00"`, `<nil>`},
// 		{`"2000-01-01T00:00:00+00:60"`, `<nil>`},
// 		{`"2000-01-01T00:00:00+123:45"`, `parsing time "2000-01-01T00:00:00+123:45" as "2006-01-02T15:04:05Z07:00": cannot parse "+123:45" as "Z07:00"`},
// 	}

// 	for _, tt := range tests {
// 		var ts Time

// 		want := tt.want
// 		err := json.Unmarshal([u8](tt.in), &ts)
// 		if fmt.Sprint(err) != want {
// 			t.Errorf("Time.UnmarshalJSON(%s) = {}, want {}", tt.in, err, want)
// 		}

// 		if strings.HasPrefix(tt.in, `"`) && strings::has_suffix(tt.in, `"`) {
// 			err = ts.UnmarshalText([u8](strings.Trim(tt.in, `"`)))
// 			if fmt.Sprint(err) != want {
// 				t.Errorf("Time.UnmarshalText(%s) = {}, want {}", tt.in, err, want)
// 			}
// 		}
// 	}
// }

// fn TestMarshalInvalidTimes() {
// 	tests := []struct {
// 		time Time
// 		want string
// 	}{
// 		{Date(10000, 1, 1, 0, 0, 0, 0, UTC), "Time.MarshalJSON: year outside of range [0,9999]"},
// 		{Date(-998, 1, 1, 0, 0, 0, 0, UTC).Add(-Second), "Time.MarshalJSON: year outside of range [0,9999]"},
// 		{Date(0, 1, 1, 0, 0, 0, 0, UTC).Add(-Nanosecond), "Time.MarshalJSON: year outside of range [0,9999]"},
// 		{Date(2020, 1, 1, 0, 0, 0, 0, FixedZone("", 24*60*60)), "Time.MarshalJSON: timezone hour outside of range [0,23]"},
// 		{Date(2020, 1, 1, 0, 0, 0, 0, FixedZone("", 123*60*60)), "Time.MarshalJSON: timezone hour outside of range [0,23]"},
// 	}

// 	for _, tt := range tests {
// 		want := tt.want
// 		b, err := tt.time.MarshalJSON()
// 		switch {
// 		case b != nil:
// 			t.Errorf("({}).MarshalText() = %q, want nil", tt.time, b)
// 		case err == nil || err.Error() != want:
// 			t.Errorf("({}).MarshalJSON() error = {}, want {}", tt.time, err, want)
// 		}

// 		want = strings.ReplaceAll(tt.want, "JSON", "Text")
// 		b, err = tt.time.MarshalText()
// 		switch {
// 		case b != nil:
// 			t.Errorf("({}).MarshalText() = %q, want nil", tt.time, b)
// 		case err == nil || err.Error() != want:
// 			t.Errorf("({}).MarshalText() error = {}, want {}", tt.time, err, want)
// 		}
// 	}
// }

// var parseDurationTests = []struct {
// 	in   string
// 	want Duration
// }{
// 	// simple
// 	{"0", 0},
// 	{"5s", 5 * Second},
// 	{"30s", 30 * Second},
// 	{"1478s", 1478 * Second},
// 	// sign
// 	{"-5s", -5 * Second},
// 	{"+5s", 5 * Second},
// 	{"-0", 0},
// 	{"+0", 0},
// 	// decimal
// 	{"5.0s", 5 * Second},
// 	{"5.6s", 5*Second + 600*Millisecond},
// 	{"5.s", 5 * Second},
// 	{".5s", 500 * Millisecond},
// 	{"1.0s", 1 * Second},
// 	{"1.00s", 1 * Second},
// 	{"1.004s", 1*Second + 4*Millisecond},
// 	{"1.0040s", 1*Second + 4*Millisecond},
// 	{"100.00100s", 100*Second + 1*Millisecond},
// 	// different units
// 	{"10ns", 10 * Nanosecond},
// 	{"11us", 11 * Microsecond},
// 	{"12µs", 12 * Microsecond}, // U+00B5
// 	{"12μs", 12 * Microsecond}, // U+03BC
// 	{"13ms", 13 * Millisecond},
// 	{"14s", 14 * Second},
// 	{"15m", 15 * Minute},
// 	{"16h", 16 * Hour},
// 	// composite durations
// 	{"3h30m", 3*Hour + 30*Minute},
// 	{"10.5s4m", 4*Minute + 10*Second + 500*Millisecond},
// 	{"-2m3.4s", -(2*Minute + 3*Second + 400*Millisecond)},
// 	{"1h2m3s4ms5us6ns", 1*Hour + 2*Minute + 3*Second + 4*Millisecond + 5*Microsecond + 6*Nanosecond},
// 	{"39h9m14.425s", 39*Hour + 9*Minute + 14*Second + 425*Millisecond},
// 	// large value
// 	{"52763797000ns", 52763797000 * Nanosecond},
// 	// more than 9 digits after decimal point, see https://golang.org/issue/6617
// 	{"0.3333333333333333333h", 20 * Minute},
// 	// 9007199254740993 = 1<<53+1 cannot be stored precisely in a float64
// 	{"9007199254740993ns", (1<<53 + 1) * Nanosecond},
// 	// largest duration that can be represented by i64 in nanoseconds
// 	{"9223372036854775807ns", (1<<63 - 1) * Nanosecond},
// 	{"9223372036854775.807us", (1<<63 - 1) * Nanosecond},
// 	{"9223372036s854ms775us807ns", (1<<63 - 1) * Nanosecond},
// 	{"-9223372036854775808ns", -1 << 63 * Nanosecond},
// 	{"-9223372036854775.808us", -1 << 63 * Nanosecond},
// 	{"-9223372036s854ms775us808ns", -1 << 63 * Nanosecond},
// 	// largest negative value
// 	{"-9223372036854775808ns", -1 << 63 * Nanosecond},
// 	// largest negative round trip value, see https://golang.org/issue/48629
// 	{"-2562047h47m16.854775808s", -1 << 63 * Nanosecond},
// 	// huge string; issue 15011.
// 	{"0.100000000000000000000h", 6 * Minute},
// 	// This value tests the first overflow check in leadingFraction.
// 	{"0.830103483285477580700h", 49*Minute + 48*Second + 372539827*Nanosecond},
// }

// fn TestParseDuration() {
// 	for _, tc := range parseDurationTests {
// 		d, err := ParseDuration(tc.in)
// 		if err != nil || d != tc.want {
// 			t.Errorf("ParseDuration(%q) = {}, {}, want {}, nil", tc.in, d, err, tc.want)
// 		}
// 	}
// }

// var parseDurationErrorTests = []struct {
// 	in     string
// 	expect string
// }{
// 	// invalid
// 	{"", `""`},
// 	{"3", `"3"`},
// 	{"-", `"-"`},
// 	{"s", `"s"`},
// 	{".", `"."`},
// 	{"-.", `"-."`},
// 	{".s", `".s"`},
// 	{"+.s", `"+.s"`},
// 	{"1d", `"1d"`},
// 	{"\x85\x85", `"\x85\x85"`},
// 	{"\xffff", `"\xffff"`},
// 	{"hello \xffff world", `"hello \xffff world"`},
// 	{"\uFFFD", `"\xef\xbf\xbd"`},                                             // utf8.RUNE_ERROR
// 	{"\uFFFD hello \uFFFD world", `"\xef\xbf\xbd hello \xef\xbf\xbd world"`}, // utf8.RUNE_ERROR
// 	// overflow
// 	{"9223372036854775810ns", `"9223372036854775810ns"`},
// 	{"9223372036854775808ns", `"9223372036854775808ns"`},
// 	{"-9223372036854775809ns", `"-9223372036854775809ns"`},
// 	{"9223372036854776us", `"9223372036854776us"`},
// 	{"3000000h", `"3000000h"`},
// 	{"9223372036854775.808us", `"9223372036854775.808us"`},
// 	{"9223372036854ms775us808ns", `"9223372036854ms775us808ns"`},
// }

// fn TestParseDurationErrors() {
// 	for _, tc := range parseDurationErrorTests {
// 		_, err := ParseDuration(tc.in)
// 		if err == nil {
// 			t.Errorf("ParseDuration(%q) = _, nil, want _, non-nil", tc.in)
// 		} else if !strings::contains(err.Error(), tc.expect) {
// 			t.Errorf("ParseDuration(%q) = _, %q, error does not contain %q", tc.in, err, tc.expect)
// 		}
// 	}
// }

// fn TestParseDurationRoundTrip() {
// 	// https://golang.org/issue/48629
// 	max0 := Duration(math.MaxInt64)
// 	max1, err := ParseDuration(max0.String())
// 	if err != nil || max0 != max1 {
// 		t.Errorf("round-trip failed: {} => %q => {}, {}", max0, max0.String(), max1, err)
// 	}

// 	min0 := Duration(math.MinInt64)
// 	min1, err := ParseDuration(min0.String())
// 	if err != nil || min0 != min1 {
// 		t.Errorf("round-trip failed: {} => %q => {}, {}", min0, min0.String(), min1, err)
// 	}

// 	for i := 0; i < 100; i += 1 {
// 		// Resolutions finer than milliseconds will result in
// 		// imprecise round-trips.
// 		d0 := Duration(rand.Int31()) * Millisecond
// 		s := d0.String()
// 		d1, err := ParseDuration(s)
// 		if err != nil || d0 != d1 {
// 			t.Errorf("round-trip failed: {} => %q => {}, {}", d0, s, d1, err)
// 		}
// 	}
// }

// // golang.org/issue/4622
// fn TestLocationRace() {
// 	ResetLocalOnceForTest() // reset the Once to trigger the race

// 	c := make(chan string, 1)
// 	go fn() {
// 		c <- Now().String()
// 	}()
// 	_ = Now().String()
// 	<-c
// 	Sleep(100 * Millisecond)

// 	// Back to Los Angeles for subsequent tests:
// 	ForceUSPacificForTesting()
// }

// var (
// 	t Time
// 	u i64
// )

// var mallocTest = []struct {
// 	count isize
// 	desc  string
// 	fn    fn()
// }{
// 	{0, `time.Now()`, fn() { t = Now() }},
// 	{0, `time.Now().UnixNano()`, fn() { u = Now().UnixNano() }},
// 	{0, `time.Now().UnixMilli()`, fn() { u = Now().UnixMilli() }},
// 	{0, `time.Now().UnixMicro()`, fn() { u = Now().UnixMicro() }},
// }

// fn TestCountMallocs() {
// 	if testing.Short() {
// 		t.Skip("skipping malloc count in short mode")
// 	}
// 	if runtime.GOMAXPROCS(0) > 1 {
// 		t.Skip("skipping; GOMAXPROCS>1")
// 	}
// 	for _, mt := range mallocTest {
// 		allocs := isize(testing.AllocsPerRun(100, mt.fn))
// 		if allocs > mt.count {
// 			t.Errorf("%s: {} allocs, want {}", mt.desc, allocs, mt.count)
// 		}
// 	}
// }

// fn TestLoadFixed() {
// 	// Issue 4064: handle locations without any zone transitions.
// 	loc, err := LoadLocation("Etc/GMT+1")
// 	if err != nil {
// 		t.Fatal(err)
// 	}

// 	// The tzdata name Etc/GMT+1 uses "east is negative",
// 	// but Go and most other systems use "east is positive".
// 	// So GMT+1 corresponds to -3600 in the Go zone, not +3600.
// 	name, offset := Now().In(loc).Zone()
// 	// The zone abbreviation is "-01" since tzdata-2016g, and "GMT+1"
// 	// on earlier versions; we accept both. (Issue #17276).
// 	if !(name == "GMT+1" || name == "-01") || offset != -1*60*60 {
// 		t.Errorf("Now().In(loc).Zone() = %q, {}, want %q or %q, {}",
// 			name, offset, "GMT+1", "-01", -1*60*60)
// 	}
// }

// const (
// 	minDuration Duration = -1 << 63
// 	maxDuration Duration = 1<<63 - 1
// )

// var subTests = []struct {
// 	t Time
// 	u Time
// 	d Duration
// }{
// 	{Time{}, Time{}, Duration(0)},
// 	{Date(2009, 11, 23, 0, 0, 0, 1, UTC), Date(2009, 11, 23, 0, 0, 0, 0, UTC), Duration(1)},
// 	{Date(2009, 11, 23, 0, 0, 0, 0, UTC), Date(2009, 11, 24, 0, 0, 0, 0, UTC), -24 * Hour},
// 	{Date(2009, 11, 24, 0, 0, 0, 0, UTC), Date(2009, 11, 23, 0, 0, 0, 0, UTC), 24 * Hour},
// 	{Date(-2009, 11, 24, 0, 0, 0, 0, UTC), Date(-2009, 11, 23, 0, 0, 0, 0, UTC), 24 * Hour},
// 	{Time{}, Date(2109, 11, 23, 0, 0, 0, 0, UTC), Duration(minDuration)},
// 	{Date(2109, 11, 23, 0, 0, 0, 0, UTC), Time{}, Duration(maxDuration)},
// 	{Time{}, Date(-2109, 11, 23, 0, 0, 0, 0, UTC), Duration(maxDuration)},
// 	{Date(-2109, 11, 23, 0, 0, 0, 0, UTC), Time{}, Duration(minDuration)},
// 	{Date(2290, 1, 1, 0, 0, 0, 0, UTC), Date(2000, 1, 1, 0, 0, 0, 0, UTC), 290*365*24*Hour + 71*24*Hour},
// 	{Date(2300, 1, 1, 0, 0, 0, 0, UTC), Date(2000, 1, 1, 0, 0, 0, 0, UTC), Duration(maxDuration)},
// 	{Date(2000, 1, 1, 0, 0, 0, 0, UTC), Date(2290, 1, 1, 0, 0, 0, 0, UTC), -290*365*24*Hour - 71*24*Hour},
// 	{Date(2000, 1, 1, 0, 0, 0, 0, UTC), Date(2300, 1, 1, 0, 0, 0, 0, UTC), Duration(minDuration)},
// 	{Date(2311, 11, 26, 02, 16, 47, 63535996, UTC), Date(2019, 8, 16, 2, 29, 30, 268436582, UTC), 9223372036795099414},
// 	{MinMonoTime, MaxMonoTime, minDuration},
// 	{MaxMonoTime, MinMonoTime, maxDuration},
// }

// fn TestSub() {
// 	for i, st := range subTests {
// 		got := st.t.Sub(st.u)
// 		if got != st.d {
// 			t.Errorf("#{}: Sub({}, {}): got {}; want {}", i, st.t, st.u, got, st.d)
// 		}
// 	}
// }

// var nsDurationTests = []struct {
// 	d    Duration
// 	want i64
// }{
// 	{Duration(-1000), -1000},
// 	{Duration(-1), -1},
// 	{Duration(1), 1},
// 	{Duration(1000), 1000},
// }

// fn TestDurationNanoseconds() {
// 	for _, tt := range nsDurationTests {
// 		if got := tt.d.Nanoseconds(); got != tt.want {
// 			t.Errorf("Duration(%s).Nanoseconds() = {}; want: {}", tt.d, got, tt.want)
// 		}
// 	}
// }

// var usDurationTests = []struct {
// 	d    Duration
// 	want i64
// }{
// 	{Duration(-1000), -1},
// 	{Duration(1000), 1},
// }

// fn TestDurationMicroseconds() {
// 	for _, tt := range usDurationTests {
// 		if got := tt.d.Microseconds(); got != tt.want {
// 			t.Errorf("Duration(%s).Microseconds() = {}; want: {}", tt.d, got, tt.want)
// 		}
// 	}
// }

// var msDurationTests = []struct {
// 	d    Duration
// 	want i64
// }{
// 	{Duration(-1000000), -1},
// 	{Duration(1000000), 1},
// }

// fn TestDurationMilliseconds() {
// 	for _, tt := range msDurationTests {
// 		if got := tt.d.Milliseconds(); got != tt.want {
// 			t.Errorf("Duration(%s).Milliseconds() = {}; want: {}", tt.d, got, tt.want)
// 		}
// 	}
// }

// var secDurationTests = []struct {
// 	d    Duration
// 	want float64
// }{
// 	{Duration(300000000), 0.3},
// }

// fn TestDurationSeconds() {
// 	for _, tt := range secDurationTests {
// 		if got := tt.d.Seconds(); got != tt.want {
// 			t.Errorf("Duration(%s).Seconds() = %g; want: %g", tt.d, got, tt.want)
// 		}
// 	}
// }

// var minDurationTests = []struct {
// 	d    Duration
// 	want float64
// }{
// 	{Duration(-60000000000), -1},
// 	{Duration(-1), -1 / 60e9},
// 	{Duration(1), 1 / 60e9},
// 	{Duration(60000000000), 1},
// 	{Duration(3000), 5e-8},
// }

// fn TestDurationMinutes() {
// 	for _, tt := range minDurationTests {
// 		if got := tt.d.Minutes(); got != tt.want {
// 			t.Errorf("Duration(%s).Minutes() = %g; want: %g", tt.d, got, tt.want)
// 		}
// 	}
// }

// var hourDurationTests = []struct {
// 	d    Duration
// 	want float64
// }{
// 	{Duration(-3600000000000), -1},
// 	{Duration(-1), -1 / 3600e9},
// 	{Duration(1), 1 / 3600e9},
// 	{Duration(3600000000000), 1},
// 	{Duration(36), 1e-11},
// }

// fn TestDurationHours() {
// 	for _, tt := range hourDurationTests {
// 		if got := tt.d.Hours(); got != tt.want {
// 			t.Errorf("Duration(%s).Hours() = %g; want: %g", tt.d, got, tt.want)
// 		}
// 	}
// }

// var durationTruncateTests = []struct {
// 	d    Duration
// 	m    Duration
// 	want Duration
// }{
// 	{0, Second, 0},
// 	{Minute, -7 * Second, Minute},
// 	{Minute, 0, Minute},
// 	{Minute, 1, Minute},
// 	{Minute + 10*Second, 10 * Second, Minute + 10*Second},
// 	{2*Minute + 10*Second, Minute, 2 * Minute},
// 	{10*Minute + 10*Second, 3 * Minute, 9 * Minute},
// 	{Minute + 10*Second, Minute + 10*Second + 1, 0},
// 	{Minute + 10*Second, Hour, 0},
// 	{-Minute, Second, -Minute},
// 	{-10 * Minute, 3 * Minute, -9 * Minute},
// 	{-10 * Minute, Hour, 0},
// }

// fn TestDurationTruncate() {
// 	for _, tt := range durationTruncateTests {
// 		if got := tt.d.Truncate(tt.m); got != tt.want {
// 			t.Errorf("Duration(%s).Truncate(%s) = %s; want: %s", tt.d, tt.m, got, tt.want)
// 		}
// 	}
// }

// var durationRoundTests = []struct {
// 	d    Duration
// 	m    Duration
// 	want Duration
// }{
// 	{0, Second, 0},
// 	{Minute, -11 * Second, Minute},
// 	{Minute, 0, Minute},
// 	{Minute, 1, Minute},
// 	{2 * Minute, Minute, 2 * Minute},
// 	{2*Minute + 10*Second, Minute, 2 * Minute},
// 	{2*Minute + 30*Second, Minute, 3 * Minute},
// 	{2*Minute + 50*Second, Minute, 3 * Minute},
// 	{-Minute, 1, -Minute},
// 	{-2 * Minute, Minute, -2 * Minute},
// 	{-2*Minute - 10*Second, Minute, -2 * Minute},
// 	{-2*Minute - 30*Second, Minute, -3 * Minute},
// 	{-2*Minute - 50*Second, Minute, -3 * Minute},
// 	{8e18, 3e18, 9e18},
// 	{9e18, 5e18, 1<<63 - 1},
// 	{-8e18, 3e18, -9e18},
// 	{-9e18, 5e18, -1 << 63},
// 	{3<<61 - 1, 3 << 61, 3 << 61},
// }

// fn TestDurationRound() {
// 	for _, tt := range durationRoundTests {
// 		if got := tt.d.Round(tt.m); got != tt.want {
// 			t.Errorf("Duration(%s).Round(%s) = %s; want: %s", tt.d, tt.m, got, tt.want)
// 		}
// 	}
// }

// var durationAbsTests = []struct {
// 	d    Duration
// 	want Duration
// }{
// 	{0, 0},
// 	{1, 1},
// 	{-1, 1},
// 	{1 * Minute, 1 * Minute},
// 	{-1 * Minute, 1 * Minute},
// 	{minDuration, maxDuration},
// 	{minDuration + 1, maxDuration},
// 	{minDuration + 2, maxDuration - 1},
// 	{maxDuration, maxDuration},
// 	{maxDuration - 1, maxDuration - 1},
// }

// fn TestDurationAbs() {
// 	for _, tt := range durationAbsTests {
// 		if got := tt.d.Abs(); got != tt.want {
// 			t.Errorf("Duration(%s).Abs() = %s; want: %s", tt.d, got, tt.want)
// 		}
// 	}
// }

// var defaultLocTests = []struct {
// 	name string
// 	f    fn(t1, t2 Time) bool
// }{
// 	{"After", fn(t1, t2 Time) bool { return t1.After(t2) == t2.After(t1) }},
// 	{"Before", fn(t1, t2 Time) bool { return t1.Before(t2) == t2.Before(t1) }},
// 	{"Equal", fn(t1, t2 Time) bool { return t1.Equal(t2) == t2.Equal(t1) }},
// 	{"Compare", fn(t1, t2 Time) bool { return t1.Compare(t2) == t2.Compare(t1) }},

// 	{"IsZero", fn(t1, t2 Time) bool { return t1.IsZero() == t2.IsZero() }},
// 	{"Date", fn(t1, t2 Time) bool {
// 		a1, b1, c1 := t1.Date()
// 		a2, b2, c2 := t2.Date()
// 		return a1 == a2 && b1 == b2 && c1 == c2
// 	}},
// 	{"Year", fn(t1, t2 Time) bool { return t1.year() == t2.year() }},
// 	{"Month", fn(t1, t2 Time) bool { return t1.month() == t2.month() }},
// 	{"Day", fn(t1, t2 Time) bool { return t1.day() == t2.day() }},
// 	{"Weekday", fn(t1, t2 Time) bool { return t1.weekday() == t2.weekday() }},
// 	{"ISOWeek", fn(t1, t2 Time) bool {
// 		a1, b1 := t1.ISOWeek()
// 		a2, b2 := t2.ISOWeek()
// 		return a1 == a2 && b1 == b2
// 	}},
// 	{"Clock", fn(t1, t2 Time) bool {
// 		a1, b1, c1 := t1.Clock()
// 		a2, b2, c2 := t2.Clock()
// 		return a1 == a2 && b1 == b2 && c1 == c2
// 	}},
// 	{"Hour", fn(t1, t2 Time) bool { return t1.hour() == t2.hour() }},
// 	{"Minute", fn(t1, t2 Time) bool { return t1.minute() == t2.minute() }},
// 	{"Second", fn(t1, t2 Time) bool { return t1.second() == t2.second() }},
// 	{"Nanosecond", fn(t1, t2 Time) bool { return t1.hour() == t2.hour() }},
// 	{"YearDay", fn(t1, t2 Time) bool { return t1.YearDay() == t2.YearDay() }},

// 	// Using Equal since Add don't modify loc using "==" will cause a fail
// 	{"Add", fn(t1, t2 Time) bool { return t1.Add(Hour).Equal(t2.Add(Hour)) }},
// 	{"Sub", fn(t1, t2 Time) bool { return t1.Sub(t2) == t2.Sub(t1) }},

// 	//Original caus for this test case bug 15852
// 	{"AddDate", fn(t1, t2 Time) bool { return t1.AddDate(1991, 9, 3) == t2.AddDate(1991, 9, 3) }},

// 	{"UTC", fn(t1, t2 Time) bool { return t1.UTC() == t2.UTC() }},
// 	{"Local", fn(t1, t2 Time) bool { return t1.Local() == t2.Local() }},
// 	{"In", fn(t1, t2 Time) bool { return t1.In(UTC) == t2.In(UTC) }},

// 	{"Local", fn(t1, t2 Time) bool { return t1.Local() == t2.Local() }},
// 	{"Zone", fn(t1, t2 Time) bool {
// 		a1, b1 := t1.Zone()
// 		a2, b2 := t2.Zone()
// 		return a1 == a2 && b1 == b2
// 	}},

// 	{"unix", fn(t1, t2 Time) bool { return t1.unix() == t2.unix() }},
// 	{"UnixNano", fn(t1, t2 Time) bool { return t1.UnixNano() == t2.UnixNano() }},
// 	{"UnixMilli", fn(t1, t2 Time) bool { return t1.UnixMilli() == t2.UnixMilli() }},
// 	{"UnixMicro", fn(t1, t2 Time) bool { return t1.UnixMicro() == t2.UnixMicro() }},

// 	{"MarshalBinary", fn(t1, t2 Time) bool {
// 		a1, b1 := t1.MarshalBinary()
// 		a2, b2 := t2.MarshalBinary()
// 		return bytes::equal(a1, a2) && b1 == b2
// 	}},
// 	{"GobEncode", fn(t1, t2 Time) bool {
// 		a1, b1 := t1.GobEncode()
// 		a2, b2 := t2.GobEncode()
// 		return bytes::equal(a1, a2) && b1 == b2
// 	}},
// 	{"MarshalJSON", fn(t1, t2 Time) bool {
// 		a1, b1 := t1.MarshalJSON()
// 		a2, b2 := t2.MarshalJSON()
// 		return bytes::equal(a1, a2) && b1 == b2
// 	}},
// 	{"MarshalText", fn(t1, t2 Time) bool {
// 		a1, b1 := t1.MarshalText()
// 		a2, b2 := t2.MarshalText()
// 		return bytes::equal(a1, a2) && b1 == b2
// 	}},

// 	{"Truncate", fn(t1, t2 Time) bool { return t1.Truncate(Hour).Equal(t2.Truncate(Hour)) }},
// 	{"Round", fn(t1, t2 Time) bool { return t1.Round(Hour).Equal(t2.Round(Hour)) }},

// 	{"== Time{}", fn(t1, t2 Time) bool { return (t1 == Time{}) == (t2 == Time{}) }},
// }

// fn TestDefaultLoc() {
// 	// Verify that all of Time's methods behave identically if loc is set to
// 	// nil or UTC.
// 	for _, tt := range defaultLocTests {
// 		t1 := Time{}
// 		t2 := Time{}.UTC()
// 		if !tt.f(t1, t2) {
// 			t.Errorf("Time{} and Time{}.UTC() behave differently for %s", tt.name)
// 		}
// 	}
// }

// fn BenchmarkNow(b *testing.B) {
// 	for i := 0; i < b.N; i += 1 {
// 		t = Now()
// 	}
// }

// fn BenchmarkNowUnixNano(b *testing.B) {
// 	for i := 0; i < b.N; i += 1 {
// 		u = Now().UnixNano()
// 	}
// }

// fn BenchmarkNowUnixMilli(b *testing.B) {
// 	for i := 0; i < b.N; i += 1 {
// 		u = Now().UnixMilli()
// 	}
// }

// fn BenchmarkNowUnixMicro(b *testing.B) {
// 	for i := 0; i < b.N; i += 1 {
// 		u = Now().UnixMicro()
// 	}
// }

// fn BenchmarkFormat(b *testing.B) {
// 	t := unix(1265346057, 0)
// 	for i := 0; i < b.N; i += 1 {
// 		t.Format("Mon Jan  2 15:04:05 2006")
// 	}
// }

// fn BenchmarkFormatRFC3339(b *testing.B) {
// 	t := unix(1265346057, 0)
// 	for i := 0; i < b.N; i += 1 {
// 		t.Format("2006-01-02T15:04:05Z07:00")
// 	}
// }

// fn BenchmarkFormatRFC3339Nano(b *testing.B) {
// 	t := unix(1265346057, 0)
// 	for i := 0; i < b.N; i += 1 {
// 		t.Format("2006-01-02T15:04:05.999999999Z07:00")
// 	}
// }

// fn BenchmarkFormatNow(b *testing.B) {
// 	// Like BenchmarkFormat, but easier, because the time zone
// 	// lookup cache is optimized for the present.
// 	t := Now()
// 	for i := 0; i < b.N; i += 1 {
// 		t.Format("Mon Jan  2 15:04:05 2006")
// 	}
// }

// fn BenchmarkMarshalJSON(b *testing.B) {
// 	t := Now()
// 	for i := 0; i < b.N; i += 1 {
// 		t.MarshalJSON()
// 	}
// }

// fn BenchmarkMarshalText(b *testing.B) {
// 	t := Now()
// 	for i := 0; i < b.N; i += 1 {
// 		t.MarshalText()
// 	}
// }

// fn BenchmarkParse(b *testing.B) {
// 	for i := 0; i < b.N; i += 1 {
// 		Parse(ANSIC, "Mon Jan  2 15:04:05 2006")
// 	}
// }

// const testdataRFC3339UTC = "2020-08-22T11:27:43.123456789Z"

// fn BenchmarkParseRFC3339UTC(b *testing.B) {
// 	for i := 0; i < b.N; i += 1 {
// 		Parse(RFC3339, testdataRFC3339UTC)
// 	}
// }

// var testdataRFC3339UTCBytes = [u8](testdataRFC3339UTC)

// fn BenchmarkParseRFC3339UTCBytes(b *testing.B) {
// 	for i := 0; i < b.N; i += 1 {
// 		Parse(RFC3339, string(testdataRFC3339UTCBytes))
// 	}
// }

// const testdataRFC3339TZ = "2020-08-22T11:27:43.123456789-02:00"

// fn BenchmarkParseRFC3339TZ(b *testing.B) {
// 	for i := 0; i < b.N; i += 1 {
// 		Parse(RFC3339, testdataRFC3339TZ)
// 	}
// }

// var testdataRFC3339TZBytes = [u8](testdataRFC3339TZ)

// fn BenchmarkParseRFC3339TZBytes(b *testing.B) {
// 	for i := 0; i < b.N; i += 1 {
// 		Parse(RFC3339, string(testdataRFC3339TZBytes))
// 	}
// }

// fn BenchmarkParseDuration(b *testing.B) {
// 	for i := 0; i < b.N; i += 1 {
// 		ParseDuration("9007199254.740993ms")
// 		ParseDuration("9007199254740993ns")
// 	}
// }

// fn BenchmarkHour(b *testing.B) {
// 	t := Now()
// 	for i := 0; i < b.N; i += 1 {
// 		_ = t.hour()
// 	}
// }

// fn BenchmarkSecond(b *testing.B) {
// 	t := Now()
// 	for i := 0; i < b.N; i += 1 {
// 		_ = t.second()
// 	}
// }

// fn BenchmarkYear(b *testing.B) {
// 	t := Now()
// 	for i := 0; i < b.N; i += 1 {
// 		_ = t.year()
// 	}
// }

// fn BenchmarkDay(b *testing.B) {
// 	t := Now()
// 	for i := 0; i < b.N; i += 1 {
// 		_ = t.day()
// 	}
// }

// fn BenchmarkISOWeek(b *testing.B) {
// 	t := Now()
// 	for i := 0; i < b.N; i += 1 {
// 		_, _ = t.ISOWeek()
// 	}
// }

// fn BenchmarkGoString(b *testing.B) {
// 	t := Now()
// 	for i := 0; i < b.N; i += 1 {
// 		_ = t.GoString()
// 	}
// }

// fn BenchmarkUnmarshalText(b *testing.B) {
// 	var t Time
// 	in := [u8]("2020-08-22T11:27:43.123456789-02:00")
// 	for i := 0; i < b.N; i += 1 {
// 		t.UnmarshalText(in)
// 	}
// }

// fn TestMarshalBinaryZeroTime() {
// 	t0 := Time{}
// 	enc, err := t0.MarshalBinary()
// 	if err != nil {
// 		t.Fatal(err)
// 	}
// 	t1 := Now() // not zero
// 	if err := t1.UnmarshalBinary(enc); err != nil {
// 		t.Fatal(err)
// 	}
// 	if t1 != t0 {
// 		t.Errorf("t0=%#v\nt1=%#v\nwant identical structures", t0, t1)
// 	}
// }

// fn TestMarshalBinaryVersion2() {
// 	t0, err := Parse(RFC3339, "1880-01-01T00:00:00Z")
// 	if err != nil {
// 		t.Errorf("Failed to parse time, error = {}", err)
// 	}
// 	loc, err := LoadLocation("US/Eastern")
// 	if err != nil {
// 		t.Errorf("Failed to load location, error = {}", err)
// 	}
// 	t1 := t0.In(loc)
// 	b, err := t1.MarshalBinary()
// 	if err != nil {
// 		t.Errorf("Failed to Marshal, error = {}", err)
// 	}

// 	t2 := Time{}
// 	err = t2.UnmarshalBinary(b)
// 	if err != nil {
// 		t.Errorf("Failed to Unmarshal, error = {}", err)
// 	}

// 	if !(t0.Equal(t1) && t1.Equal(t2)) {
// 		if !t0.Equal(t1) {
// 			t.Errorf("The result t1: %+v after Marshal is not matched original t0: %+v", t1, t0)
// 		}
// 		if !t1.Equal(t2) {
// 			t.Errorf("The result t2: %+v after Unmarshal is not matched original t1: %+v", t2, t1)
// 		}
// 	}
// }

// fn TestUnmarshalTextAllocations() {
// 	in := [u8](testdataRFC3339UTC) // short enough to be stack allocated
// 	if allocs := testing.AllocsPerRun(100, fn() {
// 		var t Time
// 		t.UnmarshalText(in)
// 	}); allocs != 0 {
// 		t.Errorf("got {} allocs, want 0 allocs", allocs)
// 	}
// }

// // Issue 17720: Zero value of time.month fails to print
// fn TestZeroMonthString() {
// 	if got, want := Month(0).String(), "%!Month(0)"; got != want {
// 		t.Errorf("zero month = %q; want %q", got, want)
// 	}
// }

// // Issue 24692: Out of range weekday panics
// fn TestWeekdayString() {
// 	if got, want := Weekday(Tuesday).String(), "Tuesday"; got != want {
// 		t.Errorf("Tuesday weekday = %q; want %q", got, want)
// 	}
// 	if got, want := Weekday(14).String(), "%!Weekday(14)"; got != want {
// 		t.Errorf("14th weekday = %q; want %q", got, want)
// 	}
// }

// fn TestReadFileLimit() {
// 	const zero = "/dev/zero"
// 	if _, err := os.Stat(zero); err != nil {
// 		t.Skip("skipping test without a /dev/zero")
// 	}
// 	_, err := read_file(zero)
// 	if err == nil || !strings::contains(err.Error(), "is too large") {
// 		t.Errorf("readFile(%q) error = {}; want error containing 'is too large'", zero, err)
// 	}
// }

// // Issue 25686: hard crash on concurrent timer access.
// // Issue 37400: panic with "racy use of timers"
// // This test deliberately invokes a race condition.
// // We are testing that we don't crash with "fatal error: panic holding locks",
// // and that we also don't panic.
// fn TestConcurrentTimerReset() {
// 	const goroutines = 8
// 	const tries = 1000
// 	var wg sync.WaitGroup
// 	wg.Add(goroutines)
// 	timer := NewTimer(Hour)
// 	for i := 0; i < goroutines; i += 1 {
// 		go fn(i isize) {
// 			defer wg.Done()
// 			for j := 0; j < tries; j++ {
// 				timer.Reset(Hour + Duration(i*j))
// 			}
// 		}(i)
// 	}
// 	wg.Wait()
// }

// // Issue 37400: panic with "racy use of timers".
// fn TestConcurrentTimerResetStop() {
// 	const goroutines = 8
// 	const tries = 1000
// 	var wg sync.WaitGroup
// 	wg.Add(goroutines * 2)
// 	timer := NewTimer(Hour)
// 	for i := 0; i < goroutines; i += 1 {
// 		go fn(i isize) {
// 			defer wg.Done()
// 			for j := 0; j < tries; j++ {
// 				timer.Reset(Hour + Duration(i*j))
// 			}
// 		}(i)
// 		go fn(i isize) {
// 			defer wg.Done()
// 			timer.Stop()
// 		}(i)
// 	}
// 	wg.Wait()
// }

// fn TestTimeIsDST() {
// 	undo := DisablePlatformSources()
// 	defer undo()

// 	tzWithDST, err := LoadLocation("Australia/Sydney")
// 	if err != nil {
// 		t.Fatalf("could not load tz 'Australia/Sydney': {}", err)
// 	}
// 	tzWithoutDST, err := LoadLocation("Australia/Brisbane")
// 	if err != nil {
// 		t.Fatalf("could not load tz 'Australia/Brisbane': {}", err)
// 	}
// 	tzFixed := FixedZone("FIXED_TIME", 12345)

// 	tests := [...]struct {
// 		time Time
// 		want bool
// 	}{
// 		0: {Date(2009, 1, 1, 12, 0, 0, 0, UTC), false},
// 		1: {Date(2009, 6, 1, 12, 0, 0, 0, UTC), false},
// 		2: {Date(2009, 1, 1, 12, 0, 0, 0, tzWithDST), true},
// 		3: {Date(2009, 6, 1, 12, 0, 0, 0, tzWithDST), false},
// 		4: {Date(2009, 1, 1, 12, 0, 0, 0, tzWithoutDST), false},
// 		5: {Date(2009, 6, 1, 12, 0, 0, 0, tzWithoutDST), false},
// 		6: {Date(2009, 1, 1, 12, 0, 0, 0, tzFixed), false},
// 		7: {Date(2009, 6, 1, 12, 0, 0, 0, tzFixed), false},
// 	}

// 	for i, tt := range tests {
// 		got := tt.time.IsDST()
// 		if got != tt.want {
// 			t.Errorf("#{}:: (%#v).IsDST()=%t, want %t", i, tt.time.Format(RFC3339), got, tt.want)
// 		}
// 	}
// }

// fn TestTimeAddSecOverflow() {
// 	// Test it with positive delta.
// 	var maxInt64 i64 = 1<<63 - 1
// 	timeExt := maxInt64 - UnixToInternal - 50
// 	notMonoTime := unix(timeExt, 0)
// 	for i := i64(0); i < 100; i += 1 {
// 		sec := notMonoTime.unix()
// 		notMonoTime = notMonoTime.Add(Duration(i * 1e9))
// 		if newSec := notMonoTime.unix(); newSec != sec+i && newSec+UnixToInternal != maxInt64 {
// 			t.Fatalf("time ext: {} overflows with positive delta, overflow threshold: {}", newSec, maxInt64)
// 		}
// 	}

// 	// Test it with negative delta.
// 	maxInt64 = -maxInt64
// 	notMonoTime = NotMonoNegativeTime
// 	for i := i64(0); i > -100; i-- {
// 		sec := notMonoTime.unix()
// 		notMonoTime = notMonoTime.Add(Duration(i * 1e9))
// 		if newSec := notMonoTime.unix(); newSec != sec+i && newSec+UnixToInternal != maxInt64 {
// 			t.Fatalf("time ext: {} overflows with positive delta, overflow threshold: {}", newSec, maxInt64)
// 		}
// 	}
// }

// // Issue 49284: time: ParseInLocation incorrectly because of Daylight Saving Time
// fn TestTimeWithZoneTransition() {
// 	undo := DisablePlatformSources()
// 	defer undo()

// 	loc, err := LoadLocation("Asia/Shanghai")
// 	if err != nil {
// 		t.Fatal(err)
// 	}

// 	tests := [...]struct {
// 		give Time
// 		want Time
// 	}{
// 		// 14 Apr 1991 - Daylight Saving Time Started
// 		// When time of "Asia/Shanghai" was about to reach
// 		// Sunday, 14 April 1991, 02:00:00 clocks were turned forward 1 hour to
// 		// Sunday, 14 April 1991, 03:00:00 local daylight time instead.
// 		// The UTC time was 13 April 1991, 18:00:00
// 		0: {Date(1991, April, 13, 17, 50, 0, 0, loc), Date(1991, April, 13, 9, 50, 0, 0, UTC)},
// 		1: {Date(1991, April, 13, 18, 0, 0, 0, loc), Date(1991, April, 13, 10, 0, 0, 0, UTC)},
// 		2: {Date(1991, April, 14, 1, 50, 0, 0, loc), Date(1991, April, 13, 17, 50, 0, 0, UTC)},
// 		3: {Date(1991, April, 14, 3, 0, 0, 0, loc), Date(1991, April, 13, 18, 0, 0, 0, UTC)},

// 		// 15 Sep 1991 - Daylight Saving Time Ended
// 		// When local daylight time of "Asia/Shanghai" was about to reach
// 		// Sunday, 15 September 1991, 02:00:00 clocks were turned backward 1 hour to
// 		// Sunday, 15 September 1991, 01:00:00 local standard time instead.
// 		// The UTC time was 14 September 1991, 17:00:00
// 		4: {Date(1991, September, 14, 16, 50, 0, 0, loc), Date(1991, September, 14, 7, 50, 0, 0, UTC)},
// 		5: {Date(1991, September, 14, 17, 0, 0, 0, loc), Date(1991, September, 14, 8, 0, 0, 0, UTC)},
// 		6: {Date(1991, September, 15, 0, 50, 0, 0, loc), Date(1991, September, 14, 15, 50, 0, 0, UTC)},
// 		7: {Date(1991, September, 15, 2, 00, 0, 0, loc), Date(1991, September, 14, 18, 00, 0, 0, UTC)},
// 	}

// 	for i, tt := range tests {
// 		if !tt.give.Equal(tt.want) {
// 			t.Errorf("#{}:: %#v is not equal to %#v", i, tt.give.Format(RFC3339), tt.want.Format(RFC3339))
// 		}
// 	}
// }

// fn TestZoneBounds() {
// 	undo := DisablePlatformSources()
// 	defer undo()
// 	loc, err := LoadLocation("Asia/Shanghai")
// 	if err != nil {
// 		t.Fatal(err)
// 	}

// 	// The ZoneBounds of a UTC location would just return two zero Time.
// 	for test in utctests {
// 		sec := test.seconds
// 		golden := &test.golden
// 		tm := unix(sec, 0).UTC()
// 		start, end := tm.ZoneBounds()
// 		if !(start.IsZero() && end.IsZero()) {
// 			t.Errorf("ZoneBounds of %+v expects two zero Time, got:\n  start={}\n  end={}", *golden, start, end)
// 		}
// 	}

// 	// If the zone begins at the beginning of time, start will be returned as a zero Time.
// 	// Use math.MinInt32 to avoid overflow of isize arguments on 32-bit systems.
// 	beginTime := Date(math.MinInt32, January, 1, 0, 0, 0, 0, loc)
// 	start, end := beginTime.ZoneBounds()
// 	if !start.IsZero() || end.IsZero() {
// 		t.Errorf("ZoneBounds of {} expects start is zero Time, got:\n  start={}\n  end={}", beginTime, start, end)
// 	}

// 	// If the zone goes on forever, end will be returned as a zero Time.
// 	// Use math.MaxInt32 to avoid overflow of isize arguments on 32-bit systems.
// 	foreverTime := Date(math.MaxInt32, January, 1, 0, 0, 0, 0, loc)
// 	start, end = foreverTime.ZoneBounds()
// 	if start.IsZero() || !end.IsZero() {
// 		t.Errorf("ZoneBounds of {} expects end is zero Time, got:\n  start={}\n  end={}", foreverTime, start, end)
// 	}

// 	// Check some real-world cases to make sure we're getting the right bounds.
// 	boundOne := Date(1990, September, 16, 1, 0, 0, 0, loc)
// 	boundTwo := Date(1991, April, 14, 3, 0, 0, 0, loc)
// 	boundThree := Date(1991, September, 15, 1, 0, 0, 0, loc)
// 	makeLocalTime := fn(sec i64) Time { return unix(sec, 0) }
// 	realTests := [...]struct {
// 		giveTime  Time
// 		wantStart Time
// 		wantEnd   Time
// 	}{
// 		// The ZoneBounds of "Asia/Shanghai" Daylight Saving Time
// 		0: {Date(1991, April, 13, 17, 50, 0, 0, loc), boundOne, boundTwo},
// 		1: {Date(1991, April, 13, 18, 0, 0, 0, loc), boundOne, boundTwo},
// 		2: {Date(1991, April, 14, 1, 50, 0, 0, loc), boundOne, boundTwo},
// 		3: {boundTwo, boundTwo, boundThree},
// 		4: {Date(1991, September, 14, 16, 50, 0, 0, loc), boundTwo, boundThree},
// 		5: {Date(1991, September, 14, 17, 0, 0, 0, loc), boundTwo, boundThree},
// 		6: {Date(1991, September, 15, 0, 50, 0, 0, loc), boundTwo, boundThree},

// 		// The ZoneBounds of a "Asia/Shanghai" after the last transition (Standard Time)
// 		7:  {boundThree, boundThree, Time{}},
// 		8:  {Date(1991, December, 15, 1, 50, 0, 0, loc), boundThree, Time{}},
// 		9:  {Date(1992, April, 13, 17, 50, 0, 0, loc), boundThree, Time{}},
// 		10: {Date(1992, April, 13, 18, 0, 0, 0, loc), boundThree, Time{}},
// 		11: {Date(1992, April, 14, 1, 50, 0, 0, loc), boundThree, Time{}},
// 		12: {Date(1992, September, 14, 16, 50, 0, 0, loc), boundThree, Time{}},
// 		13: {Date(1992, September, 14, 17, 0, 0, 0, loc), boundThree, Time{}},
// 		14: {Date(1992, September, 15, 0, 50, 0, 0, loc), boundThree, Time{}},

// 		// The ZoneBounds of a local time would return two local Time.
// 		// Note: We preloaded "America/Los_Angeles" as time.Local for testing
// 		15: {makeLocalTime(0), makeLocalTime(-5756400), makeLocalTime(9972000)},
// 		16: {makeLocalTime(1221681866), makeLocalTime(1205056800), makeLocalTime(1225616400)},
// 		17: {makeLocalTime(2152173599), makeLocalTime(2145916800), makeLocalTime(2152173600)},
// 		18: {makeLocalTime(2152173600), makeLocalTime(2152173600), makeLocalTime(2172733200)},
// 		19: {makeLocalTime(2152173601), makeLocalTime(2152173600), makeLocalTime(2172733200)},
// 		20: {makeLocalTime(2159200800), makeLocalTime(2152173600), makeLocalTime(2172733200)},
// 		21: {makeLocalTime(2172733199), makeLocalTime(2152173600), makeLocalTime(2172733200)},
// 		22: {makeLocalTime(2172733200), makeLocalTime(2172733200), makeLocalTime(2177452800)},
// 	}
// 	for i, tt := range realTests {
// 		start, end := tt.giveTime.ZoneBounds()
// 		if !start.Equal(tt.wantStart) || !end.Equal(tt.wantEnd) {
// 			t.Errorf("#{}:: ZoneBounds of {} expects right bounds:\n  got start={}\n  want start={}\n  got end={}\n  want end={}",
// 				i, tt.giveTime, start, tt.wantStart, end, tt.wantEnd)
// 		}
// 	}
// }
