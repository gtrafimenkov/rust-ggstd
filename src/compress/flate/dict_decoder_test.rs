// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2016 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use super::dict_decoder::DictDecoder;
use crate::bytes;
use std::io::Write;

#[test]
fn test_dict_decoder() {
    let abc = "ABC\n";
    let fox = "The quick brown fox jumped over the lazy dog!\n";
    let poem = r#"The Road Not Taken
Robert Frost

Two roads diverged in a yellow wood,
And sorry I could not travel both
And be one traveler, long I stood
And looked down one as far as I could
To where it bent in the undergrowth;

Then took the other, as just as fair,
And having perhaps the better claim,
Because it was grassy and wanted wear;
Though as for that the passing there
Had worn them really about the same,

And both that morning equally lay
In leaves no step had trodden black.
Oh, I kept the first for another day!
Yet knowing how way leads on to way,
I doubted if I should ever come back.

I shall be telling this with a sigh
Somewhere ages and ages hence:
Two roads diverged in a wood, and I-
I took the one less traveled by,
And that has made all the difference.
"#;

    struct Ref {
        dist: usize,
        length: usize,
    }

    impl Ref {
        const fn new(dist: usize, length: usize) -> Self {
            Self { dist, length }
        }
    }

    let poem_refs: &[Ref] = &[
        Ref::new(0, 38),
        Ref::new(33, 3),
        Ref::new(0, 48),
        Ref::new(79, 3),
        Ref::new(0, 11),
        Ref::new(34, 5),
        Ref::new(0, 6),
        Ref::new(23, 7),
        Ref::new(0, 8),
        Ref::new(50, 3),
        Ref::new(0, 2),
        Ref::new(69, 3),
        Ref::new(34, 5),
        Ref::new(0, 4),
        Ref::new(97, 3),
        Ref::new(0, 4),
        Ref::new(43, 5),
        Ref::new(0, 6),
        Ref::new(7, 4),
        Ref::new(88, 7),
        Ref::new(0, 12),
        Ref::new(80, 3),
        Ref::new(0, 2),
        Ref::new(141, 4),
        Ref::new(0, 1),
        Ref::new(196, 3),
        Ref::new(0, 3),
        Ref::new(157, 3),
        Ref::new(0, 6),
        Ref::new(181, 3),
        Ref::new(0, 2),
        Ref::new(23, 3),
        Ref::new(77, 3),
        Ref::new(28, 5),
        Ref::new(128, 3),
        Ref::new(110, 4),
        Ref::new(70, 3),
        Ref::new(0, 4),
        Ref::new(85, 6),
        Ref::new(0, 2),
        Ref::new(182, 6),
        Ref::new(0, 4),
        Ref::new(133, 3),
        Ref::new(0, 7),
        Ref::new(47, 5),
        Ref::new(0, 20),
        Ref::new(112, 5),
        Ref::new(0, 1),
        Ref::new(58, 3),
        Ref::new(0, 8),
        Ref::new(59, 3),
        Ref::new(0, 4),
        Ref::new(173, 3),
        Ref::new(0, 5),
        Ref::new(114, 3),
        Ref::new(0, 4),
        Ref::new(92, 5),
        Ref::new(0, 2),
        Ref::new(71, 3),
        Ref::new(0, 2),
        Ref::new(76, 5),
        Ref::new(0, 1),
        Ref::new(46, 3),
        Ref::new(96, 4),
        Ref::new(130, 4),
        Ref::new(0, 3),
        Ref::new(360, 3),
        Ref::new(0, 3),
        Ref::new(178, 5),
        Ref::new(0, 7),
        Ref::new(75, 3),
        Ref::new(0, 3),
        Ref::new(45, 6),
        Ref::new(0, 6),
        Ref::new(299, 6),
        Ref::new(180, 3),
        Ref::new(70, 6),
        Ref::new(0, 1),
        Ref::new(48, 3),
        Ref::new(66, 4),
        Ref::new(0, 3),
        Ref::new(47, 5),
        Ref::new(0, 9),
        Ref::new(325, 3),
        Ref::new(0, 1),
        Ref::new(359, 3),
        Ref::new(318, 3),
        Ref::new(0, 2),
        Ref::new(199, 3),
        Ref::new(0, 1),
        Ref::new(344, 3),
        Ref::new(0, 3),
        Ref::new(248, 3),
        Ref::new(0, 10),
        Ref::new(310, 3),
        Ref::new(0, 3),
        Ref::new(93, 6),
        Ref::new(0, 3),
        Ref::new(252, 3),
        Ref::new(157, 4),
        Ref::new(0, 2),
        Ref::new(273, 5),
        Ref::new(0, 14),
        Ref::new(99, 4),
        Ref::new(0, 1),
        Ref::new(464, 4),
        Ref::new(0, 2),
        Ref::new(92, 4),
        Ref::new(495, 3),
        Ref::new(0, 1),
        Ref::new(322, 4),
        Ref::new(16, 4),
        Ref::new(0, 3),
        Ref::new(402, 3),
        Ref::new(0, 2),
        Ref::new(237, 4),
        Ref::new(0, 2),
        Ref::new(432, 4),
        Ref::new(0, 1),
        Ref::new(483, 5),
        Ref::new(0, 2),
        Ref::new(294, 4),
        Ref::new(0, 2),
        Ref::new(306, 3),
        Ref::new(113, 5),
        Ref::new(0, 1),
        Ref::new(26, 4),
        Ref::new(164, 3),
        Ref::new(488, 4),
        Ref::new(0, 1),
        Ref::new(542, 3),
        Ref::new(248, 6),
        Ref::new(0, 5),
        Ref::new(205, 3),
        Ref::new(0, 8),
        Ref::new(48, 3),
        Ref::new(449, 6),
        Ref::new(0, 2),
        Ref::new(192, 3),
        Ref::new(328, 4),
        Ref::new(9, 5),
        Ref::new(433, 3),
        Ref::new(0, 3),
        Ref::new(622, 25),
        Ref::new(615, 5),
        Ref::new(46, 5),
        Ref::new(0, 2),
        Ref::new(104, 3),
        Ref::new(475, 10),
        Ref::new(549, 3),
        Ref::new(0, 4),
        Ref::new(597, 8),
        Ref::new(314, 3),
        Ref::new(0, 1),
        Ref::new(473, 6),
        Ref::new(317, 5),
        Ref::new(0, 1),
        Ref::new(400, 3),
        Ref::new(0, 3),
        Ref::new(109, 3),
        Ref::new(151, 3),
        Ref::new(48, 4),
        Ref::new(0, 4),
        Ref::new(125, 3),
        Ref::new(108, 3),
        Ref::new(0, 2),
    ];

    let mut got = bytes::Buffer::new();
    let mut want = bytes::Buffer::new();

    let mut dd = DictDecoder::new(1 << 11, &[]);

    // write data to dd by copying existing dd content, when dd is full, flush data to buf
    fn write_copy(buf: &mut bytes::Buffer, dd: &mut DictDecoder, dist: usize, length: usize) {
        let mut length = length;
        while length > 0 {
            let mut cnt = dd.try_write_copy(dist, length);
            if cnt == 0 {
                cnt = dd.write_copy(dist, length);
            }

            length -= cnt;
            if dd.avail_write() == 0 {
                buf.write(dd.read_flush()).unwrap();
            }
        }
    }

    // write string to dd as is, when dd is full, flush data to buf
    fn write_string(buf: &mut bytes::Buffer, dd: &mut DictDecoder, str: &str) {
        let mut str = str.as_bytes();
        while !str.is_empty() {
            let write_buf = dd.write_slice(str.len());
            let cnt = write_buf.len();
            write_buf[0..cnt].copy_from_slice(&str[0..cnt]);
            str = &str[cnt..];
            dd.write_mark(cnt);
            if dd.avail_write() == 0 {
                buf.write(dd.read_flush()).unwrap();
            }
        }
    }

    write_string(&mut got, &mut dd, ".");
    want.write_byte(b'.').unwrap();

    let mut str = poem.as_bytes();
    for r in poem_refs {
        if r.dist == 0 {
            write_string(
                &mut got,
                &mut dd,
                &String::from_utf8(str[..r.length].to_vec()).unwrap(),
            );
        } else {
            write_copy(&mut got, &mut dd, r.dist, r.length);
        }
        str = &str[r.length..];
    }
    want.write_string(poem).unwrap();

    {
        let size = dd.hist_size();
        write_copy(&mut got, &mut dd, size, 33);
        let first_33 = want.bytes()[..33].to_vec();
        want.write(&first_33).unwrap();
    }

    write_string(&mut got, &mut dd, abc);
    write_copy(&mut got, &mut dd, abc.len(), 59 * abc.len());
    want.write_string(&abc.repeat(60)).unwrap();

    write_string(&mut got, &mut dd, fox);
    write_copy(&mut got, &mut dd, fox.len(), 9 * fox.len());
    want.write_string(&fox.repeat(10)).unwrap();

    write_string(&mut got, &mut dd, ".");
    write_copy(&mut got, &mut dd, 1, 9);
    want.write_string(&".".repeat(10)).unwrap();

    assert_eq!(poem.to_uppercase().len(), poem.len());

    write_string(&mut got, &mut dd, &poem.to_uppercase());
    write_copy(&mut got, &mut dd, poem.len(), 7 * poem.len());
    want.write_string(&poem.to_uppercase().repeat(8)).unwrap();

    {
        let size = dd.hist_size();
        write_copy(&mut got, &mut dd, size, 10);
        let len = want.bytes().len();
        let first_10 = want.bytes()[len - size..][..10].to_vec();
        want.write(&first_10).unwrap();
    }

    got.write(dd.read_flush()).unwrap();

    assert_eq!(want.string().len(), got.string().len());
    assert_eq!(want.string(), got.string());
}
