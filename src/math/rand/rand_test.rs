// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use super::{new_source, Rand};

// import (
// 	"bytes"
// 	"errors"
// 	"fmt"
// 	"internal/testenv"
// 	"io"
// 	"math"
// 	. "math/rand"
// 	"os"
// 	"runtime"
// 	"testing"
// 	"testing/iotest"
// )

const NUM_TEST_SAMPLES: usize = 10000;

// var rn, kn, wn, fn = GetNormalDistributionParameters()
// var re, ke, we, fe = GetExponentialDistributionParameters()

struct StatsResults {
    mean: f64,
    stddev: f64,
    close_enough: f64,
    max_error: f64,
}

fn near_equal(a: f64, b: f64, close_enough: f64, max_error: f64) -> bool {
    let abs_diff = f64::abs(a - b);
    if abs_diff < close_enough {
        // Necessary when one value is zero and one value is close to zero.
        return true;
    }
    abs_diff / f64::abs(a).max(f64::abs(b)) < max_error
}

const TEST_SEEDS: &[i64] = &[1, 1754801282, 1698661970, 1550503961];

/// check_similar_distribution returns success if the mean and stddev of the
/// two StatsResults are similar.
fn check_similar_distribution(this: &StatsResults, expected: &StatsResults) {
    if !near_equal(
        this.mean,
        expected.mean,
        expected.close_enough,
        expected.max_error,
    ) {
        panic!(
            "mean {} != {} (allowed error {}, {})",
            this.mean, expected.mean, expected.close_enough, expected.max_error
        );
    }
    if !near_equal(
        this.stddev,
        expected.stddev,
        expected.close_enough,
        expected.max_error,
    ) {
        panic!(
            "stddev {} != {} (allowed error {}, {})",
            this.stddev, expected.stddev, expected.close_enough, expected.max_error
        );
    }
}

fn get_stats_results(samples: &[f64]) -> StatsResults {
    let mut sum = 0_f64;
    let mut squaresum = 0_f64;
    for s in samples {
        sum += s;
        squaresum += s * s;
    }
    let mean = sum / (samples.len() as f64);
    StatsResults {
        mean,
        stddev: f64::sqrt(squaresum / (samples.len() as f64) - mean * mean),
        close_enough: 0.0,
        max_error: 0.0,
    }
}

fn check_sample_distribution(samples: &[f64], expected: &StatsResults) {
    let actual = get_stats_results(samples);
    check_similar_distribution(&actual, expected);
}

fn check_sample_slice_distributions(samples: &[f64], nslices: usize, expected: &StatsResults) {
    let chunk = samples.len() / nslices;
    for i in 0..nslices {
        let low = i * chunk;
        let high = if i == nslices - 1 {
            samples.len() - 1
        } else {
            (i + 1) * chunk
        };
        check_sample_distribution(&samples[low..high], expected);
    }
}

//
// Normal distribution tests
//

fn generate_normal_samples(nsamples: usize, mean: f64, stddev: f64, seed: i64) -> Vec<f64> {
    let mut r = Rand::new(new_source(seed));
    let mut samples = Vec::new();
    for _ in 0..nsamples {
        samples.push(r.norm_float64() * stddev + mean);
    }
    samples
}

fn test_normal_distribution(nsamples: usize, mean: f64, stddev: f64, seed: i64) {
    //fmt.Printf("testing nsamples={} mean={} stddev={} seed={}\n", nsamples, mean, stddev, seed);

    let samples = generate_normal_samples(nsamples, mean, stddev, seed);
    let error_scale = stddev.max(1.0); // Error scales with stddev
    let expected = StatsResults {
        mean,
        stddev,
        close_enough: 0.10 * error_scale,
        max_error: 0.08 * error_scale,
    };

    // Make sure that the entire set matches the expected distribution.
    check_sample_distribution(&samples, &expected);

    // Make sure that each half of the set matches the expected distribution.
    check_sample_slice_distributions(&samples, 2, &expected);

    // Make sure that each 7th of the set matches the expected distribution.
    check_sample_slice_distributions(&samples, 7, &expected);
}

