// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

// import (
// 	"errors"
// 	"io"
// 	"math"
// 	"reflect"
// 	"sync"
// )

/// A ByteOrder specifies how to convert u8 slices into
/// 16-, 32-, or 64-bit unsigned integers.
pub trait ByteOrder {
    // 	Uint16(b: &[u8]) u16
    fn uint32(&self, b: &[u8]) -> u32;
    fn uint64(&self, b: &[u8]) -> u64;
    // 	PutUint16(b: &[u8], u16)
    fn put_uint32(&self, b: &mut [u8], v: u32);
    fn put_uint64(&self, b: &mut [u8], v: u64);
    // 	String() string
}

// // AppendByteOrder specifies how to append 16-, 32-, or 64-bit unsigned integers
// // into a u8 slice.
// type AppendByteOrder interface {
// 	AppendUint16([]u8, u16) []u8
// 	AppendUint32([]u8, u32) []u8
// 	AppendUint64([]u8, u64) []u8
// 	String() string
// }

/// LittleEndian is the little-endian implementation of ByteOrder and AppendByteOrder.
pub struct LittleEndian {}

pub const LITTLE_ENDIAN: LittleEndian = LittleEndian {};

impl ByteOrder for LittleEndian {
    fn uint32(&self, b: &[u8]) -> u32 {
        (b[0] as u32) | (b[1] as u32) << 8 | (b[2] as u32) << 16 | (b[3] as u32) << 24
    }

    fn put_uint32(&self, b: &mut [u8], v: u32) {
        b[0] = (v) as u8;
        b[1] = (v >> 8) as u8;
        b[2] = (v >> 16) as u8;
        b[3] = (v >> 24) as u8;
    }

    fn uint64(&self, b: &[u8]) -> u64 {
        (b[0] as u64)
            | (b[1] as u64) << 8
            | (b[2] as u64) << 16
            | (b[3] as u64) << 24
            | (b[4] as u64) << 32
            | (b[5] as u64) << 40
            | (b[6] as u64) << 48
            | (b[7] as u64) << 56
    }

    fn put_uint64(&self, b: &mut [u8], v: u64) {
        b[0] = (v) as u8;
        b[1] = (v >> 8) as u8;
        b[2] = (v >> 16) as u8;
        b[3] = (v >> 24) as u8;
        b[4] = (v >> 32) as u8;
        b[5] = (v >> 40) as u8;
        b[6] = (v >> 48) as u8;
        b[7] = (v >> 56) as u8;
    }
}

// var LittleEndian littleEndian

// type littleEndian struct{}

// fn (littleEndian) Uint16(b: &[u8]) u16 {
// 	_ = b[1] // bounds check hint to compiler; see golang.org/issue/14808
// 	return u16(b[0]) | u16(b[1])<<8
// }

// fn (littleEndian) PutUint16(b: &[u8], v: u16) {
// 	_ = b[1] // early bounds check to guarantee safety of writes below
// 	b[0] = u8(v)
// 	b[1] = u8(v >> 8)
// }

// fn (littleEndian) AppendUint16(b: &[u8], v: u16) []u8 {
// 	return append(b,
// 		u8(v),
// 		u8(v>>8),
// 	)
// }

// fn (littleEndian) AppendUint32(b: &[u8], v: u32) []u8 {
// 	return append(b,
// 		u8(v),
// 		u8(v>>8),
// 		u8(v>>16),
// 		u8(v>>24),
// 	)
// }

// fn (littleEndian) AppendUint64(b: &[u8], v: u64) []u8 {
// 	return append(b,
// 		u8(v),
// 		u8(v>>8),
// 		u8(v>>16),
// 		u8(v>>24),
// 		u8(v>>32),
// 		u8(v>>40),
// 		u8(v>>48),
// 		u8(v>>56),
// 	)
// }

// fn (littleEndian) String() string { return "LittleEndian" }

// fn (littleEndian) GoString() string { return "binary.LittleEndian" }

/// BigEndian is the big-endian implementation of ByteOrder and AppendByteOrder.
pub struct BigEndian {}

pub const BIG_ENDIAN: BigEndian = BigEndian {};

impl ByteOrder for BigEndian {
    // pub fn Uint16(b: &[u8]) u16 {
    // 	_ = b[1] // bounds check hint to compiler; see golang.org/issue/14808
    // 	return u16(b[1]) | u16(b[0])<<8
    // }

