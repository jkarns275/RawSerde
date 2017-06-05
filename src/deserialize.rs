use std::io::{ Error, Read };

/// The deserialize trait. Any struct that implements this can be deserialized from
/// bytes, reading from anything that implements the Read trait. The
/// location that is read from is important, if you try to read from the wrong place,
/// you will get incorrect data or may encounter a crash! You must carefully
/// manage your on-disk memory.
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
