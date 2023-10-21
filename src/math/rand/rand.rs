// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use super::rng::RngSource;
use crate::time;
use std::sync::Mutex;
use std::sync::OnceLock;

/// A Source represents a source of uniformly-distributed
/// pseudo-random i64 values in the range [0, 1<<63)
/// and pseudo-random uint64 values in the range [0, 1<<64) directly.
//
// A Source is not safe for concurrent use by multiple goroutines.
pub trait Source {
    fn int63(&mut self) -> i64;
    fn seed(&mut self, seed: i64);
    fn uint64(&mut self) -> u64;
}

/// new_source returns a new pseudo-random Source seeded with the given value.
// Unlike the default Source used by top-level functions, this source is not
// safe for concurrent use by multiple goroutines.
// The returned Source implements Source64.
pub fn new_source(seed: i64) -> Box<impl Source> {
    new_source_int(seed)
}

fn new_source_int(seed: i64) -> Box<RngSource> {
    let mut rng = Box::new(RngSource::new());
    rng.seed(seed);
    rng
}

/// A Rand is a source of random numbers.
pub struct Rand<S: Source> {
    src: Box<S>,
    // 	s64 Source64 // non-nil if src is source64

    // 	// readVal contains remainder of 63-bit integer used for bytes
    // 	// generation during most recent Read call.
    // 	// It is saved so next Read call can start where the previous
    // 	// one finished.
    // 	readVal i64
    // 	// readPos indicates the number of low-order bytes of readVal
    // 	// that are still valid.
    // 	readPos int8
}

impl<S: Source> Rand<S> {
    /// new returns a new Rand that uses random values from src
    /// to generate other random values.
    pub fn new(src: Box<S>) -> Self {
        // 	s64, _ := src.(Source64)
        Self { src }
    }

    /// seed uses the provided seed value to initialize the generator to a deterministic state.
    /// seed should not be called concurrently with any other Rand method.
    pub fn seed(&mut self, seed: i64) {
        // 	if lk, ok := r.src.(*lockedSource); ok {
        // 		lk.seedPos(seed, &self.readPos)
        // 		return
        // 	}

        self.src.seed(seed);
        // 	self.readPos = 0
    }

    /// int63 returns a non-negative pseudo-random 63-bit integer as an i64.
    pub fn int63(&mut self) -> i64 {
        self.src.int63()
    }

    /// uint32 returns a pseudo-random 32-bit value as a uint32.
    pub fn uint32(&mut self) -> u32 {
        (self.int63() >> 31) as u32
    }

    /// uint64 returns a pseudo-random 64-bit value as a uint64.
    pub fn uint64(&mut self) -> u64 {
        // 	if self.s64 != nil {
        self.src.uint64()
        // 	}
        // 	return uint64(self.int63())>>31 | uint64(self.int63())<<32
    }

    /// int31 returns a non-negative pseudo-random 31-bit integer as an i32.
    pub fn int31(&mut self) -> i32 {
        (self.int63() >> 32) as i32
    }

    /// Int returns a non-negative pseudo-random isize.
    pub fn int(&mut self) -> isize {
        let u = self.int63() as usize;
        (u << 1 >> 1) as isize // clear sign bit if isize == i32
    }

    /// int63n returns, as an i64, a non-negative pseudo-random number in the half-open interval [0,n).
    /// It panics if n <= 0.
    pub fn int63n(&mut self, n: i64) -> i64 {
        if n <= 0 {
            panic!("invalid argument to int63n")
        }
        if n & (n - 1) == 0 {
            // n is power of two, can mask
            return self.int63() & (n - 1);
        }
        let max = ((1 << 63) - 1 - (1 << 63) % (n as u64)) as i64;
        let mut v = self.int63();
        while v > max {
            v = self.int63();
        }
        v % n
    }

