#![feature(custom_derive)]
#![feature(test)]

extern crate rand;
extern crate test;

pub mod bitarea {

    use std::ops::{Shl,Shr};
    use std::fmt;

    const WIDTH: u32 = 3;
    const HEIGHT: u32 = 4;
    const USED_BITS: u32 = WIDTH*HEIGHT;
    const UNUSED_BITS: u32 = 64 - USED_BITS;

    #[derive(Copy)]
    #[derive_Rand]
    pub struct Bitarea {
        pub data: u64,
    }

    impl Bitarea {

        pub fn new() -> Bitarea {
            Bitarea {data: 0}
        }

        pub fn set(&mut self, col: u32, row:u32, val: bool) {
            assert!(row < HEIGHT && col < WIDTH);
            let mask = 1 << (63-(row*WIDTH+col));
            self.data = if val {
                self.data | mask
            } else {
                self.data & !mask
            }
        }

        pub fn get(self, col: u32, row:u32) -> bool {
            assert!(row < HEIGHT && col < WIDTH);
            let mask = 1 << (63-(row*WIDTH+col));
            self.data & mask != 0
        }

        pub fn from_parts(parts: &[u64]) -> Bitarea {
            assert!(parts.len() == HEIGHT as usize);
            let mut d = 0;
            for v in parts.iter() {
                d <<= WIDTH;
                d |= *v;
            }
            d <<= UNUSED_BITS;
            Bitarea {data: d}
        }
    }

    impl Shl<u32> for Bitarea {

        type Output = Bitarea;

        fn shl(self, rhl: u32) -> Bitarea {
            if rhl >= WIDTH {
                return Bitarea {data: 0}
            }
            let mut mask = 0;
            for _ in 0..HEIGHT {
                mask <<= WIDTH;
                mask |= (1 << rhl) - 1;
            }
            mask = (!mask) << UNUSED_BITS;
            Bitarea {data: (self.data << rhl) & mask}
        }
    }

    impl Shr<u32> for Bitarea {

        type Output = Bitarea;

        fn shr(self, rhl: u32) -> Bitarea {
            if rhl >= WIDTH {
                return Bitarea {data: 0}
            }
            let mut mask = 0;
            for _ in 0..HEIGHT {
                mask <<= WIDTH;
                mask |= (1 << (WIDTH - rhl)) - 1;
            }
            mask = mask << UNUSED_BITS;
            Bitarea {data: (self.data >> rhl) & mask}
        }
    }

    impl fmt::Debug for Bitarea {

        fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
            for r in 0..HEIGHT {
                try!(f.write_str("\n"));
                for c in 0..WIDTH {
                    try!(f.write_str(if self.get(c,r) {"1"} else {"0"}));
                }
            }
            Ok(())
        }
    }

    impl Eq for Bitarea{}
    impl PartialEq for Bitarea {

        fn eq(&self, other: &Bitarea) -> bool {
            return (self.data >> UNUSED_BITS) == (other.data >> UNUSED_BITS);
        }
    }

    #[cfg(test)]
    mod tests {

        use super::*;
        use test::Bencher;
        use test::black_box;

        #[test]
        fn fmt() {
            let x = Bitarea::from_parts(&[0b100,
                                          0b001,
                                          0b110,
                                          0b010]);
            assert_eq!("\n100\n001\n110\n010", format!("{:?}", x));
        }

        #[test]
        fn set() {
            let mut b = Bitarea::new();
            b.set(0,0, true);
            b.set(1,3, true);
            b.set(1,2, true);
            b.set(0,2, true);
            b.set(2,1, true);

            assert_eq!(Bitarea::from_parts(&[0b100,
                                             0b001,
                                             0b110,
                                             0b010]),
                       b)
        }

        #[test]
        fn get() {
            let b = Bitarea::from_parts(&[0b001,
                                          0b111,
                                          0b010,
                                          0b001]);
            assert_eq!(b.get(0,0), false);
            assert_eq!(b.get(0,1), true);
            assert_eq!(b.get(0,2), false);
            assert_eq!(b.get(0,3), false);
            assert_eq!(b.get(1,0), false);
            assert_eq!(b.get(1,1), true);
            assert_eq!(b.get(1,2), true);
            assert_eq!(b.get(1,3), false);
            assert_eq!(b.get(2,0), true);
            assert_eq!(b.get(2,1), true);
            assert_eq!(b.get(2,2), false);
            assert_eq!(b.get(2,3), true);
        }

        #[test]
        fn shl1() {
            let b = Bitarea::from_parts(   &[0b001,
                                             0b111,
                                             0b010,
                                             0b001]);

            assert_eq!(Bitarea::from_parts(&[0b001,
                                             0b111,
                                             0b010,
                                             0b001]),
                       b << 0);

            assert_eq!(Bitarea::from_parts(&[0b010,
                                             0b110,
                                             0b100,
                                             0b010]),
                       b << 1);

            assert_eq!(Bitarea::from_parts(&[0b100,
                                             0b100,
                                             0b000,
                                             0b100]),
                       b << 2);

            assert_eq!(Bitarea::from_parts(&[0b000,
                                             0b000,
                                             0b000,
                                             0b000]),
                       b << 3);

            assert_eq!(Bitarea::from_parts(&[0b000,
                                             0b000,
                                             0b000,
                                             0b000]),
                       b << 4);

        }

        #[test]
        fn shl2() {
            let b = Bitarea::from_parts(   &[0b111,
                                             0b111,
                                             0b111,
                                             0b111]);

            assert_eq!(Bitarea::from_parts(&[0b111,
                                             0b111,
                                             0b111,
                                             0b111]),
                       b << 0);

            assert_eq!(Bitarea::from_parts(&[0b110,
                                             0b110,
                                             0b110,
                                             0b110]),
                       b << 1);

            assert_eq!(Bitarea::from_parts(&[0b100,
                                             0b100,
                                             0b100,
                                             0b100]),
                       b << 2);

            assert_eq!(Bitarea::from_parts(&[0b000,
                                             0b000,
                                             0b000,
                                             0b000]),
                       b << 3);

            assert_eq!(Bitarea::from_parts(&[0b000,
                                             0b000,
                                             0b000,
                                             0b000]),
                       b << 4);

        }

        #[test]
        fn shl3() {
            let b = Bitarea::from_parts(   &[0b101,
                                             0b010,
                                             0b101,
                                             0b010]);

            assert_eq!(Bitarea::from_parts(&[0b101,
                                             0b010,
                                             0b101,
                                             0b010]),
                       b << 0);

            assert_eq!(Bitarea::from_parts(&[0b010,
                                             0b100,
                                             0b010,
                                             0b100]),
                       b << 1);

            assert_eq!(Bitarea::from_parts(&[0b100,
                                             0b000,
                                             0b100,
                                             0b000]),
                       b << 2);

            assert_eq!(Bitarea::from_parts(&[0b000,
                                             0b000,
                                             0b000,
                                             0b000]),
                       b << 3);

            assert_eq!(Bitarea::from_parts(&[0b000,
                                             0b000,
                                             0b000,
                                             0b000]),
                       b << 4);

        }

        #[test]
        fn shl4() {
            let b = Bitarea::from_parts(   &[0b010,
                                             0b101,
                                             0b010,
                                             0b101]);

            assert_eq!(Bitarea::from_parts(&[0b010,
                                             0b101,
                                             0b010,
                                             0b101]),
                       b << 0);

            assert_eq!(Bitarea::from_parts(&[0b100,
                                             0b010,
                                             0b100,
                                             0b010]),
                       b << 1);

            assert_eq!(Bitarea::from_parts(&[0b000,
                                             0b100,
                                             0b000,
                                             0b100]),
                       b << 2);

            assert_eq!(Bitarea::from_parts(&[0b000,
                                             0b000,
                                             0b000,
                                             0b000]),
                       b << 3);

            assert_eq!(Bitarea::from_parts(&[0b000,
                                             0b000,
                                             0b000,
                                             0b000]),
                       b << 4);

        }

        #[test]
        fn shr1() {
            let b = Bitarea::from_parts(   &[0b001,
                                             0b111,
                                             0b010,
                                             0b001]);

            assert_eq!(Bitarea::from_parts(&[0b001,
                                             0b111,
                                             0b010,
                                             0b001]),
                       b >> 0);

            assert_eq!(Bitarea::from_parts(&[0b000,
                                             0b011,
                                             0b001,
                                             0b000]),
                       b >> 1);

            assert_eq!(Bitarea::from_parts(&[0b000,
                                             0b001,
                                             0b000,
                                             0b000]),
                       b >> 2);

            assert_eq!(Bitarea::from_parts(&[0b000,
                                             0b000,
                                             0b000,
                                             0b000]),
                       b >> 3);

            assert_eq!(Bitarea::from_parts(&[0b000,
                                             0b000,
                                             0b000,
                                             0b000]),
                       b >> 4);

        }

        #[test]
        fn shr2() {
            let b = Bitarea::from_parts(   &[0b111,
                                             0b111,
                                             0b111,
                                             0b111]);

            assert_eq!(Bitarea::from_parts(&[0b111,
                                             0b111,
                                             0b111,
                                             0b111]),
                       b >> 0);

            assert_eq!(Bitarea::from_parts(&[0b011,
                                             0b011,
                                             0b011,
                                             0b011]),
                       b >> 1);

            assert_eq!(Bitarea::from_parts(&[0b001,
                                             0b001,
                                             0b001,
                                             0b001]),
                       b >> 2);

            assert_eq!(Bitarea::from_parts(&[0b000,
                                             0b000,
                                             0b000,
                                             0b000]),
                       b >> 3);

            assert_eq!(Bitarea::from_parts(&[0b000,
                                             0b000,
                                             0b000,
                                             0b000]),
                       b >> 4);

        }

        #[test]
        fn shr3() {
            let b = Bitarea::from_parts(   &[0b101,
                                             0b010,
                                             0b101,
                                             0b010]);

            assert_eq!(Bitarea::from_parts(&[0b101,
                                             0b010,
                                             0b101,
                                             0b010]),
                       b >> 0);

            assert_eq!(Bitarea::from_parts(&[0b010,
                                             0b001,
                                             0b010,
                                             0b001]),
                       b >> 1);

            assert_eq!(Bitarea::from_parts(&[0b001,
                                             0b000,
                                             0b001,
                                             0b000]),
                       b >> 2);

            assert_eq!(Bitarea::from_parts(&[0b000,
                                             0b000,
                                             0b000,
                                             0b000]),
                       b >> 3);

            assert_eq!(Bitarea::from_parts(&[0b000,
                                             0b000,
                                             0b000,
                                             0b000]),
                       b >> 4);

        }

        #[test]
        fn shr4() {
            let b = Bitarea::from_parts(   &[0b010,
                                             0b101,
                                             0b010,
                                             0b101]);

            assert_eq!(Bitarea::from_parts(&[0b010,
                                             0b101,
                                             0b010,
                                             0b101]),
                       b >> 0);

            assert_eq!(Bitarea::from_parts(&[0b001,
                                             0b010,
                                             0b001,
                                             0b010]),
                       b >> 1);

            assert_eq!(Bitarea::from_parts(&[0b000,
                                             0b001,
                                             0b000,
                                             0b001]),
                       b >> 2);

            assert_eq!(Bitarea::from_parts(&[0b000,
                                             0b000,
                                             0b000,
                                             0b000]),
                       b >> 3);

            assert_eq!(Bitarea::from_parts(&[0b000,
                                             0b000,
                                             0b000,
                                             0b000]),
                       b >> 4);

        }

        #[bench]
        fn bench_shl(bench: &mut Bencher) {
            let b = Bitarea::from_parts(&[0b001,
                                          0b111,
                                          0b010,
                                          0b001]);
            bench.iter(|| b << black_box(1));
        }

        #[bench]
        fn bench_shr(bench: &mut Bencher) {
            let b = Bitarea::from_parts(&[0b001,
                                          0b111,
                                          0b010,
                                          0b001]);
            bench.iter(|| b >> black_box(1));
        }

    }

}
