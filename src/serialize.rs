use std::io::{ Error, Write };

pub trait RawSerialize where Self: Sized {
    fn raw_serialize(&self, to: &mut Write) -> Result<(), Error>;
    fn raw_serialize_slice(x: &[Self], to: &mut Write) -> Result<(), Error> {
        check!(x.len().raw_serialize(to));
        for i in x.iter() {
            check!(i.raw_serialize(to));
        }
        Ok(())
    }
}
