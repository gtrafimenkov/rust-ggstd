// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2010 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

/// A Point is an X, Y coordinate pair. The axes increase right and down.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    // // String returns a string representation of p like "(3,4)".
    // fn (p Point) String() string {
    // 	return "(" + strconv.Itoa(p.x) + "," + strconv.Itoa(p.y) + ")"
    // }

    /// add returns the vector p+q.
    pub fn add(&self, q: &Point) -> Point {
        Point::new(self.x + q.x, self.y + q.y)
    }

    /// sub returns the vector p-q.
    pub fn sub(&self, q: &Point) -> Point {
        Point::new(self.x - q.x, self.y - q.y)
    }

    /// mul returns the vector p*k.
    pub fn mul(&self, k: isize) -> Point {
        Point::new(self.x * k, self.y * k)
    }

    /// div returns the vector p/k.
    pub fn div(&self, k: isize) -> Point {
        Point::new(self.x / k, self.y / k)
    }

    /// inside reports whether p is in r.
    pub fn inside(&self, r: &Rectangle) -> bool {
        r.min.x <= self.x && self.x < r.max.x && r.min.y <= self.y && self.y < r.max.y
    }

    // // Mod returns the point q in r such that p.x-q.x is a multiple of r's width
    // // and p.y-q.y is a multiple of r's height.
    // pub fn (p Point) Mod(r Rectangle) Point {
    // 	w, h := r.Dx(), r.Dy()
    // 	p = p.sub(r.Min)
    // 	p.x = p.x % w
    // 	if p.x < 0 {
    // 		p.x += w
    // 	}
    // 	p.y = p.y % h
    // 	if p.y < 0 {
    // 		p.y += h
    // 	}
    // 	return p.add(r.Min)
    // }

    // // eq reports whether p and q are equal.
    // pub fn (p Point) eq(q Point) -> bool {
    // 	return p == q
    // }

    // // ZP is the zero Point.
    // //
    // // Deprecated: Use a literal image.Point{} instead.
    // var ZP Point

    // // Pt is shorthand for Point{X, Y}.
    // pub fn Pt(X, Y int) Point {
    // 	return Point{X, Y}
    // }
}

/// A Rectangle contains the points with Min.x <= X < Max.x, Min.y <= Y < Max.y.
/// It is well-formed if Min.x <= Max.x and likewise for Y. Points are always
/// well-formed. A rectangle's methods always return well-formed outputs for
/// well-formed inputs.
///
/// A Rectangle is also an Image whose bounds are the rectangle itself. At
/// returns color.Opaque for points in the rectangle and color.Transparent
/// otherwise.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rectangle {
    pub min: Point,
    pub max: Point,
}

#[allow(non_snake_case)]
impl Rectangle {
    pub const fn new(min: Point, max: Point) -> Self {
        Self { min, max }
    }

    // // String returns a string representation of r like "(3,4)-(6,5)".
    // pub fn String(&self) string {
    // 	return r.min.String() + "-" + r.max.String()
    // }

    // Dx returns r's width.
    pub fn dx(&self) -> usize {
        return (self.max.x - self.min.x).try_into().unwrap();
    }

    // Dy returns r's height.
    pub fn dy(&self) -> usize {
        return (self.max.y - self.min.y).try_into().unwrap();
    }

    /// size returns r's width and height.
    pub fn size(&self) -> Point {
        Point {
            x: self.max.x - self.min.x,
            y: self.max.y - self.min.y,
        }
    }

    /// add returns the rectangle r translated by p.
    pub fn add(&self, p: &Point) -> Rectangle {
        Rectangle::new(
            Point::new(self.min.x + p.x, self.min.y + p.y),
            Point::new(self.max.x + p.x, self.max.y + p.y),
        )
    }

    // // sub returns the rectangle r translated by -p.
    // pub fn sub(&self, p Point) -> Rectangle {
    // 	return Rectangle{
    // 		Point{self.min.x - p.x, self.min.y - p.y},
    // 		Point{self.max.x - p.x, self.max.y - p.y},
    // 	}
    // }

    // // Inset returns the rectangle r inset by n, which may be negative. If either
    // // of r's dimensions is less than 2*n then an empty rectangle near the center
    // // of r will be returned.
    // pub fn Inset(&self, n int) -> Rectangle {
    // 	if self.Dx() < 2*n {
    // 		self.min.x = (self.min.x + self.max.x) / 2
    // 		self.max.x = self.min.x
    // 	} else {
    // 		self.min.x += n
    // 		self.max.x -= n
    // 	}
    // 	if self.Dy() < 2*n {
    // 		self.min.y = (self.min.y + self.max.y) / 2
    // 		self.max.y = self.min.y
    // 	} else {
    // 		self.min.y += n
    // 		self.max.y -= n
    // 	}
    // 	return r
    // }

    /// intersect returns the largest rectangle contained by both r and s. If the
    /// two rectangles do not overlap then the zero rectangle will be returned.
    pub fn intersect(&self, s: &Rectangle) -> Rectangle {
        let r = Rectangle::new(
            Point::new(self.min.x.max(s.min.x), self.min.y.max(s.min.y)),
            Point::new(self.max.x.min(s.max.x), self.max.y.min(s.max.y)),
        );
        // Letting r0 and s0 be the values of r and s at the time that the method
        // is called, this next line is equivalent to:
        //
        // if max(r0.min.x, s0.min.x) >= min(r0.max.x, s0.max.x) || likewiseForY { etc }
        if r.empty() {
            return ZR;
        } else {
            r
        }
    }