    // int31n returns, as an i32, a non-negative pseudo-random number in the half-open interval [0,n).
    // It panics if n <= 0.
    pub fn int31n(&mut self, n: i32) -> i32 {
        if n <= 0 {
            panic!("invalid argument to int31n");
        }
        if n & (n - 1) == 0 {
            // n is power of two, can mask
            return self.int31() & (n - 1);
        }
        let max = ((1 << 31) - 1 - (1 << 31) % (n as u32)) as i32;
        let mut v = self.int31();
        while v > max {
            v = self.int31()
        }
        v % n
    }

    // // int31n returns, as an i32, a non-negative pseudo-random number in the half-open interval [0,n).
    // // n must be > 0, but int31n does not check this; the caller must ensure it.
    // // int31n exists because int31n is inefficient, but Go 1 compatibility
    // // requires that the stream of values produced by math/rand remain unchanged.
    // // int31n can thus only be used internally, by newly introduced APIs.
    // //
    // // For implementation details, see:
    // // https://lemire.me/blog/2016/06/27/a-fast-alternative-to-the-modulo-reduction
    // // https://lemire.me/blog/2016/06/30/fast-random-shuffling
    // fn (r *Rand) int31n(n :i32) -> i32 {
    // 	v := self.uint32()
    // 	prod := uint64(v) * uint64(n)
    // 	low := uint32(prod)
    // 	if low < uint32(n) {
    // 		thresh := uint32(-n) % uint32(n)
    // 		for low < thresh {
    // 			v = self.uint32()
    // 			prod = uint64(v) * uint64(n)
    // 			low = uint32(prod)
    // 		}
    // 	}
    // 	return i32(prod >> 32)
    // }

    /// intn returns, as an isize, a non-negative pseudo-random number in the half-open interval [0,n).
    /// It panics if n <= 0.
    pub fn intn(&mut self, n: isize) -> isize {
        if n <= 0 {
            panic!("invalid argument to intn");
        }
        if n < (1 << 31) {
            (self.int31n(n as i32)) as isize
        } else {
            (self.int63n(n as i64)) as isize
        }
    }

    /// float64 returns, as a float64, a pseudo-random number in the half-open interval [0.0,1.0).
    pub fn float64(&mut self) -> f64 {
        // A clearer, simpler implementation would be:
        //	return float64(self.int63n(1<<53)) / (1<<53)
        // However, Go 1 shipped with
        //	return float64(self.int63()) / (1 << 63)
        // and we want to preserve that value stream.
        //
        // There is one bug in the value stream: self.int63() may be so close
        // to 1<<63 that the division rounds up to 1.0, and we've guaranteed
        // that the result is always less than 1.0.
        //
        // We tried to fix this by mapping 1.0 back to 0.0, but since float64
        // values near 0 are much denser than near 1, mapping 1 to 0 caused
        // a theoretically significant overshoot in the probability of returning 0.
        // Instead of that, if we round up to 1, just try again.
        // Getting 1 only happens 1/2⁵³ of the time, so most clients
        // will not observe it anyway.
        // again:
        let f = self.int63() as f64 / (1_u64 << 63) as f64;
        if f == 1.0 {
            // resample; this branch is taken O(never)
            self.int63() as f64 / (1_u64 << 63) as f64
        } else {
            f
        }
    }

    /// float32 returns, as a float32, a pseudo-random number in the half-open interval [0.0,1.0).
    pub fn float32(&mut self) -> f32 {
        // Same rationale as in float64: we want to preserve the Go 1 value
        // stream except we want to fix it not to return 1.0
        // This only happens 1/2²⁴ of the time (plus the 1/2⁵³ of the time in float64).
        let f = self.float64() as f32;
        if f == 1.0 {
            // resample; this branch is taken O(very rarely)
            self.float64() as f32
        } else {
            f
        }
    }

