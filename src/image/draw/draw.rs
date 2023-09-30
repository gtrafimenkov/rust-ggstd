// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::compat;
use crate::image::{self, Image, Img, Point, Rectangle};

// import (
// 	"image"
// 	"image/color"
// 	"image/internal/imageutil"
// )

// // m is the maximum color value returned by image.Color.RGBA.
// const m = 1<<16 - 1

// // Image is an image.Image with a Set method to change a single pixel.
// type Image interface {
// 	image.Image
// 	Set(x, y int, c color.Color)
// }

// // RGBA64Image extends both the Image and image.RGBA64Image interfaces with a
// // SetRGBA64 method to change a single pixel. SetRGBA64 is equivalent to
// // calling Set, but it can avoid allocations from converting concrete color
// // types to the color.Color interface type.
// type RGBA64Image interface {
// 	image.RGBA64Image
// 	Set(x, y int, c color.Color)
// 	SetRGBA64(x, y int, c color.RGBA64)
// }

// // Quantizer produces a palette for an image.
// type Quantizer interface {
// 	// Quantize appends up to cap(p) - len(p) colors to p and returns the
// 	// updated palette suitable for converting m to a paletted image.
// 	Quantize(p color.Palette, m image.Image) color.Palette
// }

/// Op is a Porter-Duff compositing operator.
#[derive(PartialEq)]
pub enum Op {
    /// Over specifies ``(src in mask) over dst''.
    Over,
    /// Src specifies ``src in mask''.
    Src,
}

// // draw implements the Drawer interface by calling the draw function with this
// // Op.
// fn (op: Op) draw(dst: &mut Img, r: &Rectangle, src: &Img, sp: &Point) {
// 	draw_mask(dst, r, src, sp, nil, image::Point{}, op)
// }

// // Drawer contains the draw method.
// type Drawer interface {
// 	// draw aligns r.min in dst with sp in src and then replaces the
// 	// rectangle r in dst with the result of drawing src on dst.
// 	draw(dst: &mut Img, r: &Rectangle, src: &Img, sp: &Point)
// }

// // FloydSteinberg is a Drawer that is the Src Op with Floyd-Steinberg error
// // diffusion.
// var FloydSteinberg Drawer = floydSteinberg{}

// type floydSteinberg struct{}

// fn (floydSteinberg) draw(dst: &mut Img, r: &Rectangle, src: &Img, sp: &Point) {
// 	clip(dst, &r, src, &sp, nil, nil)
// 	if r.empty() {
// 		return
// 	}
// 	drawPaletted(dst, r, src, sp, true)
// }

/// clip clips r against each image's bounds (after translating into the
/// destination image's coordinate space) and shifts the points sp and mp by
/// the same amount as the change in r.min.
fn clip(
    dst: &mut Img,
    r: &mut image::Rectangle,
    src: &Img,
    sp: &mut image::Point,
    mask: Option<&Img>,
    mp: &mut image::Point,
) {
    let orig = r.min;
    *r = r.intersect(&dst.bounds());
    *r = r.intersect(&src.bounds().add(&orig.sub(sp)));
    if let Some(mask) = mask {
        *r = r.intersect(&mask.bounds().add(&orig.sub(mp)));
    }
    let dx = r.min.x - orig.x;
    let dy = r.min.y - orig.y;
    if dx == 0 && dy == 0 {
        return;
    }
    sp.x += dx;
    sp.y += dy;
    // 	if mp != nil {
    mp.x += dx;
    mp.y += dy;
    // 	}
}

// fn processBackward(dst image.Image, r: &Rectangle, src: &Img, sp: &Point) bool {
// 	return dst == src &&
// 		r.overlaps(r.add(sp.sub(r.min))) &&
// 		(sp.y < r.min.y || (sp.y == r.min.y && sp.x < r.min.x))
// }

/// draw calls draw_mask with a nil mask.
pub fn draw(dst: &mut Img, r: &Rectangle, src: &Img, sp: &Point, op: Op) {
    draw_mask(dst, r, src, sp, None, &Point::new(0, 0), op)
}

