use std::io::{ Error, Read };

pub trait RawDeserialize where Self: Sized {
    fn raw_deserialize(from: &mut Read) -> Result<Self, Error>;
    fn raw_deserialize_vec(from: &mut Read) -> Result<Vec<Self>, Error> {
        let len;
        check!(u64::raw_deserialize(from), len);
        let mut v = Vec::with_capacity(len as usize);
        for _ in 0..len {
            let x;
            check!(Self::raw_deserialize(from), x);
            v.push(x);
        }
        Ok(v)
    }
}