    // // Perm returns, as a slice of n ints, a pseudo-random permutation of the integers
    // // in the half-open interval [0,n).
    // pub fn Perm(&mut self, n isize) []isize {
    // 	m := make([]isize, n)
    // 	// In the following loop, the iteration when i=0 always swaps m[0] with m[0].
    // 	// A change to remove this useless iteration is to assign 1 to i in the init
    // 	// statement. But Perm also effects self. Making this change will affect
    // 	// the final state of self. So this change can't be made for compatibility
    // 	// reasons for Go 1.
    // 	for i := 0; i < n; i++ {
    // 		j := self.intn(i + 1)
    // 		m[i] = m[j]
    // 		m[j] = i
    // 	}
    // 	return m
    // }

    // // Shuffle pseudo-randomizes the order of elements.
    // // n is the number of elements. Shuffle panics if n < 0.
    // // swap swaps the elements with indexes i and j.
    // pub fn Shuffle(&mut self, n isize, swap fn(i, j isize)) {
    // 	if n < 0 {
    // 		panic("invalid argument to Shuffle")
    // 	}

    // 	// Fisher-Yates shuffle: https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle
    // 	// Shuffle really ought not be called with n that doesn't fit in 32 bits.
    // 	// Not only will it take a very long time, but with 2³¹! possible permutations,
    // 	// there's no way that any PRNG can have a big enough internal state to
    // 	// generate even a minuscule percentage of the possible permutations.
    // 	// Nevertheless, the right API signature accepts an isize n, so handle it as best we can.
    // 	i := n - 1
    // 	for ; i > 1<<31-1-1; i-- {
    // 		j := isize(self.int63n(i64(i + 1)))
    // 		swap(i, j)
    // 	}
    // 	for ; i > 0; i-- {
    // 		j := isize(self.int31n(i32(i + 1)))
    // 		swap(i, j)
    // 	}
    // }

    // // Read generates len(p) random bytes and writes them into p. It
    // // always returns len(p) and a nil erroself.
    // // Read should not be called concurrently with any other Rand method.
    // pub fn Read(&mut self, p []byte) (n isize, err error) {
    // 	if lk, ok := self.src.(*lockedSource); ok {
    // 		return lk.read(p, &self.readVal, &self.readPos)
    // 	}
    // 	return read(p, self.src, &self.readVal, &self.readPos)
    // }

    /*
     * Normal distribution
     *
     * See "The Ziggurat Method for Generating Random Variables"
     * (Marsaglia & Tsang, 2000)
     * http://www.jstatsoft.org/v05/i08/paper [pdf]
     */

    /// norm_float64 returns a normally distributed float64 in
    /// the range -math.MaxFloat64 through +math.MaxFloat64 inclusive,
    /// with standard normal distribution (mean = 0, stddev = 1).
    /// To produce a different normal distribution, callers can
    /// adjust the output using:
    ///
    ///    sample = norm_float64() * desiredStdDev + desiredMean
    pub fn norm_float64(&mut self) -> f64 {
        loop {
            let j = self.uint32() as i32; // Possibly negative
            let i = j & 0x7F;
            let mut x = (j as f64) * (WN[i as usize] as f64);
            if j.unsigned_abs() < KN[i as usize] {
                // This case should be hit better than 99% of the time.
                return x;
            }

            if i == 0 {
                // This extra work is only required for the base strip.
                loop {
                    x = -self.float64().ln() * (1.0 / RN);
                    let y = -self.float64().ln();
                    if y + y >= x * x {
                        break;
                    }
                }
                if j > 0 {
                    return RN + x;
                }
                return -RN - x;
            }
            if FN[i as usize] + (self.float64() as f32) * (FN[(i - 1) as usize] - FN[i as usize])
                < (f64::exp(-0.5 * x * x) as f32)
            {
                return x;
            }
        }
    }
}