    fn uint32(&self, b: &[u8]) -> u32 {
        (b[3] as u32) | (b[2] as u32) << 8 | (b[1] as u32) << 16 | (b[0] as u32) << 24
    }

    // fn PutUint16(b: &[u8], v: u16) {
    // 	_ = b[1] // early bounds check to guarantee safety of writes below
    // 	b[0] = u8(v >> 8)
    // 	b[1] = u8(v)
    // }

    fn put_uint32(&self, b: &mut [u8], v: u32) {
        b[0] = (v >> 24) as u8;
        b[1] = (v >> 16) as u8;
        b[2] = (v >> 8) as u8;
        b[3] = (v) as u8;
    }

    fn uint64(&self, b: &[u8]) -> u64 {
        (b[7] as u64)
            | (b[6] as u64) << 8
            | (b[5] as u64) << 16
            | (b[4] as u64) << 24
            | (b[3] as u64) << 32
            | (b[2] as u64) << 40
            | (b[1] as u64) << 48
            | (b[0] as u64) << 56
    }

    fn put_uint64(&self, b: &mut [u8], v: u64) {
        b[0] = (v >> 56) as u8;
        b[1] = (v >> 48) as u8;
        b[2] = (v >> 40) as u8;
        b[3] = (v >> 32) as u8;
        b[4] = (v >> 24) as u8;
        b[5] = (v >> 16) as u8;
        b[6] = (v >> 8) as u8;
        b[7] = (v) as u8;
    }
}

impl BigEndian {
    // pub fn AppendUint16(b: &[u8], v: u16) []u8 {
    // 	return append(b,
    // 		u8(v>>8),
    // 		u8(v),
    // 	)
    // }

    // pub fn AppendUint32(b: &[u8], v: u32) []u8 {
    // 	return append(b,
    // 		u8(v>>24),
    // 		u8(v>>16),
    // 		u8(v>>8),
    // 		u8(v),
    // 	)
    // }

    // pub fn AppendUint64(b: &[u8], v: u64) []u8 {
    // 	return append(b,
    // 		u8(v>>56),
    // 		u8(v>>48),
    // 		u8(v>>40),
    // 		u8(v>>32),
    // 		u8(v>>24),
    // 		u8(v>>16),
    // 		u8(v>>8),
    // 		u8(v),
    // 	)
    // }

    // pub fn String() string { return "BigEndian" }

    // pub fn GoString() string { return "binary::BigEndian" }
}

// // Read reads structured binary data from r into data.
// // Data must be a pointer to a fixed-size value or a slice
// // of fixed-size values.
// // Bytes read from r are decoded using the specified u8 order
// // and written to successive fields of the data.
// // When decoding boolean values, a zero u8 is decoded as false, and
// // any other non-zero u8 is decoded as true.
// // When reading into structs, the field data for fields with
// // blank (_) field names is skipped; i.e., blank field names
// // may be used for padding.
// // When reading into a struct, all non-blank fields must be exported
// // or Read may panic.
// //
// // The error is EOF only if no bytes were read.
// // If an EOF happens after reading some but not all the bytes,
// // Read returns ErrUnexpectedEOF.
// fn Read(r io.Reader, order ByteOrder, data any) error {
// 	// Fast path for basic types and slices.
// 	if n := intDataSize(data); n != 0 {
// 		bs := make([]u8, n)
// 		if _, err := io.read_full(r, bs); err != nil {
// 			return err
// 		}
// 		switch data := data.(type) {
// 		case *bool:
// 			*data = bs[0] != 0
// 		case *int8:
// 			*data = int8(bs[0])
// 		case *uint8:
// 			*data = bs[0]
// 		case *int16:
// 			*data = int16(order.Uint16(bs))
// 		case *u16:
// 			*data = order.Uint16(bs)
// 		case *int32:
// 			*data = int32(order.Uint32(bs))
// 		case *u32:
// 			*data = order.Uint32(bs)
// 		case *int64:
// 			*data = int64(order.Uint64(bs))
// 		case *u64:
// 			*data = order.Uint64(bs)
// 		case *float32:
// 			*data = math.Float32frombits(order.Uint32(bs))
// 		case *float64:
// 			*data = math.Float64frombits(order.Uint64(bs))
// 		case []bool:
// 			for i, x := range bs { // Easier to loop over the input for 8-bit values.
// 				data[i] = x != 0
// 			}
// 		case []int8:
// 			for i, x := range bs {
// 				data[i] = int8(x)
// 			}
// 		case []uint8:
// 			copy(data, bs)
// 		case []int16:
// 			for i := range data {
// 				data[i] = int16(order.Uint16(bs[2*i:]))
// 			}
// 		case []u16:
// 			for i := range data {
// 				data[i] = order.Uint16(bs[2*i:])
// 			}
// 		case []int32:
// 			for i := range data {
// 				data[i] = int32(order.Uint32(bs[4*i:]))
// 			}
// 		case []u32:
// 			for i := range data {
// 				data[i] = order.Uint32(bs[4*i:])
// 			}
// 		case []int64:
// 			for i := range data {
// 				data[i] = int64(order.Uint64(bs[8*i:]))
// 			}
// 		case []u64:
// 			for i := range data {
// 				data[i] = order.Uint64(bs[8*i:])
// 			}
// 		case []float32:
// 			for i := range data {
// 				data[i] = math.Float32frombits(order.Uint32(bs[4*i:]))
// 			}
// 		case []float64:
// 			for i := range data {
// 				data[i] = math.Float64frombits(order.Uint64(bs[8*i:]))
// 			}
// 		default:
// 			n = 0 // fast path doesn't apply
// 		}
// 		if n != 0 {
// 			return nil
// 		}
// 	}