// Actual tests

#[test]
fn test_standard_normal_values() {
    for seed in TEST_SEEDS {
        test_normal_distribution(NUM_TEST_SAMPLES, 0.0, 1.0, *seed);
    }
}

// #[test]
// fn TestNonStandardNormalValues() {
// 	sdmax := 1000.0
// 	mmax := 1000.0
// 	if testing.Short() {
// 		sdmax = 5
// 		mmax = 5
// 	}
// 	for sd := 0.5; sd < sdmax; sd *= 2 {
// 		for m := 0.5; m < mmax; m *= 2 {
// 			for seed in TEST_SEEDS {
// 				test_normal_distribution(t, NUM_TEST_SAMPLES, m, sd, seed)
// 				if testing.Short() {
// 					break
// 				}
// 			}
// 		}
// 	}
// }

// //
// // Exponential distribution tests
// //

// fn generateExponentialSamples(nsamples isize, rate float64, seed int64) &[f64] {
// 	r := New(NewSource(seed))
// 	samples := make(&[f64], nsamples)
// 	for i := range samples {
// 		samples[i] = r.ExpFloat64() / rate
// 	}
// 	return samples
// }

// fn testExponentialDistribution(t *testing.T, nsamples isize, rate float64, seed int64) {
// 	//fmt.Printf("testing nsamples={} rate={} seed={}\n", nsamples, rate, seed);

// 	mean := 1 / rate
// 	stddev := mean

// 	samples := generateExponentialSamples(nsamples, rate, seed)
// 	errorScale := max(1.0, 1/rate) // Error scales with the inverse of the rate
// 	expected := &StatsResults{mean, stddev, 0.10 * errorScale, 0.20 * errorScale}

// 	// Make sure that the entire set matches the expected distribution.
// 	check_sample_distribution(t, samples, expected)

// 	// Make sure that each half of the set matches the expected distribution.
// 	check_sample_slice_distributions(t, samples, 2, expected)

// 	// Make sure that each 7th of the set matches the expected distribution.
// 	check_sample_slice_distributions(t, samples, 7, expected)
// }

// // Actual tests

// #[test]
// fn TestStandardExponentialValues() {
// 	for seed in TEST_SEEDS {
// 		testExponentialDistribution(t, NUM_TEST_SAMPLES, 1, seed)
// 	}
// }

// #[test]
// fn TestNonStandardExponentialValues() {
// 	for rate := 0.05; rate < 10; rate *= 2 {
// 		for seed in TEST_SEEDS {
// 			testExponentialDistribution(t, NUM_TEST_SAMPLES, rate, seed)
// 			if testing.Short() {
// 				break
// 			}
// 		}
// 	}
// }

// //
// // Table generation tests
// //

// fn initNorm() (testKn []uint32, testWn, testFn []float32) {
// 	const m1 = 1 << 31
// 	var (
// 		dn float64 = rn
// 		tn         = dn
// 		vn float64 = 9.91256303526217e-3
// 	)

// 	testKn = make([]uint32, 128)
// 	testWn = make([]float32, 128)
// 	testFn = make([]float32, 128)

// 	q := vn / math.Exp(-0.5*dn*dn)
// 	testKn[0] = uint32((dn / q) * m1)
// 	testKn[1] = 0
// 	testWn[0] = float32(q / m1)
// 	testWn[127] = float32(dn / m1)
// 	testFn[0] = 1.0
// 	testFn[127] = float32(math.Exp(-0.5 * dn * dn))
// 	for i := 126; i >= 1; i-- {
// 		dn = math.Sqrt(-2.0 * math.Log(vn/dn+math.Exp(-0.5*dn*dn)))
// 		testKn[i+1] = uint32((dn / tn) * m1)
// 		tn = dn
// 		testFn[i] = float32(math.Exp(-0.5 * dn * dn))
// 		testWn[i] = float32(dn / m1)
// 	}
// 	return
// }

