// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2015 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use super::{rect, Point, Rectangle, ZR};

#[test]
fn test_rectangle() {
    // in checks that every point in f is in g.
    let in_fn = |f: &Rectangle, g: &Rectangle| -> Result<(), String> {
        if !f.inside(g) {
            return Err(format!(
                "f={:?}, f.inside({:?}): got false, want true",
                f, g
            ));
        }
        for y in f.min.y..f.max.y {
            for x in f.min.x..f.max.x {
                let p = Point::new(x, y);
                if !p.inside(g) {
                    return Err(format!(
                        "p={:?}, p.inside({:?}): got false, want true",
                        p, g
                    ));
                }
            }
        }
        Ok(())
    };

    let rects = &[
        rect(0, 0, 10, 10),
        rect(10, 0, 20, 10),
        rect(1, 2, 3, 4),
        rect(4, 6, 10, 10),
        rect(2, 3, 12, 5),
        rect(-1, -2, 0, 0),
        rect(-1, -2, 4, 6),
        rect(-10, -20, 30, 40),
        rect(8, 8, 8, 8),
        rect(88, 88, 88, 88),
        rect(6, 5, 4, 3),
    ];

    // r.Eq(s) should be equivalent to every point in r being in s, and every
    // point in s being in r.
    for r in rects {
        for s in rects {
            let got = r.eq(s);
            let want = in_fn(r, s).is_ok() && in_fn(s, r).is_ok();
            assert_eq!(
                got, want,
                "Eq: r={:?}, s={:?}: got {}, want {}",
                r, s, got, want
            );
        }
    }

    // The intersection should be the largest rectangle a such that every point
    // in a is both in r and in s.
    for r in rects {
        for s in rects {
            let a = r.intersect(s);
            let res = in_fn(&a, &r);
            assert!(
                res.is_ok(),
                "intersect: r={:?}, s={:?}, a={:?}, a not in r: {:?}",
                r,
                s,
                a,
                res
            );
            let res = in_fn(&a, &s);
            assert!(
                res.is_ok(),
                "intersect: r={:?}, s={:?}, a={:?}, a not in s: {:?}",
                r,
                s,
                a,
                res
            );
            let is_zero = a == ZR;
            let overlaps = r.overlaps(&s);
            assert!(
                is_zero != overlaps,
                "intersect: r={:?}, s={:?}, a={:?}: isZero={} same as overlaps={}",
                r,
                s,
                a,
                is_zero,
                overlaps
            );
            let mut larger_than_a = [a, a, a, a];
            larger_than_a[0].min.x -= 1;
            larger_than_a[1].min.y -= 1;
            larger_than_a[2].max.x += 1;
            larger_than_a[3].max.y += 1;
            for (i, b) in larger_than_a.iter().enumerate() {
                if b.empty() {
                    // b isn't actually larger than a.
                    continue;
                }
                if in_fn(b, r).is_ok() && in_fn(b, s).is_ok() {
                    assert!(
                        false,
                        "intersect: r={:?}, s={:?}, a={:?}, b={:?}, i={}: intersection could be larger",
                        r, s, a, b, i
                    );
                }
            }
        }
    }

    // The union should be the smallest rectangle a such that every point in r
    // is in a and every point in s is in a.
    for r in rects {
        for s in rects {
            let a = r.union(s);
            let res = in_fn(&r, &a);
            assert!(
                res.is_ok(),
                "Union: r={:?}, s={:?}, a={:?}, r not in a: {:?}",
                r,
                s,
                a,
                res
            );
            // 			}
            let res = in_fn(&s, &a);
            assert!(
                res.is_ok(),
                "Union: r={:?}, s={:?}, a={:?}, s not in a: {:?}",
                r,
                s,
                a,
                res
            );
            if a.empty() {
                // You can't get any smaller than a.
                continue;
            }
            let mut smaller_than_a = [a, a, a, a];
            smaller_than_a[0].min.x += 1;
            smaller_than_a[1].min.y += 1;
            smaller_than_a[2].max.x -= 1;
            smaller_than_a[3].max.y -= 1;
            for (i, b) in smaller_than_a.iter().enumerate() {
                if in_fn(&r, &b).is_ok() && in_fn(&s, &b).is_ok() {
                    assert!(
                        false,
                        "Union: r={:?}, s={:?}, a={:?}, b={:?}, i={}: union could be smaller",
                        r, s, a, b, i
                    );
                }
            }
        }
    }
}