// 	// Fallback to reflect-based decoding.
// 	v := reflect.ValueOf(data)
// 	size := -1
// 	switch v.Kind() {
// 	case reflect.Pointer:
// 		v = v.Elem()
// 		size = dataSize(v)
// 	case reflect.Slice:
// 		size = dataSize(v)
// 	}
// 	if size < 0 {
// 		return errors.New("binary.Read: invalid type " + reflect.TypeOf(data).String())
// 	}
// 	d := &decoder{order: order, buf: make([]u8, size)}
// 	if _, err := io.read_full(r, d.buf); err != nil {
// 		return err
// 	}
// 	d.value(v)
// 	return nil
// }

// // Write writes the binary representation of data into w.
// // Data must be a fixed-size value or a slice of fixed-size
// // values, or a pointer to such data.
// // Boolean values encode as one u8: 1 for true, and 0 for false.
// // Bytes written to w are encoded using the specified u8 order
// // and read from successive fields of the data.
// // When writing structs, zero values are written for fields
// // with blank (_) field names.
// fn Write(w: &mut dyn ggio::Writer, order ByteOrder, data any) error {
// 	// Fast path for basic types and slices.
// 	if n := intDataSize(data); n != 0 {
// 		bs := make([]u8, n)
// 		switch v := data.(type) {
// 		case *bool:
// 			if *v {
// 				bs[0] = 1
// 			} else {
// 				bs[0] = 0
// 			}
// 		case bool:
// 			if v {
// 				bs[0] = 1
// 			} else {
// 				bs[0] = 0
// 			}
// 		case []bool:
// 			for i, x := range v {
// 				if x {
// 					bs[i] = 1
// 				} else {
// 					bs[i] = 0
// 				}
// 			}
// 		case *int8:
// 			bs[0] = u8(*v)
// 		case int8:
// 			bs[0] = u8(v)
// 		case []int8:
// 			for i, x := range v {
// 				bs[i] = u8(x)
// 			}
// 		case *uint8:
// 			bs[0] = *v
// 		case uint8:
// 			bs[0] = v
// 		case []uint8:
// 			bs = v
// 		case *int16:
// 			order.PutUint16(bs, u16(*v))
// 		case int16:
// 			order.PutUint16(bs, u16(v))
// 		case []int16:
// 			for i, x := range v {
// 				order.PutUint16(bs[2*i:], u16(x))
// 			}
// 		case *u16:
// 			order.PutUint16(bs, *v)
// 		case u16:
// 			order.PutUint16(bs, v)
// 		case []u16:
// 			for i, x := range v {
// 				order.PutUint16(bs[2*i:], x)
// 			}
// 		case *int32:
// 			order.put_uint32(bs, u32(*v))
// 		case int32:
// 			order.put_uint32(bs, u32(v))
// 		case []int32:
// 			for i, x := range v {
// 				order.put_uint32(bs[4*i:], u32(x))
// 			}
// 		case *u32:
// 			order.put_uint32(bs, *v)
// 		case u32:
// 			order.put_uint32(bs, v)
// 		case []u32:
// 			for i, x := range v {
// 				order.put_uint32(bs[4*i:], x)
// 			}
// 		case *int64:
// 			order.put_uint64(bs, u64(*v))
// 		case int64:
// 			order.put_uint64(bs, u64(v))
// 		case []int64:
// 			for i, x := range v {
// 				order.put_uint64(bs[8*i:], u64(x))
// 			}
// 		case *u64:
// 			order.put_uint64(bs, *v)
// 		case u64:
// 			order.put_uint64(bs, v)
// 		case []u64:
// 			for i, x := range v {
// 				order.put_uint64(bs[8*i:], x)
// 			}
// 		case *float32:
// 			order.put_uint32(bs, math.Float32bits(*v))
// 		case float32:
// 			order.put_uint32(bs, math.Float32bits(v))
// 		case []float32:
// 			for i, x := range v {
// 				order.put_uint32(bs[4*i:], math.Float32bits(x))
// 			}
// 		case *float64:
// 			order.put_uint64(bs, math.Float64bits(*v))
// 		case float64:
// 			order.put_uint64(bs, math.Float64bits(v))
// 		case []float64:
// 			for i, x := range v {
// 				order.put_uint64(bs[8*i:], math.Float64bits(x))
// 			}
// 		}
// 		_, err := w.write(bs)
// 		return err
// 	}