// fn read(p []byte, src Source, readVal *i64, readPos *int8) (n isize, err error) {
// 	pos := *readPos
// 	val := *readVal
// 	rng, _ := src.(*RngSource)
// 	for n = 0; n < len(p); n++ {
// 		if pos == 0 {
// 			if rng != nil {
// 				val = rng.int63()
// 			} else {
// 				val = src.int63()
// 			}
// 			pos = 7
// 		}
// 		p[n] = byte(val)
// 		val >>= 8
// 		pos--
// 	}
// 	*readPos = pos
// 	*readVal = val
// 	return
// }

/*
 * Top-level convenience functions
 */

// var globalRand = Rand::new(new(lockedSource))
fn global_rand() -> &'static Mutex<Rand<RngSource>> {
    static GLOBAL_RAND: OnceLock<Mutex<Rand<RngSource>>> = OnceLock::new();
    // ggstd:
    //   Go uses fastrand64() to seed the random generator.
    //   No matter how it is seeded, the result is not suitable for
    //   crypto-sensitive work.  Hence it doesn't matter what is used
    //   for the seed.  Here we use number of seconds since epoch.
    let seed = time::now().unix() as i64;
    GLOBAL_RAND.get_or_init(|| Mutex::new(Rand::new(new_source_int(seed))))
}

/// Seed uses the provided seed value to initialize the default Source to a
/// deterministic state. Seed values that have the same remainder when
/// divided by 2³¹-1 generate the same pseudo-random sequence.
/// Seed, unlike the Rand.Seed method, is safe for concurrent use.
//
// If Seed is not called, the generator is seeded randomly at program startup.
//
// Prior to Go 1.20, the generator was seeded like Seed(1) at program startup.
// To force the old behavior, call Seed(1) at program startup.
// Alternately, set GODEBUG=randautoseed=0 in the environment
// before making any calls to functions in this package.
//
/// Deprecated: Programs that call Seed and then expect a specific sequence
/// of results from the global random source (using functions such as Int)
/// can be broken when a dependency changes how much it consumes
/// from the global random source. To avoid such breakages, programs
/// that need a specific result sequence should use NewRand(NewSource(seed))
/// to obtain a random generator that other packages cannot access.
#[deprecated]
pub fn seed(seed: i64) {
    global_rand().lock().unwrap().seed(seed)
}

// // int63 returns a non-negative pseudo-random 63-bit integer as an i64
// // from the default Source.
// pub fn int63() -> i64 { return global_rand().int63() }

// // uint32 returns a pseudo-random 32-bit value as a uint32
// // from the default Source.
// pub fn uint32() -> u32 { return global_rand().uint32() }

// // uint64 returns a pseudo-random 64-bit value as a uint64
// // from the default Source.
// pub fn uint64() -> u64 { return global_rand().uint64() }

/// int31 returns a non-negative pseudo-random 31-bit integer as an i32
/// from the default Source.
pub fn int31() -> i32 {
    global_rand().lock().unwrap().int31()
}

/// int returns a non-negative pseudo-random isize from the default Source.
pub fn int() -> isize {
    global_rand().lock().unwrap().int()
}

/// int63n returns, as an i64, a non-negative pseudo-random number in the half-open interval [0,n)
/// from the default Source.
/// It panics if n <= 0.
pub fn int63n(n: i64) -> i64 {
    return global_rand().lock().unwrap().int63n(n);
}

/// int31n returns, as an i32, a non-negative pseudo-random number in the half-open interval [0,n)
/// from the default Source.
/// It panics if n <= 0.
pub fn int31n(n: i32) -> i32 {
    return global_rand().lock().unwrap().int31n(n);
}

/// intn returns, as an isize, a non-negative pseudo-random number in the half-open interval [0,n)
/// from the default Source.
/// It panics if n <= 0.
pub fn intn(n: isize) -> isize {
    global_rand().lock().unwrap().intn(n)
}

/// float64 returns, as a float64, a pseudo-random number in the half-open interval [0.0,1.0)
/// from the default Source.
pub fn float64() -> f64 {
    global_rand().lock().unwrap().float64()
}