// fn initExp() (testKe []uint32, testWe, testFe []float32) {
// 	const m2 = 1 << 32
// 	var (
// 		de float64 = re
// 		te         = de
// 		ve float64 = 3.9496598225815571993e-3
// 	)

// 	testKe = make([]uint32, 256)
// 	testWe = make([]float32, 256)
// 	testFe = make([]float32, 256)

// 	q := ve / math.Exp(-de)
// 	testKe[0] = uint32((de / q) * m2)
// 	testKe[1] = 0
// 	testWe[0] = float32(q / m2)
// 	testWe[255] = float32(de / m2)
// 	testFe[0] = 1.0
// 	testFe[255] = float32(math.Exp(-de))
// 	for i := 254; i >= 1; i-- {
// 		de = -math.Log(ve/de + math.Exp(-de))
// 		testKe[i+1] = uint32((de / te) * m2)
// 		te = de
// 		testFe[i] = float32(math.Exp(-de))
// 		testWe[i] = float32(de / m2)
// 	}
// 	return
// }

// // compareUint32Slices returns the first index where the two slices
// // disagree, or <0 if the lengths are the same and all elements
// // are identical.
// fn compareUint32Slices(s1, s2 []uint32) isize {
// 	if len(s1) != len(s2) {
// 		if len(s1) > len(s2) {
// 			return len(s2) + 1
// 		}
// 		return len(s1) + 1
// 	}
// 	for i := range s1 {
// 		if s1[i] != s2[i] {
// 			return i
// 		}
// 	}
// 	return -1
// }

// // compareFloat32Slices returns the first index where the two slices
// // disagree, or <0 if the lengths are the same and all elements
// // are identical.
// fn compareFloat32Slices(s1, s2 []float32) isize {
// 	if len(s1) != len(s2) {
// 		if len(s1) > len(s2) {
// 			return len(s2) + 1
// 		}
// 		return len(s1) + 1
// 	}
// 	for i := range s1 {
// 		if !near_equal(float64(s1[i]), float64(s2[i]), 0, 1e-7) {
// 			return i
// 		}
// 	}
// 	return -1
// }

// #[test]
// fn TestNormTables() {
// 	testKn, testWn, testFn := initNorm()
// 	if i := compareUint32Slices(kn[0:], testKn); i >= 0 {
// 		t.Errorf("kn disagrees at index {}; {} != {}", i, kn[i], testKn[i])
// 	}
// 	if i := compareFloat32Slices(wn[0:], testWn); i >= 0 {
// 		t.Errorf("wn disagrees at index {}; {} != {}", i, wn[i], testWn[i])
// 	}
// 	if i := compareFloat32Slices(fn[0:], testFn); i >= 0 {
// 		t.Errorf("fn disagrees at index {}; {} != {}", i, fn[i], testFn[i])
// 	}
// }

// #[test]
// fn TestExpTables() {
// 	testKe, testWe, testFe := initExp()
// 	if i := compareUint32Slices(ke[0:], testKe); i >= 0 {
// 		t.Errorf("ke disagrees at index {}; {} != {}", i, ke[i], testKe[i])
// 	}
// 	if i := compareFloat32Slices(we[0:], testWe); i >= 0 {
// 		t.Errorf("we disagrees at index {}; {} != {}", i, we[i], testWe[i])
// 	}
// 	if i := compareFloat32Slices(fe[0:], testFe); i >= 0 {
// 		t.Errorf("fe disagrees at index {}; {} != {}", i, fe[i], testFe[i])
// 	}
// }

// fn hasSlowFloatingPoint() bool {
// 	switch runtime.GOARCH {
// 	case "arm":
// 		return os.Getenv("GOARM") == "5"
// 	case "mips", "mipsle", "mips64", "mips64le":
// 		// Be conservative and assume that all mips boards
// 		// have emulated floating point.
// 		// TODO: detect what it actually has.
// 		return true
// 	}
// 	return false
// }