// 	// Fallback to reflect-based encoding.
// 	v := reflect.Indirect(reflect.ValueOf(data))
// 	size := dataSize(v)
// 	if size < 0 {
// 		return errors.New("binary.Write: invalid type " + reflect.TypeOf(data).String())
// 	}
// 	buf := make([]u8, size)
// 	e := &encoder{order: order, buf: buf}
// 	e.value(v)
// 	_, err := w.write(buf)
// 	return err
// }

// // Size returns how many bytes Write would generate to encode the value v, which
// // must be a fixed-size value or a slice of fixed-size values, or a pointer to such data.
// // If v is neither of these, Size returns -1.
// fn Size(v any) int {
// 	return dataSize(reflect.Indirect(reflect.ValueOf(v)))
// }

// var structSize sync.Map // map[reflect.Type]int

// // dataSize returns the number of bytes the actual data represented by v occupies in memory.
// // For compound structures, it sums the sizes of the elements. Thus, for instance, for a slice
// // it returns the length of the slice times the element size and does not count the memory
// // occupied by the header. If the type of v is not acceptable, dataSize returns -1.
// fn dataSize(v reflect.Value) int {
// 	switch v.Kind() {
// 	case reflect.Slice:
// 		if s := sizeof(v.Type().Elem()); s >= 0 {
// 			return s * v.Len()
// 		}
// 		return -1

// 	case reflect.Struct:
// 		t := v.Type()
// 		if size, ok := structSize.Load(t); ok {
// 			return size.(int)
// 		}
// 		size := sizeof(t)
// 		structSize.Store(t, size)
// 		return size

// 	default:
// 		return sizeof(v.Type())
// 	}
// }

// // sizeof returns the size >= 0 of variables for the given type or -1 if the type is not acceptable.
// fn sizeof(t reflect.Type) int {
// 	switch t.Kind() {
// 	case reflect.Array:
// 		if s := sizeof(t.Elem()); s >= 0 {
// 			return s * t.Len()
// 		}

// 	case reflect.Struct:
// 		sum := 0
// 		for i, n := 0, t.NumField(); i < n; i += 1 {
// 			s := sizeof(t.Field(i).Type)
// 			if s < 0 {
// 				return -1
// 			}
// 			sum += s
// 		}
// 		return sum

// 	case reflect.Bool,
// 		reflect.Uint8, reflect.Uint16, reflect.Uint32, reflect.Uint64,
// 		reflect.Int8, reflect.Int16, reflect.Int32, reflect.Int64,
// 		reflect.Float32, reflect.Float64, reflect.Complex64, reflect.Complex128:
// 		return int(t.Size())
// 	}

// 	return -1
// }

// type coder struct {
// 	order  ByteOrder
// 	buf    []u8
// 	offset int
// }

// type decoder coder
// type encoder coder

// fn (d *decoder) bool() bool {
// 	x := d.buf[d.offset]
// 	d.offset++
// 	return x != 0
// }

// fn (e *encoder) bool(x bool) {
// 	if x {
// 		e.buf[e.offset] = 1
// 	} else {
// 		e.buf[e.offset] = 0
// 	}
// 	e.offset++
// }

