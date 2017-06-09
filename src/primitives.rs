use serialize::RawSerialize;
use deserialize::RawDeserialize;
use std::io::{ Read, Write };
use std::slice;
use std::mem;
use std::io::Error;

const SIZE_OF_U128: usize = 16;
const SIZE_OF_I128: usize = 16;
const SIZE_OF_U64: usize = 8;
const SIZE_OF_U32: usize = 4;
const SIZE_OF_U16: usize = 2;
const SIZE_OF_U8:  usize = 1;
const SIZE_OF_I64: usize = 8;
const SIZE_OF_I32: usize = 4;
const SIZE_OF_I16: usize = 2;
const SIZE_OF_I8:  usize = 1;

/// Serializing usize should be avoided if you want your data to be portable.
impl RawSerialize for bool {
    #[inline(always)]
    fn raw_serialize(&self, to: &mut Write) -> Result<u64, Error> {
        if *self {
            1u8.raw_serialize(to)
        } else {
            0u8.raw_serialize(to)
        }
    }
}

/// Serializing usize should be avoided if you want your data to be portable.
impl RawDeserialize for usize {
    #[inline(always)]
    fn raw_deserialize(from: &mut Read) -> Result<Self, Error> {
        let x;
        check!(u8::raw_deserialize(from), x);
        Ok(x == 1)
    }
}

/// Serializing usize should be avoided if you want your data to be portable.
impl RawSerialize for usize {
    #[inline(always)]
    fn raw_serialize(&self, to: &mut Write) -> Result<u64, Error> {
        let x = [*self];
        let y = unsafe { slice::from_raw_parts((&x).as_ptr() as *const u8, 8) };
        if let Err(e) = to.write_all(y) {
            Err(e)
        } else {
            Ok(mem::size_of::<usize>() as u64)
        }
    }
}

/// Serializing usize should be avoided if you want your data to be portable.
impl RawDeserialize for usize {
    #[inline(always)]
    fn raw_deserialize(from: &mut Read) -> Result<Self, Error> {
        let mut buffer = [0u8; 8];
        match from.read_exact(&mut buffer) {
            Ok(_) => Ok(unsafe { mem::transmute::<[u8; 8], u64>(buffer) as usize } ),
            Err(e) => Err(e)
        }
    }
}

/// Generates code to serialize primitives, uses a bit of unsafe code to (hopefully)
/// speed things up.
macro_rules! serialize_primitive {
    ( $prim:ty, $size:expr ) => (
        impl RawSerialize for $prim {
            #[inline(always)]
            fn raw_serialize(&self, to: &mut Write) -> Result<u64, Error> {
                let x = [*self];
                let y = unsafe { slice::from_raw_parts((&x).as_ptr() as *const u8, $size) };
                if let Err(e) = to.write_all(y) {
                    Err(e)
                } else {
                    Ok($size as u64)
                }
            }
            fn raw_serialize_slice(x: &[Self], to: &mut Write) -> Result<u64, Error> {
                let len = x.len() as u64;
                match len.raw_serialize(to) {
                    Err(e) => return Err(e),
                    Ok(_) => ()
                };
                let y = unsafe { slice::from_raw_parts(x.as_ptr() as *const u8, $size * (len as usize)) };
                if let Err(e) = to.write_all(y) {
                    Err(e)
                } else {
                    Ok($size as u64 * len + 8)
                }
            }
        }
        impl RawDeserialize for $prim {
            #[inline(always)]
            fn raw_deserialize(from: &mut Read) -> Result<Self, Error> {
                let mut buffer = [0u8; $size];

                match from.read_exact(&mut buffer) {
                    Ok(_) => Ok( unsafe { mem::transmute::<[u8; $size], $prim>(buffer) } ),
                    Err(e) => Err(e)
                }
            }
            fn raw_deserialize_vec(from: &mut Read) -> Result<Vec<Self>, Error> {
                let size: u64;
                match u64::raw_deserialize(from) {
                    Ok(x) => {
                        size = x;
                    },
                    Err(e) => return Err(e)
                };
                let mut dat = vec![0u8; size as usize * $size];
                check!(from.read_exact(&mut dat));
                unsafe {
                    Ok(Vec::<$prim>::from_raw_parts(Box::into_raw(dat.into_boxed_slice()) as *mut $prim, size as usize, size as usize))
                }
            }
        }

        impl<'b> RawSerialize for &'b [$prim] {
            fn raw_serialize(&self, to: &mut Write) -> Result<u64, Error> {
                let len = self.len() as u64;
                match len.raw_serialize(to) {
                    Err(e) => return Err(e),
                    Ok(_) => {}
                };
                let y = unsafe { slice::from_raw_parts((*self).as_ptr() as *const u8, $size * self.len()) };
                if let Err(e) = to.write_all(&y) {
                    Err(e)
                } else {
                    Ok($size as u64 * len + 8)
                }
            }
        }
    )
}

serialize_primitive!(i8,  SIZE_OF_I8);
serialize_primitive!(u64, SIZE_OF_U64);
serialize_primitive!(u8,  SIZE_OF_U8);


serialize_primitive!(i16, SIZE_OF_I16);
serialize_primitive!(i32, SIZE_OF_I32);
serialize_primitive!(i64, SIZE_OF_I64);
serialize_primitive!(u16, SIZE_OF_U16);
serialize_primitive!(u32, SIZE_OF_U32);

serialize_primitive!(f32, SIZE_OF_U32);
serialize_primitive!(f64, SIZE_OF_U64);

serialize_primitive!(i128, SIZE_OF_I128);
serialize_primitive!(u128, SIZE_OF_U128);

impl RawSerialize for String {
    fn raw_serialize(&self, from: &mut Write) -> Result<u64, Error> {
        self.as_bytes().raw_serialize(from)
    }
}

impl RawDeserialize for String {
    fn raw_deserialize(to: &mut Read) -> Result<Self, Error> {
        match u8::raw_deserialize_vec(to) {
            Ok(ret) => {
                Ok(String::from_utf8_lossy(&ret).into_owned())
            },
            Err(e) => Err(e)
        }
    }
}
