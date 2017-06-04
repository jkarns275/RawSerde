use serialize::RawSerialize;
use deserialize::RawDeserialize;
use std::io::{ Read, Write };
use std::io::Error;

impl<T> RawSerialize for Option<T> where T: RawSerialize {
    #[inline(always)]
    fn raw_serialize(&self, to: &mut Write) -> Result<u64, Error> {
        let mut size = 0; // 1 for the byte representing the enum value
        match self {
            &Some(ref x) => {
                check!(1u8.raw_serialize(to));
                check!(x.raw_serialize(to), size);
            },
            &None => {
                check!(0u8.raw_serialize(to));
            }
        };
        Ok(size + 1)
    }
}
impl<T> RawDeserialize for Option<T> where T: RawDeserialize {
    #[inline(always)]
    fn raw_deserialize(from: &mut Read) -> Result<Self, Error> {
        let t;
        check!(u8::raw_deserialize(from), t);
        match t {
            0 => {
                Ok(None)
            },
            1 => {
                let value;
                check!(T::raw_deserialize(from), value);
                Ok(Some(value))
            },
            _ => {
                panic!("Error parsing Option<T>: possibly corrupted data.");
            }
        }
    }
}