// fn (d *decoder) uint8() uint8 {
// 	x := d.buf[d.offset]
// 	d.offset++
// 	return x
// }

// fn (e *encoder) uint8(x uint8) {
// 	e.buf[e.offset] = x
// 	e.offset++
// }

// fn (d *decoder) u16() u16 {
// 	x := d.order.Uint16(d.buf[d.offset : d.offset+2])
// 	d.offset += 2
// 	return x
// }

// fn (e *encoder) u16(x u16) {
// 	e.order.PutUint16(e.buf[e.offset:e.offset+2], x)
// 	e.offset += 2
// }

// fn (d *decoder) u32() -> u32 {
// 	x := d.order.Uint32(d.buf[d.offset : d.offset+4])
// 	d.offset += 4
// 	return x
// }

// fn (e *encoder) u32(x u32) {
// 	e.order.put_uint32(e.buf[e.offset:e.offset+4], x)
// 	e.offset += 4
// }

// fn (d *decoder) u64() -> u64 {
// 	x := d.order.Uint64(d.buf[d.offset : d.offset+8])
// 	d.offset += 8
// 	return x
// }

// fn (e *encoder) u64(x u64) {
// 	e.order.put_uint64(e.buf[e.offset:e.offset+8], x)
// 	e.offset += 8
// }

// fn (d *decoder) int8() int8 { return int8(d.uint8()) }

// fn (e *encoder) int8(x int8) { e.uint8(uint8(x)) }

// fn (d *decoder) int16() int16 { return int16(d.u16()) }

// fn (e *encoder) int16(x int16) { e.u16(u16(x)) }

// fn (d *decoder) int32() int32 { return int32(d.u32()) }

// fn (e *encoder) int32(x int32) { e.u32(u32(x)) }

// fn (d *decoder) int64() int64 { return int64(d.u64()) }

// fn (e *encoder) int64(x int64) { e.u64(u64(x)) }

// fn (d *decoder) value(v reflect.Value) {
// 	switch v.Kind() {
// 	case reflect.Array:
// 		l := v.Len()
// 		for i := 0; i < l; i += 1 {
// 			d.value(v.Index(i))
// 		}

// 	case reflect.Struct:
// 		t := v.Type()
// 		l := v.NumField()
// 		for i := 0; i < l; i += 1 {
// 			// Note: Calling v.CanSet() below is an optimization.
// 			// It would be sufficient to check the field name,
// 			// but creating the StructField info for each field is
// 			// costly (run "go test -bench=ReadStruct" and compare
// 			// results when making changes to this code).
// 			if v := v.Field(i); v.CanSet() || t.Field(i).Name != "_" {
// 				d.value(v)
// 			} else {
// 				d.skip(v)
// 			}
// 		}

// 	case reflect.Slice:
// 		l := v.Len()
// 		for i := 0; i < l; i += 1 {
// 			d.value(v.Index(i))
// 		}

// 	case reflect.Bool:
// 		v.SetBool(d.bool())

// 	case reflect.Int8:
// 		v.SetInt(int64(d.int8()))
// 	case reflect.Int16:
// 		v.SetInt(int64(d.int16()))
// 	case reflect.Int32:
// 		v.SetInt(int64(d.int32()))
// 	case reflect.Int64:
// 		v.SetInt(d.int64())

// 	case reflect.Uint8:
// 		v.SetUint(u64(d.uint8()))
// 	case reflect.Uint16:
// 		v.SetUint(u64(d.u16()))
// 	case reflect.Uint32:
// 		v.SetUint(u64(d.u32()))
// 	case reflect.Uint64:
// 		v.SetUint(d.u64())

// 	case reflect.Float32:
// 		v.SetFloat(float64(math.Float32frombits(d.u32())))
// 	case reflect.Float64:
// 		v.SetFloat(math.Float64frombits(d.u64()))

// 	case reflect.Complex64:
// 		v.SetComplex(complex(
// 			float64(math.Float32frombits(d.u32())),
// 			float64(math.Float32frombits(d.u32())),
// 		))
// 	case reflect.Complex128:
// 		v.SetComplex(complex(
// 			math.Float64frombits(d.u64()),
// 			math.Float64frombits(d.u64()),
// 		))
// 	}
// }

// fn (e *encoder) value(v reflect.Value) {
// 	switch v.Kind() {
// 	case reflect.Array:
// 		l := v.Len()
// 		for i := 0; i < l; i += 1 {
// 			e.value(v.Index(i))
// 		}