/// draw_mask aligns r.min in dst with sp in src and mp in mask and then replaces the rectangle r
/// in dst with the result of a Porter-Duff composition. A nil mask is treated as opaque.
pub fn draw_mask(
    dst: &mut Img,
    r: &Rectangle,
    src: &Img,
    sp: &Point,
    mask: Option<&Img>,
    mp: &Point,
    op: Op,
) {
    let mut r = *r;
    let mut sp = *sp;
    let mut mp = *mp;
    clip(dst, &mut r, src, &mut sp, mask, &mut mp);
    if r.empty() {
        return;
    }

    // Fast paths for special cases. If none of them apply, then we fall back
    // to general but slower implementations.
    //
    // For NRGBA and NRGBA64 image types, the code paths aren't just faster.
    // They also avoid the information loss that would otherwise occur from
    // converting non-alpha-premultiplied color to and from alpha-premultiplied
    // color. See TestDrawSrcNonpremultiplied.
    // 	switch dst0 := dst.(type) {
    match dst {
        Img::RGBA(dst) => {
            match op {
                Op::Over => {
                    unimplemented!()
                    // 			if mask == nil {
                    // 				switch src0 := src.(type) {
                    // 				case *image.Uniform:
                    // 					sr, sg, sb, sa := src0.RGBA()
                    // 					if sa == 0xffff {
                    // 						drawFillSrc(dst0, r, sr, sg, sb, sa)
                    // 					} else {
                    // 						drawFillOver(dst0, r, sr, sg, sb, sa)
                    // 					}
                    // 					return
                    // 				case *image.RGBA:
                    // 					drawCopyOver(dst0, r, src0, sp)
                    // 					return
                    // 				case *image.NRGBA:
                    // 					drawNRGBAOver(dst0, r, src0, sp)
                    // 					return
                    // 				case *image.YCbCr:
                    // 					// An image.YCbCr is always fully opaque, and so if the
                    // 					// mask is nil (i.e. fully opaque) then the op is
                    // 					// effectively always Src. Similarly for image.Gray and
                    // 					// image.CMYK.
                    // 					if imageutil.DrawYCbCr(dst0, r, src0, sp) {
                    // 						return
                    // 					}
                    // 				case *image.Gray:
                    // 					drawGray(dst0, r, src0, sp)
                    // 					return
                    // 				case *image.CMYK:
                    // 					drawCMYK(dst0, r, src0, sp)
                    // 					return
                    // 				}
                    // 			} else if mask0, ok := mask.(*image.Alpha); ok {
                    // 				switch src0 := src.(type) {
                    // 				case *image.Uniform:
                    // 					drawGlyphOver(dst0, r, src0, mask0, mp)
                    // 					return
                    // 				case *image.RGBA:
                    // 					drawRGBAMaskOver(dst0, r, src0, sp, mask0, mp)
                    // 					return
                    // 				case *image.Gray:
                    // 					drawGrayMaskOver(dst0, r, src0, sp, mask0, mp)
                    // 					return
                    // 				// Case order matters. The next case (image.RGBA64Image) is an
                    // 				// interface type that the concrete types above also implement.
                    // 				case image.RGBA64Image:
                    // 					drawRGBA64ImageMaskOver(dst0, r, src0, sp, mask0, mp)
                    // 					return
                    // 				}
                    // 			}
                }
                Op::Src => {
                    if mask.is_none() {
                        match src {
                            // Img::Alpha(_) => todo!(),
                            // Img::Alpha16(_) => todo!(),
                            // Img::Gray(_) => todo!(),
                            // Img::Gray16(_) => todo!(),
                            Img::NRGBA(src) => {
                                draw_nrgba_src(dst, &r, src, &sp);
                                return;
                            }
                            // Img::NRGBA64(_) => todo!(),
                            // Img::Paletted(_) => todo!(),
                            Img::RGBA(_) => {
                                unimplemented!();
                                // 					d0 := dst0.pix_offset(r.min.x, r.min.y)
                                // 					s0 := src0.pix_offset(sp.x, sp.y)
                                // 					draw_copy_src(
                                // 						dst0.pix[d0..], dst0.stride, r, src0.pix[s0..], src0.stride, sp, 4*r.dx());
                                // 					return
                            }
                            // Img::RGBA64(_) => todo!(),
                            _ => unimplemented!(),
                        }
                        // 				switch src0 := src.(type) {
                        // 				case *image.Uniform:
                        // 					sr, sg, sb, sa := src0.RGBA()
                        // 					drawFillSrc(dst0, r, sr, sg, sb, sa)
                        // 					return
                        // 				case *image.YCbCr:
                        // 					if imageutil.DrawYCbCr(dst0, r, src0, sp) {
                        // 						return
                        // 					}
                        // 				case *image.Gray:
                        // 					drawGray(dst0, r, src0, sp)
                        // 					return
                        // 				case *image.CMYK:
                        // 					drawCMYK(dst0, r, src0, sp)
                        // 					return
                        // 				}
                    }
                }
            }
            // 		drawRGBA(dst0, r, src, sp, mask, mp, op)
            // 		return
        }
        // 	case *image.Paletted:
        // 		if op == Src && mask == nil {
        // 			if src0, ok := src.(*image.Uniform); ok {
        // 				colorIndex := uint8(dst0.Palette.Index(src0.C))
        // 				i0 := dst0.pix_offset(r.min.x, r.min.y)
        // 				i1 := i0 + r.dx()
        // 				for i := i0; i < i1; i++ {
        // 					dst0.pix[i] = colorIndex
        // 				}
        // 				firstRow := dst0.pix[i0:i1]
        // 				for y := r.min.y + 1; y < r.max.y; y++ {
        // 					i0 += dst0.stride
        // 					i1 += dst0.stride
        // 					copy(dst0.pix[i0:i1], firstRow)
        // 				}
        // 				return
        // 			} else if !processBackward(dst, r, src, sp) {
        // 				drawPaletted(dst0, r, src, sp, false)
        // 				return
        // 			}
        // 		}
        Img::NRGBA(dst) => {
            if op == Op::Src && mask.is_none() {
                if let Img::NRGBA(src) = src {
                    let d0 = dst.pix_offset(r.min.x, r.min.y);
                    let s0 = src.pix_offset(sp.x, sp.y);
                    draw_copy_src(
                        &mut dst.pix[d0..],
                        dst.stride,
                        &r,
                        &src.pix[s0..],
                        src.stride,
                        &sp,
                        4 * r.dx(),
                    );
                    return;
                }
            }
        }
        // 	case *image.NRGBA64:
        // 		if op == Src && mask == nil {
        // 			if src0, ok := src.(*image.NRGBA64); ok {
        // 				d0 := dst0.pix_offset(r.min.x, r.min.y)
        // 				s0 := src0.pix_offset(sp.x, sp.y)
        // 				draw_copy_src(
        // 					dst0.pix[d0..], dst0.stride, r, src0.pix[s0..], src0.stride, sp, 8*r.dx());
        // 				return
        // 			}
        // 		}
        // 	}
        // Img::Alpha(_) => todo!(),
        // Img::Alpha16(_) => todo!(),
        // Img::Gray(_) => todo!(),
        // Img::Gray16(_) => todo!(),
        // Img::NRGBA64(_) => todo!(),
        // Img::Paletted(_) => todo!(),
        // Img::RGBA64(_) => todo!(),
        _ => unimplemented!(),
    }

    // 	x0, x1, dx := r.min.x, r.max.x, 1
    // 	y0, y1, dy := r.min.y, r.max.y, 1
    // 	if processBackward(dst, r, src, sp) {
    // 		x0, x1, dx = x1-1, x0-1, -1
    // 		y0, y1, dy = y1-1, y0-1, -1
    // 	}

    // 	// FALLBACK1.17
    // 	//
    // 	// Try the draw.RGBA64Image and image.RGBA64Image interfaces, part of the
    // 	// standard library since Go 1.17. These are like the draw.Image and
    // 	// image.Image interfaces but they can avoid allocations from converting
    // 	// concrete color types to the color.Color interface type.

    // 	if dst0, _ := dst.(RGBA64Image); dst0 != nil {
    // 		if src0, _ := src.(image.RGBA64Image); src0 != nil {
    // 			if mask == nil {
    // 				sy := sp.y + y0 - r.min.y
    // 				my := mp.y + y0 - r.min.y
    // 				for y := y0; y != y1; y, sy, my = y+dy, sy+dy, my+dy {
    // 					sx := sp.x + x0 - r.min.x
    // 					mx := mp.x + x0 - r.min.x
    // 					for x := x0; x != x1; x, sx, mx = x+dx, sx+dx, mx+dx {
    // 						if op == Src {
    // 							dst0.SetRGBA64(x, y, src0.RGBA64At(sx, sy))
    // 						} else {
    // 							srgba := src0.RGBA64At(sx, sy)
    // 							a := m - uint32(srgba.A)
    // 							drgba := dst0.RGBA64At(x, y)
    // 							dst0.SetRGBA64(x, y, color.RGBA64{
    // 								R: uint16((uint32(drgba.R)*a)/m) + srgba.R,
    // 								G: uint16((uint32(drgba.G)*a)/m) + srgba.G,
    // 								B: uint16((uint32(drgba.B)*a)/m) + srgba.B,
    // 								A: uint16((uint32(drgba.A)*a)/m) + srgba.A,
    // 							})
    // 						}
    // 					}
    // 				}
    // 				return

    // 			} else if mask0, _ := mask.(image.RGBA64Image); mask0 != nil {
    // 				sy := sp.y + y0 - r.min.y
    // 				my := mp.y + y0 - r.min.y
    // 				for y := y0; y != y1; y, sy, my = y+dy, sy+dy, my+dy {
    // 					sx := sp.x + x0 - r.min.x
    // 					mx := mp.x + x0 - r.min.x
    // 					for x := x0; x != x1; x, sx, mx = x+dx, sx+dx, mx+dx {
    // 						ma := uint32(mask0.RGBA64At(mx, my).A)
    // 						switch {
    // 						case ma == 0:
    // 							if op == Over {
    // 								// No-op.
    // 							} else {
    // 								dst0.SetRGBA64(x, y, color.RGBA64{})
    // 							}
    // 						case ma == m && op == Src:
    // 							dst0.SetRGBA64(x, y, src0.RGBA64At(sx, sy))
    // 						default:
    // 							srgba := src0.RGBA64At(sx, sy)
    // 							if op == Over {
    // 								drgba := dst0.RGBA64At(x, y)
    // 								a := m - (uint32(srgba.A) * ma / m)
    // 								dst0.SetRGBA64(x, y, color.RGBA64{
    // 									R: uint16((uint32(drgba.R)*a + uint32(srgba.R)*ma) / m),
    // 									G: uint16((uint32(drgba.G)*a + uint32(srgba.G)*ma) / m),
    // 									B: uint16((uint32(drgba.B)*a + uint32(srgba.B)*ma) / m),
    // 									A: uint16((uint32(drgba.A)*a + uint32(srgba.A)*ma) / m),
    // 								})
    // 							} else {
    // 								dst0.SetRGBA64(x, y, color.RGBA64{
    // 									R: uint16(uint32(srgba.R) * ma / m),
    // 									G: uint16(uint32(srgba.G) * ma / m),
    // 									B: uint16(uint32(srgba.B) * ma / m),
    // 									A: uint16(uint32(srgba.A) * ma / m),
    // 								})
    // 							}
    // 						}
    // 					}
    // 				}
    // 				return
    // 			}
    // 		}
    // 	}

    // 	// FALLBACK1.0
    // 	//
    // 	// If none of the faster code paths above apply, use the draw.Image and
    // 	// image.Image interfaces, part of the standard library since Go 1.0.

    // 	var out color.RGBA64
    // 	sy := sp.y + y0 - r.min.y
    // 	my := mp.y + y0 - r.min.y
    // 	for y := y0; y != y1; y, sy, my = y+dy, sy+dy, my+dy {
    // 		sx := sp.x + x0 - r.min.x
    // 		mx := mp.x + x0 - r.min.x
    // 		for x := x0; x != x1; x, sx, mx = x+dx, sx+dx, mx+dx {
    // 			ma := uint32(m)
    // 			if mask != nil {
    // 				_, _, _, ma = mask.At(mx, my).RGBA()
    // 			}
    // 			switch {
    // 			case ma == 0:
    // 				if op == Over {
    // 					// No-op.
    // 				} else {
    // 					dst.Set(x, y, color.Transparent)
    // 				}
    // 			case ma == m && op == Src:
    // 				dst.Set(x, y, src.At(sx, sy))
    // 			default:
    // 				sr, sg, sb, sa := src.At(sx, sy).RGBA()
    // 				if op == Over {
    // 					dr, dg, db, da := dst.At(x, y).RGBA()
    // 					a := m - (sa * ma / m)
    // 					out.R = uint16((dr*a + sr*ma) / m)
    // 					out.G = uint16((dg*a + sg*ma) / m)
    // 					out.B = uint16((db*a + sb*ma) / m)
    // 					out.A = uint16((da*a + sa*ma) / m)
    // 				} else {
    // 					out.R = uint16(sr * ma / m)
    // 					out.G = uint16(sg * ma / m)
    // 					out.B = uint16(sb * ma / m)
    // 					out.A = uint16(sa * ma / m)
    // 				}
    // 				// The third argument is &out instead of out (and out is
    // 				// declared outside of the inner loop) to avoid the implicit
    // 				// conversion to color.Color here allocating memory in the
    // 				// inner loop if sizeof(color.RGBA64) > sizeof(uintptr).
    // 				dst.Set(x, y, &out)
    // 			}
    // 		}
    // 	}
}

