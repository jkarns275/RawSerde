#![feature(i128_type)]

#[macro_use]
extern crate raw_serde_derive;

#[macro_use]
pub mod check;
pub mod collections;
pub mod serialize;
pub mod deserialize;
pub mod primitives;
pub mod array;
pub mod option;
pub mod tuple;

pub use tuple::*;
pub use raw_serde_derive::*;
pub use option::*;
pub use serialize::RawSerialize;
pub use deserialize::RawDeserialize;
pub use collections::*;
pub use primitives::*;
pub use array::*;



#[cfg(test)]
mod tests {

    use std::fs::{ File, OpenOptions };
    use std::io::{ Read, Write, Seek, SeekFrom };
    use super::*;

    use std::collections::*;

    #[derive(RawSerialize, RawDeserialize, Debug, PartialEq, Eq)]
    struct test_struct {
	      x: i32,
	      y: i32,
          t: [i32; 5],
        z: i128
    }

    #[derive(RawSerialize, RawDeserialize, Debug, PartialEq, Eq)]
    struct test_struct2(pub i32, pub i32, pub i64);

    #[test]
    fn test_vec() {
        let mut v = vec![];
        for _ in 0..10 {
            let mut x = HashMap::<String, i32>::new();
            for i in 0..100 {
                x.insert(i.to_string(), i);
            }
            v.push(x);
        }
        let mut file = OpenOptions::new().read(true).write(true).create(true).open("test_vec.dat").unwrap();
        v.raw_serialize(&mut file).unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();
        let x = Vec::<HashMap<String, i32>>::raw_deserialize(&mut file).unwrap();
        assert!(x == v)
    }

    #[test]
    fn test_vec_simple() {
        let mut lfsr: i32 = 0xACE1;
        let mut bit: i32 = 0;
        let mut rand = || {
            bit  = ((lfsr >> 0) ^ (lfsr >> 2) ^ (lfsr >> 3) ^ (lfsr >> 5) ) & 1;
            lfsr = (lfsr >> 1) | (bit << 15);
            lfsr as i32
        };

        let mut file = OpenOptions::new().read(true).write(true).create(true).open("test_vec_simple.dat").unwrap();
        let mut test = vec![];
        for _ in 0..10 {
            let z = test_struct2(rand(), rand(), rand() as i64 * 1000);
            println!("{:?}", z);
            test.push(z);
        }
        test.raw_serialize(&mut file).unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();
        let x = test_struct2::raw_deserialize_vec(&mut file).unwrap();
        assert!(test == x);
        println!("{:?}", x);
    }
}