/// float32 returns, as a float32, a pseudo-random number in the half-open interval [0.0,1.0)
/// from the default Source.
pub fn float32() -> f32 {
    global_rand().lock().unwrap().float32()
}

// // Perm returns, as a slice of n ints, a pseudo-random permutation of the integers
// // in the half-open interval [0,n) from the default Source.
// pub fn Perm(n isize) []isize { return global_rand().Perm(n) }

// // Shuffle pseudo-randomizes the order of elements using the default Source.
// // n is the number of elements. Shuffle panics if n < 0.
// // swap swaps the elements with indexes i and j.
// pub fn Shuffle(n isize, swap fn(i, j isize)) { global_rand().Shuffle(n, swap) }

// // Read generates len(p) random bytes from the default Source and
// // writes them into p. It always returns len(p) and a nil error.
// // Read, unlike the Rand.Read method, is safe for concurrent use.
// //
// // Deprecated: For almost all use cases, crypto/rand.Read is more appropriate.
// pub fn Read(p []byte) (n isize, err error) { return global_rand().Read(p) }

/// norm_float64 returns a normally distributed float64 in the range
/// [-math.MaxFloat64, +math.MaxFloat64] with
/// standard normal distribution (mean = 0, stddev = 1)
/// from the default Source.
/// To produce a different normal distribution, callers can
/// adjust the output using:
///
///     sample = norm_float64() * desiredStdDev + desiredMean
pub fn norm_float64() -> f64 {
    return global_rand().lock().unwrap().norm_float64();
}

// // ExpFloat64 returns an exponentially distributed float64 in the range
// // (0, +math.MaxFloat64] with an exponential distribution whose rate parameter
// // (lambda) is 1 and whose mean is 1/lambda (1) from the default Source.
// // To produce a distribution with a different rate parameter,
// // callers can adjust the output using:
// //
// //	sample = ExpFloat64() / desiredRateParameter
// pub fn ExpFloat64() -> f64 { return global_rand().ExpFloat64() }

const RN: f64 = 3.442619855899;