// fn drawFillOver(dst *image.RGBA, r: &Rectangle, sr, sg, sb, sa uint32) {
// 	// The 0x101 is here for the same reason as in drawRGBA.
// 	a := (m - sa) * 0x101
// 	i0 := dst.pix_offset(r.min.x, r.min.y)
// 	i1 := i0 + r.dx()*4
// 	for y := r.min.y; y != r.max.y; y++ {
// 		for i := i0; i < i1; i += 4 {
// 			dr := &dst.pix[i+0]
// 			dg := &dst.pix[i+1]
// 			db := &dst.pix[i+2]
// 			da := &dst.pix[i+3]

// 			*dr = uint8((uint32(*dr)*a/m + sr) >> 8)
// 			*dg = uint8((uint32(*dg)*a/m + sg) >> 8)
// 			*db = uint8((uint32(*db)*a/m + sb) >> 8)
// 			*da = uint8((uint32(*da)*a/m + sa) >> 8)
// 		}
// 		i0 += dst.stride
// 		i1 += dst.stride
// 	}
// }

// fn drawFillSrc(dst *image.RGBA, r: &Rectangle, sr, sg, sb, sa uint32) {
// 	sr8 := uint8(sr >> 8)
// 	sg8 := uint8(sg >> 8)
// 	sb8 := uint8(sb >> 8)
// 	sa8 := uint8(sa >> 8)
// 	// The built-in copy function is faster than a straightforward for loop to fill the destination with
// 	// the color, but copy requires a slice source. We therefore use a for loop to fill the first row, and
// 	// then use the first row as the slice source for the remaining rows.
// 	i0 := dst.pix_offset(r.min.x, r.min.y)
// 	i1 := i0 + r.dx()*4
// 	for i := i0; i < i1; i += 4 {
// 		dst.pix[i+0] = sr8
// 		dst.pix[i+1] = sg8
// 		dst.pix[i+2] = sb8
// 		dst.pix[i+3] = sa8
// 	}
// 	firstRow := dst.pix[i0:i1]
// 	for y := r.min.y + 1; y < r.max.y; y++ {
// 		i0 += dst.stride
// 		i1 += dst.stride
// 		copy(dst.pix[i0:i1], firstRow)
// 	}
// }