#[test]
fn test_float32() {
    // For issue 6721, the problem came after 7533753 calls, so check 10e6.
    let num = 10_000_000;
    // 	// But do the full amount only on builders (not locally).
    // 	// But ARM5 floating point emulation is slow (Issue 10749), so
    // 	// do less for that builder:
    // 	if testing.Short() && (testenv.Builder() == "" || hasSlowFloatingPoint()) {
    // 		num /= 100 // 1.72 seconds instead of 172 seconds
    // 	}

    let mut r = Rand::new(new_source(1));
    for ct in 0..num {
        let f = r.float32();
        assert!(
            f < 1.0,
            "float32() should be in range [0,1). ct: {}, f: {}",
            ct,
            f
        );
    }
}

// fn testReadUniformity(t *testing.T, n isize, seed int64) {
// 	r := New(NewSource(seed))
// 	buf := make([]byte, n)
// 	nRead, err := r.Read(buf)
// 	if err != nil {
// 		t.Errorf("Read err {}", err)
// 	}
// 	if nRead != n {
// 		t.Errorf("Read returned unexpected n; %d != %d", nRead, n)
// 	}

// 	// Expect a uniform distribution of byte values, which lie in [0, 255].
// 	var (
// 		mean       = 255.0 / 2
// 		stddev     = 256.0 / math.Sqrt(12.0)
// 		errorScale = stddev / math.Sqrt(float64(n))
// 	)

// 	expected := &StatsResults{mean, stddev, 0.10 * errorScale, 0.08 * errorScale}

// 	// Cast bytes as floats to use the common distribution-validity checks.
// 	samples := make(&[f64], n)
// 	for i, val := range buf {
// 		samples[i] = float64(val)
// 	}
// 	// Make sure that the entire set matches the expected distribution.
// 	check_sample_distribution(t, samples, expected)
// }

// #[test]
// fn TestReadUniformity() {
// 	testBufferSizes := []isize{
// 		2, 4, 7, 64, 1024, 1 << 16, 1 << 20,
// 	}
// 	for seed in TEST_SEEDS {
// 		for _, n := range testBufferSizes {
// 			testReadUniformity(t, n, seed)
// 		}
// 	}
// }

// #[test]
// fn TestReadEmpty() {
// 	r := New(NewSource(1))
// 	buf := make([]byte, 0)
// 	n, err := r.Read(buf)
// 	if err != nil {
// 		t.Errorf("Read err into empty buffer; {}", err)
// 	}
// 	if n != 0 {
// 		t.Errorf("Read into empty buffer returned unexpected n of %d", n)
// 	}
// }

// #[test]
// fn TestReadByOneByte() {
// 	r := New(NewSource(1))
// 	b1 := make([]byte, 100)
// 	_, err := io.ReadFull(iotest.OneByteReader(r), b1)
// 	if err != nil {
// 		t.Errorf("read by one byte: {}", err)
// 	}
// 	r = New(NewSource(1))
// 	b2 := make([]byte, 100)
// 	_, err = r.Read(b2)
// 	if err != nil {
// 		t.Errorf("read: {}", err)
// 	}
// 	if !bytes.Equal(b1, b2) {
// 		t.Errorf("read by one byte vs single read:\n%x\n%x", b1, b2)
// 	}
// }

// #[test]
// fn TestReadSeedReset() {
// 	r := New(NewSource(42))
// 	b1 := make([]byte, 128)
// 	_, err := r.Read(b1)
// 	if err != nil {
// 		t.Errorf("read: {}", err)
// 	}
// 	r.Seed(42)
// 	b2 := make([]byte, 128)
// 	_, err = r.Read(b2)
// 	if err != nil {
// 		t.Errorf("read: {}", err)
// 	}
// 	if !bytes.Equal(b1, b2) {
// 		t.Errorf("mismatch after re-seed:\n%x\n%x", b1, b2)
// 	}
// }

