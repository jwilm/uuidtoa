// Copyright 2018 Joe Wilm
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg_attr(all(test, feature="bench"), feature(test))]

extern crate uuid;

use std::mem;
use std::io;

use uuid::Uuid;

#[inline]
fn top4(byte: u8) -> u8 {
    byte >> 4
}

#[inline]
fn bot4(byte: u8) -> u8 {
    byte & 0xf
}

fn to_hex_lower(byte: u8) -> u8 {
    debug_assert!(bot4(byte) == byte);

    match byte {
        0x0 => '0' as u8,
        0x1 => '1' as u8,
        0x2 => '2' as u8,
        0x3 => '3' as u8,
        0x4 => '4' as u8,
        0x5 => '5' as u8,
        0x6 => '6' as u8,
        0x7 => '7' as u8,
        0x8 => '8' as u8,
        0x9 => '9' as u8,
        0xa => 'a' as u8,
        0xb => 'b' as u8,
        0xc => 'c' as u8,
        0xd => 'd' as u8,
        0xe => 'e' as u8,
        0xf => 'f' as u8,
        _ => unreachable!(),
    }
}

fn to_hex_upper(byte: u8) -> u8 {
    debug_assert!(bot4(byte) == byte);

    match byte {
        0x0 => '0' as u8,
        0x1 => '1' as u8,
        0x2 => '2' as u8,
        0x3 => '3' as u8,
        0x4 => '4' as u8,
        0x5 => '5' as u8,
        0x6 => '6' as u8,
        0x7 => '7' as u8,
        0x8 => '8' as u8,
        0x9 => '9' as u8,
        0xa => 'A' as u8,
        0xb => 'B' as u8,
        0xc => 'C' as u8,
        0xd => 'D' as u8,
        0xe => 'E' as u8,
        0xf => 'F' as u8,
        _ => unreachable!(),
    }
}

#[inline]
pub fn write_lower<W: io::Write>(io: &mut W, uuid: &Uuid) -> io::Result<usize> {
    write_hex(io, uuid, to_hex_lower)
}

#[inline]
pub fn write_upper<W: io::Write>(io: &mut W, uuid: &Uuid) -> io::Result<usize> {
    write_hex(io, uuid, to_hex_upper)
}

fn write_hex<W, F>(io: &mut W, uuid: &Uuid, to_hex: F) -> io::Result<usize>
    where
        W: io::Write,
        F: Fn(u8) -> u8
{
    let bytes = uuid.as_bytes();
    let mut buf: [u8; 36] = unsafe { mem::uninitialized() };

    buf[0]  = to_hex(top4(bytes[0]));
    buf[1]  = to_hex(bot4(bytes[0]));
    buf[2]  = to_hex(top4(bytes[1]));
    buf[3]  = to_hex(bot4(bytes[1]));
    buf[4]  = to_hex(top4(bytes[2]));
    buf[5]  = to_hex(bot4(bytes[2]));
    buf[6]  = to_hex(top4(bytes[3]));
    buf[7]  = to_hex(bot4(bytes[3]));
    buf[8]  = '-' as u8;
    buf[9]  = to_hex(top4(bytes[4]));
    buf[10] = to_hex(bot4(bytes[4]));
    buf[11] = to_hex(top4(bytes[5]));
    buf[12] = to_hex(bot4(bytes[5]));
    buf[13] = '-' as u8;
    buf[14] = to_hex(top4(bytes[6]));
    buf[15] = to_hex(bot4(bytes[6]));
    buf[16] = to_hex(top4(bytes[7]));
    buf[17] = to_hex(bot4(bytes[7]));
    buf[18] = '-' as u8;
    buf[19] = to_hex(top4(bytes[8]));
    buf[20] = to_hex(bot4(bytes[8]));
    buf[21] = to_hex(top4(bytes[9]));
    buf[22] = to_hex(bot4(bytes[9]));
    buf[23] = '-' as u8;
    buf[24] = to_hex(top4(bytes[10]));
    buf[25] = to_hex(bot4(bytes[10]));
    buf[26] = to_hex(top4(bytes[11]));
    buf[27] = to_hex(bot4(bytes[11]));
    buf[28] = to_hex(top4(bytes[12]));
    buf[29] = to_hex(bot4(bytes[12]));
    buf[30] = to_hex(top4(bytes[13]));
    buf[31] = to_hex(bot4(bytes[13]));
    buf[32] = to_hex(top4(bytes[14]));
    buf[33] = to_hex(bot4(bytes[14]));
    buf[34] = to_hex(top4(bytes[15]));
    buf[35] = to_hex(bot4(bytes[15]));

    io.write_all(&buf[..])?;
    Ok(36)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use uuid::Uuid;
    use super::{write_lower, write_upper};

    macro_rules! assert_lower {
        ($input:expr) => {
            let uuid = Uuid::from_str($input).expect("input valid uuid");
            let mut buf: Vec<u8> = Vec::with_capacity(36);

            write_lower(&mut buf, &uuid).unwrap();
            assert_eq!(::std::str::from_utf8(&buf).unwrap(), $input);
        }
    }

    macro_rules! assert_upper {
        ($input:expr) => {
            let uuid = Uuid::from_str($input).expect("input valid uuid");
            let mut buf: Vec<u8> = Vec::with_capacity(36);

            write_upper(&mut buf, &uuid).unwrap();
            assert_eq!(::std::str::from_utf8(&buf).unwrap(), $input);
        }
    }

    #[test]
    fn to_lower_works() {
        assert_lower!("53ff95bd-7e61-4946-bcce-a43b5c91b8dc");
        assert_lower!("e098d1f6-7e49-4c34-99f6-e9c6cef8fcce");
        assert_lower!("00000000-0000-0000-0000-000000000000");
        assert_lower!("ffffffff-ffff-ffff-ffff-ffffffffffff");
        assert_lower!("11111111-1111-1111-1111-111111111111");
    }

    #[test]
    fn to_upper_works() {
        assert_upper!("53FF95BD-7E61-4946-BCCE-A43B5C91B8DC");
        assert_upper!("E098D1F6-7E49-4C34-99F6-E9C6CEF8FCCE");
        assert_upper!("00000000-0000-0000-0000-000000000000");
        assert_upper!("FFFFFFFF-FFFF-FFFF-FFFF-FFFFFFFFFFFF");
        assert_upper!("11111111-1111-1111-1111-111111111111");
    }
}

#[cfg(all(feature="bench", test))]
mod benches {
    extern crate test;
    use self::test::{Bencher, black_box};

    use std::str::FromStr;
    use uuid::Uuid;
    use super::{write_lower};

    #[bench]
    fn bench_lower(b: &mut Bencher) {
        let uuid = Uuid::from_str("e098d1f6-7e49-4c34-99f6-e9c6cef8fcce").unwrap();
        b.iter(|| {
            let mut buf: Vec<u8> = Vec::with_capacity(36);
            write_lower(&mut buf, &uuid).unwrap();
            black_box(buf);
        });
    }

    #[bench]
    fn bench_fmt(b: &mut Bencher) {
        let uuid = Uuid::from_str("e098d1f6-7e49-4c34-99f6-e9c6cef8fcce").unwrap();
        b.iter(|| {
            let buf = format!("{}", uuid.hyphenated());
            black_box(buf);
        });
    }
}