// fn drawCopyOver(dst *image.RGBA, r: &Rectangle, src *image.RGBA, sp: &Point) {
// 	dx, dy := r.dx(), r.dy()
// 	d0 := dst.pix_offset(r.min.x, r.min.y)
// 	s0 := src.pix_offset(sp.x, sp.y)
// 	var (
// 		ddelta, sdelta int
// 		i0, i1, idelta int
// 	)
// 	if r.min.y < sp.y || r.min.y == sp.y && r.min.x <= sp.x {
// 		ddelta = dst.stride
// 		sdelta = src.stride
// 		i0, i1, idelta = 0, dx*4, +4
// 	} else {
// 		// If the source start point is higher than the destination start point, or equal height but to the left,
// 		// then we compose the rows in right-to-left, bottom-up order instead of left-to-right, top-down.
// 		d0 += (dy - 1) * dst.stride
// 		s0 += (dy - 1) * src.stride
// 		ddelta = -dst.stride
// 		sdelta = -src.stride
// 		i0, i1, idelta = (dx-1)*4, -4, -4
// 	}
// 	for ; dy > 0; dy-- {
// 		dpix := dst.pix[d0..];
// 		spix := src.pix[s0..];
// 		for i := i0; i != i1; i += idelta {
// 			s := spix[i : i+4 : i+4] // Small cap improves performance, see https://golang.org/issue/27857
// 			sr := uint32(s[0]) * 0x101
// 			sg := uint32(s[1]) * 0x101
// 			sb := uint32(s[2]) * 0x101
// 			sa := uint32(s[3]) * 0x101

// 			// The 0x101 is here for the same reason as in drawRGBA.
// 			a := (m - sa) * 0x101

// 			d := dpix[i : i+4 : i+4] // Small cap improves performance, see https://golang.org/issue/27857
// 			d[0] = uint8((uint32(d[0])*a/m + sr) >> 8)
// 			d[1] = uint8((uint32(d[1])*a/m + sg) >> 8)
// 			d[2] = uint8((uint32(d[2])*a/m + sb) >> 8)
// 			d[3] = uint8((uint32(d[3])*a/m + sa) >> 8)
// 		}
// 		d0 += ddelta
// 		s0 += sdelta
// 	}
// }

/// draw_copy_src copies bytes to dstPix from srcPix. These arguments roughly
/// correspond to the Pix fields of the image package's concrete image.Image
/// implementations, but are offset (dstPix is dst.pix[dpOffset..] not dst.pix).;
fn draw_copy_src(
    dst_pix: &mut [u8],
    dst_stride: usize,
    r: &Rectangle,
    src_pix: &[u8],
    src_stride: usize,
    sp: &Point,
    bytes_per_row: usize,
) {
    let (mut d0, mut s0, mut ddelta, mut sdelta, mut dy) =
        (0, 0, dst_stride as isize, src_stride as isize, r.dy());
    if r.min.y > sp.y {
        // If the source start point is higher than the destination start
        // point, then we compose the rows in bottom-up order instead of
        // top-down. Unlike the drawCopyOver function, we don't have to check
        // the x coordinates because the built-in copy function can handle
        // overlapping slices.
        d0 = (dy - 1) * dst_stride;
        s0 = (dy - 1) * src_stride;
        ddelta = -(dst_stride as isize);
        sdelta = -(src_stride as isize);
    }
    while dy > 0 {
        compat::copy(
            &mut dst_pix[d0..d0 + bytes_per_row],
            &src_pix[s0..s0 + bytes_per_row],
        );
        d0 = ((d0 as isize) + ddelta) as usize;
        s0 = ((s0 as isize) + sdelta) as usize;
        dy -= 1;
    }
}

// fn drawNRGBAOver(dst *image.RGBA, r: &Rectangle, src *image.NRGBA, sp: &Point) {
// 	i0 := (r.min.x - dst.rect.min.x) * 4
// 	i1 := (r.max.x - dst.rect.min.x) * 4
// 	si0 := (sp.x - src.rect.min.x) * 4
// 	yMax := r.max.y - dst.rect.min.y

// 	y := r.min.y - dst.rect.min.y
// 	sy := sp.y - src.rect.min.y
// 	for ; y != yMax; y, sy = y+1, sy+1 {
// 		dpix := dst.pix[y*dst.stride..];
// 		spix := src.pix[sy*src.stride..];

// 		for i, si := i0, si0; i < i1; i, si = i+4, si+4 {
// 			// Convert from non-premultiplied color to pre-multiplied color.
// 			s := spix[si : si+4 : si+4] // Small cap improves performance, see https://golang.org/issue/27857
// 			sa := uint32(s[3]) * 0x101
// 			sr := uint32(s[0]) * sa / 0xff
// 			sg := uint32(s[1]) * sa / 0xff
// 			sb := uint32(s[2]) * sa / 0xff

// 			d := dpix[i : i+4 : i+4] // Small cap improves performance, see https://golang.org/issue/27857
// 			dr := uint32(d[0])
// 			dg := uint32(d[1])
// 			db := uint32(d[2])
// 			da := uint32(d[3])

// 			// The 0x101 is here for the same reason as in drawRGBA.
// 			a := (m - sa) * 0x101

// 			d[0] = uint8((dr*a/m + sr) >> 8)
// 			d[1] = uint8((dg*a/m + sg) >> 8)
// 			d[2] = uint8((db*a/m + sb) >> 8)
// 			d[3] = uint8((da*a/m + sa) >> 8)
// 		}
// 	}
// }

fn draw_nrgba_src(dst: &mut image::RGBA, r: &Rectangle, src: &image::NRGBA, sp: &Point) {
    let i0 = (r.min.x - dst.rect.min.x) as usize * 4;
    let i1 = (r.max.x - dst.rect.min.x) as usize * 4;
    let si0 = (sp.x - src.rect.min.x) as usize * 4;
    let y_max = (r.max.y - dst.rect.min.y) as usize;

    let mut y = (r.min.y - dst.rect.min.y) as usize;
    let mut sy = (sp.y - src.rect.min.y) as usize;
    while y != y_max {
        let dpix = &mut dst.pix[y * dst.stride..];
        let spix = &src.pix[(sy * src.stride) as usize..];
        let (mut i, mut si) = (i0, si0);
        while i < i1 {
            // Convert from non-premultiplied color to pre-multiplied color.
            let s = &spix[si..si + 4];
            let sa = (s[3] as u32) * 0x101;
            let sr = (s[0] as u32) * sa / 0xff;
            let sg = (s[1] as u32) * sa / 0xff;
            let sb = (s[2] as u32) * sa / 0xff;

            let d = &mut dpix[i..i + 4];
            d[0] = (sr >> 8) as u8;
            d[1] = (sg >> 8) as u8;
            d[2] = (sb >> 8) as u8;
            d[3] = (sa >> 8) as u8;
            (i, si) = (i + 4, si + 4);
        }
        (y, sy) = (y + 1, sy + 1);
    }
}