// #[test]
// fn TestShuffleSmall() {
// 	// Check that Shuffle allows n=0 and n=1, but that swap is never called for them.
// 	r := New(NewSource(1))
// 	for n := 0; n <= 1; n++ {
// 		r.Shuffle(n, fn(i, j isize) { t.Fatalf("swap called, n=%d i=%d j=%d", n, i, j) })
// 	}
// }

// // encodePerm converts from a permuted slice of length n, such as Perm generates, to an isize in [0, n!).
// // See https://en.wikipedia.org/wiki/Lehmer_code.
// // encodePerm modifies the input slice.
// fn encodePerm(s []isize) isize {
// 	// Convert to Lehmer code.
// 	for i, x := range s {
// 		r := s[i+1:]
// 		for j, y := range r {
// 			if y > x {
// 				r[j]--
// 			}
// 		}
// 	}
// 	// Convert to isize in [0, n!).
// 	m := 0
// 	fact := 1
// 	for i := len(s) - 1; i >= 0; i-- {
// 		m += s[i] * fact
// 		fact *= len(s) - i
// 	}
// 	return m
// }

// #[test]// // TestUniformFactorial tests several ways of generating a uniform value in [0, n!).
// fn TestUniformFactorial() {
// 	r := New(NewSource(TEST_SEEDS[0]))
// 	top := 6
// 	if testing.Short() {
// 		top = 3
// 	}

// #[test]// 	for n := 3; n <= top; n++ {
// 		t.Run(fmt.Sprintf("n=%d", n), fn() {
// 			// Calculate n!.
// 			nfact := 1
// 			for i := 2; i <= n; i++ {
// 				nfact *= i
// 			}

// 			// Test a few different ways to generate a uniform distribution.
// 			p := make([]isize, n) // re-usable slice for Shuffle generator
// 			tests := [...]struct {
// 				name string
// 				fn   fn() isize
// 			}{
// 				{name: "Int31n", fn: fn() isize { return isize(r.Int31n(int32(nfact))) }},
// 				{name: "int31n", fn: fn() isize { return isize(Int31nForTest(r, int32(nfact))) }},
// 				{name: "Perm", fn: fn() isize { return encodePerm(r.Perm(n)) }},
// 				{name: "Shuffle", fn: fn() isize {
// 					// Generate permutation using Shuffle.
// 					for i := range p {
// 						p[i] = i
// 					}
// 					r.Shuffle(n, fn(i, j isize) { p[i], p[j] = p[j], p[i] })
// 					return encodePerm(p)
// 				}},
// 			}

// #[test]// 			for _, test := range tests {
// 				t.Run(test.name, fn() {
// 					// Gather chi-squared values and check that they follow
// 					// the expected normal distribution given n!-1 degrees of freedom.
// 					// See https://en.wikipedia.org/wiki/Pearson%27s_chi-squared_test and
// 					// https://www.johndcook.com/Beautiful_Testing_ch10.pdf.
// 					nsamples := 10 * nfact
// 					if nsamples < 200 {
// 						nsamples = 200
// 					}
// 					samples := make(&[f64], nsamples)
// 					for i := range samples {
// 						// Generate some uniformly distributed values and count their occurrences.
// 						const iters = 1000
// 						counts := make([]isize, nfact)
// 						for i := 0; i < iters; i++ {
// 							counts[test.fn()]++
// 						}
// 						// Calculate chi-squared and add to samples.
// 						want := iters / float64(nfact)
// 						var χ2 float64
// 						for _, have := range counts {
// 							err := float64(have) - want
// 							χ2 += err * err
// 						}
// 						χ2 /= want
// 						samples[i] = χ2
// 					}

// 					// Check that our samples approximate the appropriate normal distribution.
// 					dof := float64(nfact - 1)
// 					expected := &StatsResults{mean: dof, stddev: math.Sqrt(2 * dof)}
// 					errorScale := max(1.0, expected.stddev)
// 					expected.close_enough = 0.10 * errorScale
// 					expected.max_error = 0.08 // TODO: What is the right value here? See issue 21211.
// 					check_sample_distribution(t, samples, expected)
// 				})
// 			}
// 		})
// 	}
// }