// 	case reflect.Struct:
// 		t := v.Type()
// 		l := v.NumField()
// 		for i := 0; i < l; i += 1 {
// 			// see comment for corresponding code in decoder.value()
// 			if v := v.Field(i); v.CanSet() || t.Field(i).Name != "_" {
// 				e.value(v)
// 			} else {
// 				e.skip(v)
// 			}
// 		}

// 	case reflect.Slice:
// 		l := v.Len()
// 		for i := 0; i < l; i += 1 {
// 			e.value(v.Index(i))
// 		}

// 	case reflect.Bool:
// 		e.bool(v.Bool())

// 	case reflect.Int, reflect.Int8, reflect.Int16, reflect.Int32, reflect.Int64:
// 		switch v.Type().Kind() {
// 		case reflect.Int8:
// 			e.int8(int8(v.Int()))
// 		case reflect.Int16:
// 			e.int16(int16(v.Int()))
// 		case reflect.Int32:
// 			e.int32(int32(v.Int()))
// 		case reflect.Int64:
// 			e.int64(v.Int())
// 		}

// 	case reflect.Uint, reflect.Uint8, reflect.Uint16, reflect.Uint32, reflect.Uint64, reflect.Uintptr:
// 		switch v.Type().Kind() {
// 		case reflect.Uint8:
// 			e.uint8(uint8(v.Uint()))
// 		case reflect.Uint16:
// 			e.u16(u16(v.Uint()))
// 		case reflect.Uint32:
// 			e.u32(u32(v.Uint()))
// 		case reflect.Uint64:
// 			e.u64(v.Uint())
// 		}

// 	case reflect.Float32, reflect.Float64:
// 		switch v.Type().Kind() {
// 		case reflect.Float32:
// 			e.u32(math.Float32bits(float32(v.Float())))
// 		case reflect.Float64:
// 			e.u64(math.Float64bits(v.Float()))
// 		}

// 	case reflect.Complex64, reflect.Complex128:
// 		switch v.Type().Kind() {
// 		case reflect.Complex64:
// 			x := v.Complex()
// 			e.u32(math.Float32bits(float32(real(x))))
// 			e.u32(math.Float32bits(float32(imag(x))))
// 		case reflect.Complex128:
// 			x := v.Complex()
// 			e.u64(math.Float64bits(real(x)))
// 			e.u64(math.Float64bits(imag(x)))
// 		}
// 	}
// }

// fn (d *decoder) skip(v reflect.Value) {
// 	d.offset += dataSize(v)
// }

// fn (e *encoder) skip(v reflect.Value) {
// 	n := dataSize(v)
// 	zero := e.buf[e.offset : e.offset+n]
// 	for i := range zero {
// 		zero[i] = 0
// 	}
// 	e.offset += n
// }

// // intDataSize returns the size of the data required to represent the data when encoded.
// // It returns zero if the type cannot be implemented by the fast path in Read or Write.
// fn intDataSize(data any) int {
// 	switch data := data.(type) {
// 	case bool, int8, uint8, *bool, *int8, *uint8:
// 		return 1
// 	case []bool:
// 		return len(data)
// 	case []int8:
// 		return len(data)
// 	case []uint8:
// 		return len(data)
// 	case int16, u16, *int16, *u16:
// 		return 2
// 	case []int16:
// 		return 2 * len(data)
// 	case []u16:
// 		return 2 * len(data)
// 	case int32, u32, *int32, *u32:
// 		return 4
// 	case []int32:
// 		return 4 * len(data)
// 	case []u32:
// 		return 4 * len(data)
// 	case int64, u64, *int64, *u64:
// 		return 8
// 	case []int64:
// 		return 8 * len(data)
// 	case []u64:
// 		return 8 * len(data)
// 	case float32, *float32:
// 		return 4
// 	case float64, *float64:
// 		return 8
// 	case []float32:
// 		return 4 * len(data)
// 	case []float64:
// 		return 8 * len(data)
// 	}
// 	return 0
// }

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_bigendian_put_u32() {
        let mut buf = [0; 4];
        BIG_ENDIAN.put_uint32(&mut buf, 0x01020304);
        assert_eq!([0x01, 0x02, 0x03, 0x04], buf);
    }

    #[test]
    fn test_bigendian_put_u64() {
        let mut buf = [0; 8];
        BIG_ENDIAN.put_uint64(&mut buf, 0x0102030405060708);
        assert_eq!([0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08], buf);
    }
}