// fn drawGray(dst *image.RGBA, r: &Rectangle, src *image.Gray, sp: &Point) {
// 	i0 := (r.min.x - dst.rect.min.x) * 4
// 	i1 := (r.max.x - dst.rect.min.x) * 4
// 	si0 := (sp.x - src.rect.min.x) * 1
// 	yMax := r.max.y - dst.rect.min.y

// 	y := r.min.y - dst.rect.min.y
// 	sy := sp.y - src.rect.min.y
// 	for ; y != yMax; y, sy = y+1, sy+1 {
// 		dpix := dst.pix[y*dst.stride..];
// 		spix := src.pix[sy*src.stride..];

// 		for i, si := i0, si0; i < i1; i, si = i+4, si+1 {
// 			p := spix[si]
// 			d := dpix[i : i+4 : i+4] // Small cap improves performance, see https://golang.org/issue/27857
// 			d[0] = p
// 			d[1] = p
// 			d[2] = p
// 			d[3] = 255
// 		}
// 	}
// }

// fn drawCMYK(dst *image.RGBA, r: &Rectangle, src *image.CMYK, sp: &Point) {
// 	i0 := (r.min.x - dst.rect.min.x) * 4
// 	i1 := (r.max.x - dst.rect.min.x) * 4
// 	si0 := (sp.x - src.rect.min.x) * 4
// 	yMax := r.max.y - dst.rect.min.y

// 	y := r.min.y - dst.rect.min.y
// 	sy := sp.y - src.rect.min.y
// 	for ; y != yMax; y, sy = y+1, sy+1 {
// 		dpix := dst.pix[y*dst.stride..];
// 		spix := src.pix[sy*src.stride..];

// 		for i, si := i0, si0; i < i1; i, si = i+4, si+4 {
// 			s := spix[si : si+4 : si+4] // Small cap improves performance, see https://golang.org/issue/27857
// 			d := dpix[i : i+4 : i+4]
// 			d[0], d[1], d[2] = color.CMYKToRGB(s[0], s[1], s[2], s[3])
// 			d[3] = 255
// 		}
// 	}
// }

// fn drawGlyphOver(dst *image.RGBA, r: &Rectangle, src *image.Uniform, mask *image.Alpha, mp: &Point) {
// 	i0 := dst.pix_offset(r.min.x, r.min.y)
// 	i1 := i0 + r.dx()*4
// 	mi0 := mask.pix_offset(mp.x, mp.y)
// 	sr, sg, sb, sa := src.RGBA()
// 	for y, my := r.min.y, mp.y; y != r.max.y; y, my = y+1, my+1 {
// 		for i, mi := i0, mi0; i < i1; i, mi = i+4, mi+1 {
// 			ma := uint32(mask.pix[mi])
// 			if ma == 0 {
// 				continue
// 			}
// 			ma |= ma << 8

// 			// The 0x101 is here for the same reason as in drawRGBA.
// 			a := (m - (sa * ma / m)) * 0x101

// 			d := dst.pix[i : i+4 : i+4] // Small cap improves performance, see https://golang.org/issue/27857
// 			d[0] = uint8((uint32(d[0])*a + sr*ma) / m >> 8)
// 			d[1] = uint8((uint32(d[1])*a + sg*ma) / m >> 8)
// 			d[2] = uint8((uint32(d[2])*a + sb*ma) / m >> 8)
// 			d[3] = uint8((uint32(d[3])*a + sa*ma) / m >> 8)
// 		}
// 		i0 += dst.stride
// 		i1 += dst.stride
// 		mi0 += mask.stride
// 	}
// }

// fn drawGrayMaskOver(dst *image.RGBA, r: &Rectangle, src *image.Gray, sp: &Point, mask *image.Alpha, mp: &Point) {
// 	x0, x1, dx := r.min.x, r.max.x, 1
// 	y0, y1, dy := r.min.y, r.max.y, 1
// 	if r.overlaps(r.add(sp.sub(r.min))) {
// 		if sp.y < r.min.y || sp.y == r.min.y && sp.x < r.min.x {
// 			x0, x1, dx = x1-1, x0-1, -1
// 			y0, y1, dy = y1-1, y0-1, -1
// 		}
// 	}

// 	sy := sp.y + y0 - r.min.y
// 	my := mp.y + y0 - r.min.y
// 	sx0 := sp.x + x0 - r.min.x
// 	mx0 := mp.x + x0 - r.min.x
// 	sx1 := sx0 + (x1 - x0)
// 	i0 := dst.pix_offset(x0, y0)
// 	di := dx * 4
// 	for y := y0; y != y1; y, sy, my = y+dy, sy+dy, my+dy {
// 		for i, sx, mx := i0, sx0, mx0; sx != sx1; i, sx, mx = i+di, sx+dx, mx+dx {
// 			mi := mask.pix_offset(mx, my)
// 			ma := uint32(mask.pix[mi])
// 			ma |= ma << 8
// 			si := src.pix_offset(sx, sy)
// 			sy := uint32(src.pix[si])
// 			sy |= sy << 8
// 			sa := uint32(0xffff)

// 			d := dst.pix[i : i+4 : i+4] // Small cap improves performance, see https://golang.org/issue/27857
// 			dr := uint32(d[0])
// 			dg := uint32(d[1])
// 			db := uint32(d[2])
// 			da := uint32(d[3])

// 			// dr, dg, db and da are all 8-bit color at the moment, ranging in [0,255].
// 			// We work in 16-bit color, and so would normally do:
// 			// dr |= dr << 8
// 			// and similarly for dg, db and da, but instead we multiply a
// 			// (which is a 16-bit color, ranging in [0,65535]) by 0x101.
// 			// This yields the same result, but is fewer arithmetic operations.
// 			a := (m - (sa * ma / m)) * 0x101

// 			d[0] = uint8((dr*a + sy*ma) / m >> 8)
// 			d[1] = uint8((dg*a + sy*ma) / m >> 8)
// 			d[2] = uint8((db*a + sy*ma) / m >> 8)
// 			d[3] = uint8((da*a + sa*ma) / m >> 8)
// 		}
// 		i0 += dy * dst.stride
// 	}
// }

