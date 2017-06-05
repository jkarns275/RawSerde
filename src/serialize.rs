use std::io::{ Error, Write };

/// The serialize trait. Any struct that implements this can be serialized into
/// bytes, and be stuck into anything that implements the Write trait. The
/// location that is written to is important, if you try to read or write from the wrong place,
/// you will get incorrect data, overwrite your data, or may encounter a crash! You must carefully
/// manage your on-disk memory.
pub trait RawSerialize where Self: Sized {
    fn raw_serialize(&self, to: &mut Write) -> Result<u64, Error>;
    fn raw_serialize_slice(x: &[Self], to: &mut Write) -> Result<u64, Error> {
        let mut size = 8;
        check!((x.len() as u64).raw_serialize(to));
        for i in x.iter() {
            let p;
            check!(i.raw_serialize(to), p);
            size += p;
        }
        Ok(size)
    }
}
