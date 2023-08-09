// // Copyright 2022 The Go Authors. All rights reserved.
// // Use of this source code is governed by a BSD-style
// // license that can be found in the LICENSE file.

// package errors_test

// import (
// 	"errors"
// 	"reflect"
// 	"testing"
// )

// func TestJoinReturnsNil(t *testing.T) {
// 	if err := errors.Join(); err != nil {
// 		t.Errorf("errors.Join() = {}, want nil", err)
// 	}
// 	if err := errors.Join(nil); err != nil {
// 		t.Errorf("errors.Join(nil) = {}, want nil", err)
// 	}
// 	if err := errors.Join(nil, nil); err != nil {
// 		t.Errorf("errors.Join(nil, nil) = {}, want nil", err)
// 	}
// }

// func TestJoin(t *testing.T) {
// 	err1 := errors.New("err1")
// 	err2 := errors.New("err2")
// 	for _, test := range []struct {
// 		errs []error
// 		want []error
// 	}{{
// 		errs: []error{err1},
// 		want: []error{err1},
// 	}, {
// 		errs: []error{err1, err2},
// 		want: []error{err1, err2},
// 	}, {
// 		errs: []error{err1, nil, err2},
// 		want: []error{err1, err2},
// 	}} {
// 		got := errors.Join(test.errs...).(interface{ Unwrap() []error }).Unwrap()
// 		if !reflect.DeepEqual(got, test.want) {
// 			t.Errorf("Join({}) = {}; want {}", test.errs, got, test.want)
// 		}
// 		if len(got) != cap(got) {
// 			t.Errorf("Join({}) returns errors with len={}, cap={}; want len==cap", test.errs, len(got), cap(got))
// 		}
// 	}
// }

// func TestJoinErrorMethod(t *testing.T) {
// 	err1 := errors.New("err1")
// 	err2 := errors.New("err2")
// 	for _, test := range []struct {
// 		errs []error
// 		want string
// 	}{{
// 		errs: []error{err1},
// 		want: "err1",
// 	}, {
// 		errs: []error{err1, err2},
// 		want: "err1\nerr2",
// 	}, {
// 		errs: []error{err1, nil, err2},
// 		want: "err1\nerr2",
// 	}} {
// 		got := errors.Join(test.errs...).Error()
// 		if got != test.want {
// 			t.Errorf("Join({}).Error() = %q; want %q", test.errs, got, test.want)
// 		}
// 	}
// }