// fn drawRGBAMaskOver(dst *image.RGBA, r: &Rectangle, src *image.RGBA, sp: &Point, mask *image.Alpha, mp: &Point) {
// 	x0, x1, dx := r.min.x, r.max.x, 1
// 	y0, y1, dy := r.min.y, r.max.y, 1
// 	if dst == src && r.overlaps(r.add(sp.sub(r.min))) {
// 		if sp.y < r.min.y || sp.y == r.min.y && sp.x < r.min.x {
// 			x0, x1, dx = x1-1, x0-1, -1
// 			y0, y1, dy = y1-1, y0-1, -1
// 		}
// 	}

// 	sy := sp.y + y0 - r.min.y
// 	my := mp.y + y0 - r.min.y
// 	sx0 := sp.x + x0 - r.min.x
// 	mx0 := mp.x + x0 - r.min.x
// 	sx1 := sx0 + (x1 - x0)
// 	i0 := dst.pix_offset(x0, y0)
// 	di := dx * 4
// 	for y := y0; y != y1; y, sy, my = y+dy, sy+dy, my+dy {
// 		for i, sx, mx := i0, sx0, mx0; sx != sx1; i, sx, mx = i+di, sx+dx, mx+dx {
// 			mi := mask.pix_offset(mx, my)
// 			ma := uint32(mask.pix[mi])
// 			ma |= ma << 8
// 			si := src.pix_offset(sx, sy)
// 			sr := uint32(src.pix[si+0])
// 			sg := uint32(src.pix[si+1])
// 			sb := uint32(src.pix[si+2])
// 			sa := uint32(src.pix[si+3])
// 			sr |= sr << 8
// 			sg |= sg << 8
// 			sb |= sb << 8
// 			sa |= sa << 8
// 			d := dst.pix[i : i+4 : i+4] // Small cap improves performance, see https://golang.org/issue/27857
// 			dr := uint32(d[0])
// 			dg := uint32(d[1])
// 			db := uint32(d[2])
// 			da := uint32(d[3])

// 			// dr, dg, db and da are all 8-bit color at the moment, ranging in [0,255].
// 			// We work in 16-bit color, and so would normally do:
// 			// dr |= dr << 8
// 			// and similarly for dg, db and da, but instead we multiply a
// 			// (which is a 16-bit color, ranging in [0,65535]) by 0x101.
// 			// This yields the same result, but is fewer arithmetic operations.
// 			a := (m - (sa * ma / m)) * 0x101

// 			d[0] = uint8((dr*a + sr*ma) / m >> 8)
// 			d[1] = uint8((dg*a + sg*ma) / m >> 8)
// 			d[2] = uint8((db*a + sb*ma) / m >> 8)
// 			d[3] = uint8((da*a + sa*ma) / m >> 8)
// 		}
// 		i0 += dy * dst.stride
// 	}
// }

// fn drawRGBA64ImageMaskOver(dst *image.RGBA, r: &Rectangle, src image.RGBA64Image, sp: &Point, mask *image.Alpha, mp: &Point) {
// 	x0, x1, dx := r.min.x, r.max.x, 1
// 	y0, y1, dy := r.min.y, r.max.y, 1
// 	if image.Image(dst) == src && r.overlaps(r.add(sp.sub(r.min))) {
// 		if sp.y < r.min.y || sp.y == r.min.y && sp.x < r.min.x {
// 			x0, x1, dx = x1-1, x0-1, -1
// 			y0, y1, dy = y1-1, y0-1, -1
// 		}
// 	}

// 	sy := sp.y + y0 - r.min.y
// 	my := mp.y + y0 - r.min.y
// 	sx0 := sp.x + x0 - r.min.x
// 	mx0 := mp.x + x0 - r.min.x
// 	sx1 := sx0 + (x1 - x0)
// 	i0 := dst.pix_offset(x0, y0)
// 	di := dx * 4
// 	for y := y0; y != y1; y, sy, my = y+dy, sy+dy, my+dy {
// 		for i, sx, mx := i0, sx0, mx0; sx != sx1; i, sx, mx = i+di, sx+dx, mx+dx {
// 			mi := mask.pix_offset(mx, my)
// 			ma := uint32(mask.pix[mi])
// 			ma |= ma << 8
// 			srgba := src.RGBA64At(sx, sy)
// 			d := dst.pix[i : i+4 : i+4] // Small cap improves performance, see https://golang.org/issue/27857
// 			dr := uint32(d[0])
// 			dg := uint32(d[1])
// 			db := uint32(d[2])
// 			da := uint32(d[3])

// 			// dr, dg, db and da are all 8-bit color at the moment, ranging in [0,255].
// 			// We work in 16-bit color, and so would normally do:
// 			// dr |= dr << 8
// 			// and similarly for dg, db and da, but instead we multiply a
// 			// (which is a 16-bit color, ranging in [0,65535]) by 0x101.
// 			// This yields the same result, but is fewer arithmetic operations.
// 			a := (m - (uint32(srgba.A) * ma / m)) * 0x101

// 			d[0] = uint8((dr*a + uint32(srgba.R)*ma) / m >> 8)
// 			d[1] = uint8((dg*a + uint32(srgba.G)*ma) / m >> 8)
// 			d[2] = uint8((db*a + uint32(srgba.B)*ma) / m >> 8)
// 			d[3] = uint8((da*a + uint32(srgba.A)*ma) / m >> 8)
// 		}
// 		i0 += dy * dst.stride
// 	}
// }

// fn drawRGBA(dst *image.RGBA, r: &Rectangle, src: &Img, sp: &Point, mask: &Img, mp: &Point, op: Op) {
// 	x0, x1, dx := r.min.x, r.max.x, 1
// 	y0, y1, dy := r.min.y, r.max.y, 1
// 	if image.Image(dst) == src && r.overlaps(r.add(sp.sub(r.min))) {
// 		if sp.y < r.min.y || sp.y == r.min.y && sp.x < r.min.x {
// 			x0, x1, dx = x1-1, x0-1, -1
// 			y0, y1, dy = y1-1, y0-1, -1
// 		}
// 	}

// 	sy := sp.y + y0 - r.min.y
// 	my := mp.y + y0 - r.min.y
// 	sx0 := sp.x + x0 - r.min.x
// 	mx0 := mp.x + x0 - r.min.x
// 	sx1 := sx0 + (x1 - x0)
// 	i0 := dst.pix_offset(x0, y0)
// 	di := dx * 4