const KN: [u32; 128] = [
    0x76ad2212, 0x0, 0x600f1b53, 0x6ce447a6, 0x725b46a2, 0x7560051d, 0x774921eb, 0x789a25bd,
    0x799045c3, 0x7a4bce5d, 0x7adf629f, 0x7b5682a6, 0x7bb8a8c6, 0x7c0ae722, 0x7c50cce7, 0x7c8cec5b,
    0x7cc12cd6, 0x7ceefed2, 0x7d177e0b, 0x7d3b8883, 0x7d5bce6c, 0x7d78dd64, 0x7d932886, 0x7dab0e57,
    0x7dc0dd30, 0x7dd4d688, 0x7de73185, 0x7df81cea, 0x7e07c0a3, 0x7e163efa, 0x7e23b587, 0x7e303dfd,
    0x7e3beec2, 0x7e46db77, 0x7e51155d, 0x7e5aabb3, 0x7e63abf7, 0x7e6c222c, 0x7e741906, 0x7e7b9a18,
    0x7e82adfa, 0x7e895c63, 0x7e8fac4b, 0x7e95a3fb, 0x7e9b4924, 0x7ea0a0ef, 0x7ea5b00d, 0x7eaa7ac3,
    0x7eaf04f3, 0x7eb3522a, 0x7eb765a5, 0x7ebb4259, 0x7ebeeafd, 0x7ec2620a, 0x7ec5a9c4, 0x7ec8c441,
    0x7ecbb365, 0x7ece78ed, 0x7ed11671, 0x7ed38d62, 0x7ed5df12, 0x7ed80cb4, 0x7eda175c, 0x7edc0005,
    0x7eddc78e, 0x7edf6ebf, 0x7ee0f647, 0x7ee25ebe, 0x7ee3a8a9, 0x7ee4d473, 0x7ee5e276, 0x7ee6d2f5,
    0x7ee7a620, 0x7ee85c10, 0x7ee8f4cd, 0x7ee97047, 0x7ee9ce59, 0x7eea0eca, 0x7eea3147, 0x7eea3568,
    0x7eea1aab, 0x7ee9e071, 0x7ee98602, 0x7ee90a88, 0x7ee86d08, 0x7ee7ac6a, 0x7ee6c769, 0x7ee5bc9c,
    0x7ee48a67, 0x7ee32efc, 0x7ee1a857, 0x7edff42f, 0x7ede0ffa, 0x7edbf8d9, 0x7ed9ab94, 0x7ed7248d,
    0x7ed45fae, 0x7ed1585c, 0x7ece095f, 0x7eca6ccb, 0x7ec67be2, 0x7ec22eee, 0x7ebd7d1a, 0x7eb85c35,
    0x7eb2c075, 0x7eac9c20, 0x7ea5df27, 0x7e9e769f, 0x7e964c16, 0x7e8d44ba, 0x7e834033, 0x7e781728,
    0x7e6b9933, 0x7e5d8a1a, 0x7e4d9ded, 0x7e3b737a, 0x7e268c2f, 0x7e0e3ff5, 0x7df1aa5d, 0x7dcf8c72,
    0x7da61a1e, 0x7d72a0fb, 0x7d30e097, 0x7cd9b4ab, 0x7c600f1a, 0x7ba90bdc, 0x7a722176, 0x77d664e5,
];
const WN: [f32; 128] = [
    1.729_040_5e-9,
    1.2680929e-10,
    1.6897518e-10,
    1.9862688e-10,
    2.2232431e-10,
    2.4244937e-10,
    2.601613e-10,
    2.7611988e-10,
    2.9073963e-10,
    3.042997e-10,
    3.1699796e-10,
    3.289802e-10,
    3.4035738e-10,
    3.5121603e-10,
    3.616251e-10,
    3.7164058e-10,
    3.8130857e-10,
    3.9066758e-10,
    3.9975012e-10,
    4.08584e-10,
    4.1719309e-10,
    4.2559822e-10,
    4.338176e-10,
    4.418672e-10,
    4.497613e-10,
    4.5751258e-10,
    4.651324e-10,
    4.7263105e-10,
    4.8001775e-10,
    4.87301e-10,
    4.944885e-10,
    5.015873e-10,
    5.0860405e-10,
    5.155446e-10,
    5.2241467e-10,
    5.2921934e-10,
    5.359635e-10,
    5.426517e-10,
    5.4928817e-10,
    5.5587696e-10,
    5.624219e-10,
    5.6892646e-10,
    5.753941e-10,
    5.818282e-10,
    5.882317e-10,
    5.946077e-10,
    6.00959e-10,
    6.072884e-10,
    6.135985e-10,
    6.19892e-10,
    6.2617134e-10,
    6.3243905e-10,
    6.386974e-10,
    6.449488e-10,
    6.511956e-10,
    6.5744005e-10,
    6.6368433e-10,
    6.699307e-10,
    6.7618144e-10,
    6.824387e-10,
    6.8870465e-10,
    6.949815e-10,
    7.012715e-10,
    7.075768e-10,
    7.1389966e-10,
    7.202424e-10,
    7.266073e-10,
    7.329966e-10,
    7.394128e-10,
    7.4585826e-10,
    7.5233547e-10,
    7.58847e-10,
    7.653954e-10,
    7.719835e-10,
    7.7861395e-10,
    7.852897e-10,
    7.920138e-10,
    7.987892e-10,
    8.0561924e-10,
    8.125073e-10,
    8.194569e-10,
    8.2647167e-10,
    8.3355556e-10,
    8.407127e-10,
    8.479473e-10,
    8.55264e-10,
    8.6266755e-10,
    8.7016316e-10,
    8.777562e-10,
    8.8545243e-10,
    8.932582e-10,
    9.0117996e-10,
    9.09225e-10,
    9.174008e-10,
    9.2571584e-10,
    9.341788e-10,
    9.427997e-10,
    9.515889e-10,
    9.605579e-10,
    9.697193e-10,
    9.790869e-10,
    9.88676e-10,
    9.985036e-10,
    1.008_588_2e-9,
    1.018_950_9e-9,
    1.029_615_1e-9,
    1.040_606_9e-9,
    1.051_956_6e-9,
    1.063_698e-9,
    1.075_870_2e-9,
    1.088_518_3e-9,
    1.101_694_7e-9,
    1.115_461_1e-9,
    1.129_890_2e-9,
    1.145_069_6e-9,
    1.161_105_2e-9,
    1.178_127_6e-9,
    1.196_299_5e-9,
    1.215_828_7e-9,
    1.236_985_6e-9,
    1.260_132_3e-9,
    1.285_769_7e-9,
    1.314_620_2e-9,
    1.347_784e-9,
    1.387_063_6e-9,
    1.435_740_3e-9,
    1.500_865_9e-9,
    1.603_094_8e-9,
];
const FN: [f32; 128] = [
    1.0,
    0.9635997,
    0.9362827,
    0.9130436,
    0.89228165,
    0.87324303,
    0.8555006,
    0.8387836,
    0.8229072,
    0.8077383,
    0.793177,
    0.7791461,
    0.7655842,
    0.7524416,
    0.73967725,
    0.7272569,
    0.7151515,
    0.7033361,
    0.69178915,
    0.68049186,
    0.6694277,
    0.658582,
    0.6479418,
    0.63749546,
    0.6272325,
    0.6171434,
    0.6072195,
    0.5974532,
    0.58783704,
    0.5783647,
    0.56903,
    0.5598274,
    0.5507518,
    0.54179835,
    0.5329627,
    0.52424055,
    0.5156282,
    0.50712204,
    0.49871865,
    0.49041483,
    0.48220766,
    0.4740943,
    0.46607214,
    0.4581387,
    0.45029163,
    0.44252872,
    0.43484783,
    0.427247,
    0.41972435,
    0.41227803,
    0.40490642,
    0.39760786,
    0.3903808,
    0.3832238,
    0.37613547,
    0.36911446,
    0.3621595,
    0.35526937,
    0.34844297,
    0.34167916,
    0.33497685,
    0.3283351,
    0.3217529,
    0.3152294,
    0.30876362,
    0.30235484,
    0.29600215,
    0.28970486,
    0.2834622,
    0.2772735,
    0.27113807,
    0.2650553,
    0.25902456,
    0.2530453,
    0.24711695,
    0.241239,
    0.23541094,
    0.22963232,
    0.2239027,
    0.21822165,
    0.21258877,
    0.20700371,
    0.20146611,
    0.19597565,
    0.19053204,
    0.18513499,
    0.17978427,
    0.17447963,
    0.1692209,
    0.16400786,
    0.15884037,
    0.15371831,
    0.14864157,
    0.14361008,
    0.13862377,
    0.13368265,
    0.12878671,
    0.12393598,
    0.119130544,
    0.11437051,
    0.10965602,
    0.104987256,
    0.10036444,
    0.095787846,
    0.0912578,
    0.08677467,
    0.0823389,
    0.077950984,
    0.073611505,
    0.06932112,
    0.06508058,
    0.06089077,
    0.056752663,
    0.0526674,
    0.048636295,
    0.044660863,
    0.040742867,
    0.03688439,
    0.033087887,
    0.029356318,
    0.025693292,
    0.022103304,
    0.018592102,
    0.015167298,
    0.011839478,
    0.008624485,
    0.005548995,
    0.0026696292,
];