    /// union returns the smallest rectangle that contains both r and s.
    pub fn union(&self, s: &Rectangle) -> Rectangle {
        if self.empty() {
            return s.clone();
        }
        if s.empty() {
            return self.clone();
        }
        let r = Rectangle::new(
            Point::new(self.min.x.min(s.min.x), self.min.y.min(s.min.y)),
            Point::new(self.max.x.max(s.max.x), self.max.y.max(s.max.y)),
        );
        return r;
    }

    /// empty reports whether the rectangle contains no points.
    pub fn empty(&self) -> bool {
        return self.min.x >= self.max.x || self.min.y >= self.max.y;
    }

    /// eq reports whether r and s contain the same set of points. All empty
    /// rectangles are considered equal.
    pub fn eq(&self, s: &Rectangle) -> bool {
        return *self == *s || self.empty() && s.empty();
    }

    /// Overlaps reports whether r and s have a non-empty intersection.
    pub fn overlaps(&self, s: &Rectangle) -> bool {
        !self.empty()
            && !s.empty()
            && self.min.x < s.max.x
            && s.min.x < self.max.x
            && self.min.y < s.max.y
            && s.min.y < self.max.y
    }

    /// inside reports whether every point in r is in s.
    pub fn inside(&self, s: &Rectangle) -> bool {
        if self.empty() {
            return true;
        }
        // Note that self.Max is an exclusive bound for r, so that self.inside(s)
        // does not require that self.max.inside(s).
        s.min.x <= self.min.x
            && self.max.x <= s.max.x
            && s.min.y <= self.min.y
            && self.max.y <= s.max.y
    }

    // // Canon returns the canonical version of self. The returned rectangle has minimum
    // // and maximum coordinates swapped if necessary so that it is well-formed.
    // fn Canon(&self) -> Rectangle {
    // 	if self.max.x < self.min.x {
    // 		self.min.x, self.max.x = self.max.x, self.min.x
    // 	}
    // 	if self.max.y < self.min.y {
    // 		self.min.y, self.max.y = self.max.y, self.min.y
    // 	}
    // 	return r
    // }

    // // At implements the Image interface.
    // fn At(&self, x, y int) color.Color {
    // 	if (Point{x, y}).inside(r) {
    // 		return color.Opaque
    // 	}
    // 	return color.Transparent
    // }

    // // RGBA64At implements the RGBA64Image interface.
    // fn RGBA64At(&self, x, y int) color.RGBA64 {
    // 	if (Point{x, y}).inside(r) {
    // 		return color.RGBA64{0xffff, 0xffff, 0xffff, 0xffff}
    // 	}
    // 	return color.RGBA64{}
    // }

    // // Bounds implements the Image interface.
    // fn Bounds(&self) -> Rectangle {
    // 	return r
    // }

    // // ColorModel implements the Image interface.
    // fn ColorModel(&self) color::Model {
    // 	return color.Alpha16Model
    // }
}

/// ZR is the zero Rectangle.
//
// Deprecated: Use a literal image.Rectangle{} instead.
pub const ZR: Rectangle = Rectangle::new(Point::new(0, 0), Point::new(0, 0));

/// rect is shorthand for Rectangle{Pt(x0, y0), Pt(x1, y1)}. The returned
/// rectangle has minimum and maximum coordinates swapped if necessary so that
/// it is well-formed.
pub fn rect(x0: isize, y0: isize, x1: isize, y1: isize) -> Rectangle {
    let mut x0 = x0;
    let mut x1 = x1;
    let mut y0 = y0;
    let mut y1 = y1;
    if x0 > x1 {
        (x0, x1) = (x1, x0);
    }
    if y0 > y1 {
        (y0, y1) = (y1, y0);
    }
    return Rectangle {
        min: Point { x: x0, y: y0 },
        max: Point { x: x1, y: y1 },
    };
}

// // mul3NonNeg returns (x * y * z), unless at least one argument is negative or
// // if the computation overflows the int type, in which case it returns -1.
// fn mul3NonNeg(x int, y int, z int) int {
// 	if (x < 0) || (y < 0) || (z < 0) {
// 		return -1
// 	}
// 	hi, lo := bits.Mul64(uint64(x), uint64(y))
// 	if hi != 0 {
// 		return -1
// 	}
// 	hi, lo = bits.Mul64(lo, uint64(z))
// 	if hi != 0 {
// 		return -1
// 	}
// 	a := int(lo)
// 	if (a < 0) || (uint64(a) != lo) {
// 		return -1
// 	}
// 	return a
// }

// // add2NonNeg returns (x + y), unless at least one argument is negative or if
// // the computation overflows the int type, in which case it returns -1.
// fn add2NonNeg(x int, y int) int {
// 	if (x < 0) || (y < 0) {
// 		return -1
// 	}
// 	a := x + y
// 	if a < 0 {
// 		return -1
// 	}
// 	return a
// }