// 	// Try the image.RGBA64Image interface, part of the standard library since
// 	// Go 1.17.
// 	//
// 	// This optimization is similar to how FALLBACK1.17 optimizes FALLBACK1.0
// 	// in draw_mask, except here the concrete type of dst is known to be
// 	// *image.RGBA.
// 	if src0, _ := src.(image.RGBA64Image); src0 != nil {
// 		if mask == nil {
// 			if op == Over {
// 				for y := y0; y != y1; y, sy, my = y+dy, sy+dy, my+dy {
// 					for i, sx, mx := i0, sx0, mx0; sx != sx1; i, sx, mx = i+di, sx+dx, mx+dx {
// 						srgba := src0.RGBA64At(sx, sy)
// 						d := dst.pix[i : i+4 : i+4]
// 						dr := uint32(d[0])
// 						dg := uint32(d[1])
// 						db := uint32(d[2])
// 						da := uint32(d[3])
// 						a := (m - uint32(srgba.A)) * 0x101
// 						d[0] = uint8((dr*a/m + uint32(srgba.R)) >> 8)
// 						d[1] = uint8((dg*a/m + uint32(srgba.G)) >> 8)
// 						d[2] = uint8((db*a/m + uint32(srgba.B)) >> 8)
// 						d[3] = uint8((da*a/m + uint32(srgba.A)) >> 8)
// 					}
// 					i0 += dy * dst.stride
// 				}
// 			} else {
// 				for y := y0; y != y1; y, sy, my = y+dy, sy+dy, my+dy {
// 					for i, sx, mx := i0, sx0, mx0; sx != sx1; i, sx, mx = i+di, sx+dx, mx+dx {
// 						srgba := src0.RGBA64At(sx, sy)
// 						d := dst.pix[i : i+4 : i+4]
// 						d[0] = uint8(srgba.R >> 8)
// 						d[1] = uint8(srgba.G >> 8)
// 						d[2] = uint8(srgba.B >> 8)
// 						d[3] = uint8(srgba.A >> 8)
// 					}
// 					i0 += dy * dst.stride
// 				}
// 			}
// 			return

// 		} else if mask0, _ := mask.(image.RGBA64Image); mask0 != nil {
// 			if op == Over {
// 				for y := y0; y != y1; y, sy, my = y+dy, sy+dy, my+dy {
// 					for i, sx, mx := i0, sx0, mx0; sx != sx1; i, sx, mx = i+di, sx+dx, mx+dx {
// 						ma := uint32(mask0.RGBA64At(mx, my).A)
// 						srgba := src0.RGBA64At(sx, sy)
// 						d := dst.pix[i : i+4 : i+4]
// 						dr := uint32(d[0])
// 						dg := uint32(d[1])
// 						db := uint32(d[2])
// 						da := uint32(d[3])
// 						a := (m - (uint32(srgba.A) * ma / m)) * 0x101
// 						d[0] = uint8((dr*a + uint32(srgba.R)*ma) / m >> 8)
// 						d[1] = uint8((dg*a + uint32(srgba.G)*ma) / m >> 8)
// 						d[2] = uint8((db*a + uint32(srgba.B)*ma) / m >> 8)
// 						d[3] = uint8((da*a + uint32(srgba.A)*ma) / m >> 8)
// 					}
// 					i0 += dy * dst.stride
// 				}
// 			} else {
// 				for y := y0; y != y1; y, sy, my = y+dy, sy+dy, my+dy {
// 					for i, sx, mx := i0, sx0, mx0; sx != sx1; i, sx, mx = i+di, sx+dx, mx+dx {
// 						ma := uint32(mask0.RGBA64At(mx, my).A)
// 						srgba := src0.RGBA64At(sx, sy)
// 						d := dst.pix[i : i+4 : i+4]
// 						d[0] = uint8(uint32(srgba.R) * ma / m >> 8)
// 						d[1] = uint8(uint32(srgba.G) * ma / m >> 8)
// 						d[2] = uint8(uint32(srgba.B) * ma / m >> 8)
// 						d[3] = uint8(uint32(srgba.A) * ma / m >> 8)
// 					}
// 					i0 += dy * dst.stride
// 				}
// 			}
// 			return
// 		}
// 	}

// 	// Use the image.Image interface, part of the standard library since Go
// 	// 1.0.
// 	//
// 	// This is similar to FALLBACK1.0 in draw_mask, except here the concrete
// 	// type of dst is known to be *image.RGBA.
// 	for y := y0; y != y1; y, sy, my = y+dy, sy+dy, my+dy {
// 		for i, sx, mx := i0, sx0, mx0; sx != sx1; i, sx, mx = i+di, sx+dx, mx+dx {
// 			ma := uint32(m)
// 			if mask != nil {
// 				_, _, _, ma = mask.At(mx, my).RGBA()
// 			}
// 			sr, sg, sb, sa := src.At(sx, sy).RGBA()
// 			d := dst.pix[i : i+4 : i+4] // Small cap improves performance, see https://golang.org/issue/27857
// 			if op == Over {
// 				dr := uint32(d[0])
// 				dg := uint32(d[1])
// 				db := uint32(d[2])
// 				da := uint32(d[3])

// 				// dr, dg, db and da are all 8-bit color at the moment, ranging in [0,255].
// 				// We work in 16-bit color, and so would normally do:
// 				// dr |= dr << 8
// 				// and similarly for dg, db and da, but instead we multiply a
// 				// (which is a 16-bit color, ranging in [0,65535]) by 0x101.
// 				// This yields the same result, but is fewer arithmetic operations.
// 				a := (m - (sa * ma / m)) * 0x101

// 				d[0] = uint8((dr*a + sr*ma) / m >> 8)
// 				d[1] = uint8((dg*a + sg*ma) / m >> 8)
// 				d[2] = uint8((db*a + sb*ma) / m >> 8)
// 				d[3] = uint8((da*a + sa*ma) / m >> 8)

// 			} else {
// 				d[0] = uint8(sr * ma / m >> 8)
// 				d[1] = uint8(sg * ma / m >> 8)
// 				d[2] = uint8(sb * ma / m >> 8)
// 				d[3] = uint8(sa * ma / m >> 8)
// 			}
// 		}
// 		i0 += dy * dst.stride
// 	}
// }

// // clamp clamps i to the interval [0, 0xffff].
// fn clamp(i int32) int32 {
// 	if i < 0 {
// 		return 0
// 	}
// 	if i > 0xffff {
// 		return 0xffff
// 	}
// 	return i
// }

// // sqDiff returns the squared-difference of x and y, shifted by 2 so that
// // adding four of those won't overflow a uint32.
// //
// // x and y are both assumed to be in the range [0, 0xffff].
// fn sqDiff(x, y int32) uint32 {
// 	// This is an optimized code relying on the overflow/wrap around
// 	// properties of unsigned integers operations guaranteed by the language
// 	// spec. See sqDiff from the image/color package for more details.
// 	d := uint32(x - y)
// 	return (d * d) >> 2
// }

