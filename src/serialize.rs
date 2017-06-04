use std::io::{ Error, Write };

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