// // Benchmarks

// fn BenchmarkInt63Threadsafe(b *testing.B) {
// 	for n := b.N; n > 0; n-- {
// 		Int63()
// 	}
// }

// fn BenchmarkInt63ThreadsafeParallel(b *testing.B) {
// 	b.RunParallel(fn(pb *testing.PB) {
// 		for pb.Next() {
// 			Int63()
// 		}
// 	})
// }

// fn BenchmarkInt63Unthreadsafe(b *testing.B) {
// 	r := New(NewSource(1))
// 	for n := b.N; n > 0; n-- {
// 		r.Int63()
// 	}
// }

// fn BenchmarkIntn1000(b *testing.B) {
// 	r := New(NewSource(1))
// 	for n := b.N; n > 0; n-- {
// 		r.Intn(1000)
// 	}
// }

// fn BenchmarkInt63n1000(b *testing.B) {
// 	r := New(NewSource(1))
// 	for n := b.N; n > 0; n-- {
// 		r.Int63n(1000)
// 	}
// }

// fn BenchmarkInt31n1000(b *testing.B) {
// 	r := New(NewSource(1))
// 	for n := b.N; n > 0; n-- {
// 		r.Int31n(1000)
// 	}
// }

// fn BenchmarkFloat32(b *testing.B) {
// 	r := New(NewSource(1))
// 	for n := b.N; n > 0; n-- {
// 		r.Float32()
// 	}
// }

// fn BenchmarkFloat64(b *testing.B) {
// 	r := New(NewSource(1))
// 	for n := b.N; n > 0; n-- {
// 		r.Float64()
// 	}
// }

// fn BenchmarkPerm3(b *testing.B) {
// 	r := New(NewSource(1))
// 	for n := b.N; n > 0; n-- {
// 		r.Perm(3)
// 	}
// }

// fn BenchmarkPerm30(b *testing.B) {
// 	r := New(NewSource(1))
// 	for n := b.N; n > 0; n-- {
// 		r.Perm(30)
// 	}
// }

// fn BenchmarkPerm30ViaShuffle(b *testing.B) {
// 	r := New(NewSource(1))
// 	for n := b.N; n > 0; n-- {
// 		p := make([]isize, 30)
// 		for i := range p {
// 			p[i] = i
// 		}
// 		r.Shuffle(30, fn(i, j isize) { p[i], p[j] = p[j], p[i] })
// 	}
// }

// // BenchmarkShuffleOverhead uses a minimal swap function
// // to measure just the shuffling overhead.
// fn BenchmarkShuffleOverhead(b *testing.B) {
// 	r := New(NewSource(1))
// 	for n := b.N; n > 0; n-- {
// 		r.Shuffle(52, fn(i, j isize) {
// 			if i < 0 || i >= 52 || j < 0 || j >= 52 {
// 				b.Fatalf("bad swap(%d, %d)", i, j)
// 			}
// 		})
// 	}
// }

// fn BenchmarkRead3(b *testing.B) {
// 	r := New(NewSource(1))
// 	buf := make([]byte, 3)
// 	b.ResetTimer()
// 	for n := b.N; n > 0; n-- {
// 		r.Read(buf)
// 	}
// }

// fn BenchmarkRead64(b *testing.B) {
// 	r := New(NewSource(1))
// 	buf := make([]byte, 64)
// 	b.ResetTimer()
// 	for n := b.N; n > 0; n-- {
// 		r.Read(buf)
// 	}
// }

// fn BenchmarkRead1000(b *testing.B) {
// 	r := New(NewSource(1))
// 	buf := make([]byte, 1000)
// 	b.ResetTimer()
// 	for n := b.N; n > 0; n-- {
// 		r.Read(buf)
// 	}
// }