// fn drawPaletted(dst: &mut Img, r: &Rectangle, src: &Img, sp: &Point, floydSteinberg bool) {
// 	// TODO(nigeltao): handle the case where the dst and src overlap.
// 	// Does it even make sense to try and do Floyd-Steinberg whilst
// 	// walking the image backward (right-to-left bottom-to-top)?

// 	// If dst is an *image.Paletted, we have a fast path for dst.Set and
// 	// dst.At. The dst.Set equivalent is a batch version of the algorithm
// 	// used by color.Palette's Index method in image/color/color.go, plus
// 	// optional Floyd-Steinberg error diffusion.
// 	palette, pix, stride := [][4]int32(nil), [u8](nil), 0
// 	if p, ok := dst.(*image.Paletted); ok {
// 		palette = make([][4]int32, len(p.Palette))
// 		for i, col := range p.Palette {
// 			r, g, b, a := col.RGBA()
// 			palette[i][0] = int32(r)
// 			palette[i][1] = int32(g)
// 			palette[i][2] = int32(b)
// 			palette[i][3] = int32(a)
// 		}
// 		pix, stride = p.pix[p.pix_offset(r.min.x, r.min.y)..], p.stride;
// 	}

// 	// quantErrorCurr and quantErrorNext are the Floyd-Steinberg quantization
// 	// errors that have been propagated to the pixels in the current and next
// 	// rows. The +2 simplifies calculation near the edges.
// 	var quantErrorCurr, quantErrorNext [][4]int32
// 	if floydSteinberg {
// 		quantErrorCurr = make([][4]int32, r.dx()+2)
// 		quantErrorNext = make([][4]int32, r.dx()+2)
// 	}
// 	pxRGBA := fn(x, y int) (r, g, b, a uint32) { return src.At(x, y).RGBA() }
// 	// Fast paths for special cases to avoid excessive use of the color.Color
// 	// interface which escapes to the heap but need to be discovered for
// 	// each pixel on r. See also https://golang.org/issues/15759.
// 	switch src0 := src.(type) {
// 	case *image.RGBA:
// 		pxRGBA = fn(x, y int) (r, g, b, a uint32) { return src0.RGBAAt(x, y).RGBA() }
// 	case *image.NRGBA:
// 		pxRGBA = fn(x, y int) (r, g, b, a uint32) { return src0.NRGBAAt(x, y).RGBA() }
// 	case *image.YCbCr:
// 		pxRGBA = fn(x, y int) (r, g, b, a uint32) { return src0.YCbCrAt(x, y).RGBA() }
// 	}

// 	// Loop over each source pixel.
// 	out := color.RGBA64{A: 0xffff}
// 	for y := 0; y != r.dy(); y++ {
// 		for x := 0; x != r.dx(); x++ {
// 			// er, eg and eb are the pixel's R,G,B values plus the
// 			// optional Floyd-Steinberg error.
// 			sr, sg, sb, sa := pxRGBA(sp.x+x, sp.y+y)
// 			er, eg, eb, ea := int32(sr), int32(sg), int32(sb), int32(sa)
// 			if floydSteinberg {
// 				er = clamp(er + quantErrorCurr[x+1][0]/16)
// 				eg = clamp(eg + quantErrorCurr[x+1][1]/16)
// 				eb = clamp(eb + quantErrorCurr[x+1][2]/16)
// 				ea = clamp(ea + quantErrorCurr[x+1][3]/16)
// 			}

// 			if palette != nil {
// 				// Find the closest palette color in Euclidean R,G,B,A space:
// 				// the one that minimizes sum-squared-difference.
// 				// TODO(nigeltao): consider smarter algorithms.
// 				bestIndex, bestSum := 0, uint32(1<<32-1)
// 				for index, p := range palette {
// 					sum := sqDiff(er, p[0]) + sqDiff(eg, p[1]) + sqDiff(eb, p[2]) + sqDiff(ea, p[3])
// 					if sum < bestSum {
// 						bestIndex, bestSum = index, sum
// 						if sum == 0 {
// 							break
// 						}
// 					}
// 				}
// 				pix[y*stride+x] = byte(bestIndex)

// 				if !floydSteinberg {
// 					continue
// 				}
// 				er -= palette[bestIndex][0]
// 				eg -= palette[bestIndex][1]
// 				eb -= palette[bestIndex][2]
// 				ea -= palette[bestIndex][3]

// 			} else {
// 				out.R = uint16(er)
// 				out.G = uint16(eg)
// 				out.B = uint16(eb)
// 				out.A = uint16(ea)
// 				// The third argument is &out instead of out (and out is
// 				// declared outside of the inner loop) to avoid the implicit
// 				// conversion to color.Color here allocating memory in the
// 				// inner loop if sizeof(color.RGBA64) > sizeof(uintptr).
// 				dst.Set(r.min.x+x, r.min.y+y, &out)

// 				if !floydSteinberg {
// 					continue
// 				}
// 				sr, sg, sb, sa = dst.At(r.min.x+x, r.min.y+y).RGBA()
// 				er -= int32(sr)
// 				eg -= int32(sg)
// 				eb -= int32(sb)
// 				ea -= int32(sa)
// 			}

// 			// Propagate the Floyd-Steinberg quantization error.
// 			quantErrorNext[x+0][0] += er * 3
// 			quantErrorNext[x+0][1] += eg * 3
// 			quantErrorNext[x+0][2] += eb * 3
// 			quantErrorNext[x+0][3] += ea * 3
// 			quantErrorNext[x+1][0] += er * 5
// 			quantErrorNext[x+1][1] += eg * 5
// 			quantErrorNext[x+1][2] += eb * 5
// 			quantErrorNext[x+1][3] += ea * 5
// 			quantErrorNext[x+2][0] += er * 1
// 			quantErrorNext[x+2][1] += eg * 1
// 			quantErrorNext[x+2][2] += eb * 1
// 			quantErrorNext[x+2][3] += ea * 1
// 			quantErrorCurr[x+2][0] += er * 7
// 			quantErrorCurr[x+2][1] += eg * 7
// 			quantErrorCurr[x+2][2] += eb * 7
// 			quantErrorCurr[x+2][3] += ea * 7
// 		}

// 		// Recycle the quantization error buffers.
// 		if floydSteinberg {
// 			quantErrorCurr, quantErrorNext = quantErrorNext, quantErrorCurr
// 			for i := range quantErrorNext {
// 				quantErrorNext[i] = [4]int32{}
// 			}
// 		}
// 	}
// }
