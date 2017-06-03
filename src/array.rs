use serialize::RawSerialize;
use deserialize::RawDeserialize;
use std::io::{ Error, Read, Write };
use std::slice;
use std::mem;


impl RawSerialize for [u8; 1] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u8; 1], *const u8>(&(*self) as *const [u8; 1]), 1 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u8; 1] {
    fn raw_deserialize(from: &mut Read) -> Result<[u8; 1], Error> {
        unsafe { let mut buffer: [u8; 1] = [0u8; 1];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u8; 1], *mut u8>((&mut buffer) as *mut [u8; 1]), 1)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u8; 2] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u8; 2], *const u8>(&(*self) as *const [u8; 2]), 2 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u8; 2] {
    fn raw_deserialize(from: &mut Read) -> Result<[u8; 2], Error> {
        unsafe { let mut buffer: [u8; 2] = [0u8; 2];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u8; 2], *mut u8>((&mut buffer) as *mut [u8; 2]), 2)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u8; 3] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u8; 3], *const u8>(&(*self) as *const [u8; 3]), 3 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u8; 3] {
    fn raw_deserialize(from: &mut Read) -> Result<[u8; 3], Error> {
        unsafe { let mut buffer: [u8; 3] = [0u8; 3];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u8; 3], *mut u8>((&mut buffer) as *mut [u8; 3]), 3)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u8; 4] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u8; 4], *const u8>(&(*self) as *const [u8; 4]), 4 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u8; 4] {
    fn raw_deserialize(from: &mut Read) -> Result<[u8; 4], Error> {
        unsafe { let mut buffer: [u8; 4] = [0u8; 4];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u8; 4], *mut u8>((&mut buffer) as *mut [u8; 4]), 4)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u8; 5] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u8; 5], *const u8>(&(*self) as *const [u8; 5]), 5 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u8; 5] {
    fn raw_deserialize(from: &mut Read) -> Result<[u8; 5], Error> {
        unsafe { let mut buffer: [u8; 5] = [0u8; 5];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u8; 5], *mut u8>((&mut buffer) as *mut [u8; 5]), 5)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u8; 6] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u8; 6], *const u8>(&(*self) as *const [u8; 6]), 6 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u8; 6] {
    fn raw_deserialize(from: &mut Read) -> Result<[u8; 6], Error> {
        unsafe { let mut buffer: [u8; 6] = [0u8; 6];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u8; 6], *mut u8>((&mut buffer) as *mut [u8; 6]), 6)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u8; 7] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u8; 7], *const u8>(&(*self) as *const [u8; 7]), 7 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u8; 7] {
    fn raw_deserialize(from: &mut Read) -> Result<[u8; 7], Error> {
        unsafe { let mut buffer: [u8; 7] = [0u8; 7];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u8; 7], *mut u8>((&mut buffer) as *mut [u8; 7]), 7)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u8; 8] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u8; 8], *const u8>(&(*self) as *const [u8; 8]), 8 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u8; 8] {
    fn raw_deserialize(from: &mut Read) -> Result<[u8; 8], Error> {
        unsafe { let mut buffer: [u8; 8] = [0u8; 8];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u8; 8], *mut u8>((&mut buffer) as *mut [u8; 8]), 8)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u8; 9] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u8; 9], *const u8>(&(*self) as *const [u8; 9]), 9 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u8; 9] {
    fn raw_deserialize(from: &mut Read) -> Result<[u8; 9], Error> {
        unsafe { let mut buffer: [u8; 9] = [0u8; 9];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u8; 9], *mut u8>((&mut buffer) as *mut [u8; 9]), 9)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u8; 10] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u8; 10], *const u8>(&(*self) as *const [u8; 10]), 10 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u8; 10] {
    fn raw_deserialize(from: &mut Read) -> Result<[u8; 10], Error> {
        unsafe { let mut buffer: [u8; 10] = [0u8; 10];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u8; 10], *mut u8>((&mut buffer) as *mut [u8; 10]), 10)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u8; 11] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u8; 11], *const u8>(&(*self) as *const [u8; 11]), 11 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u8; 11] {
    fn raw_deserialize(from: &mut Read) -> Result<[u8; 11], Error> {
        unsafe { let mut buffer: [u8; 11] = [0u8; 11];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u8; 11], *mut u8>((&mut buffer) as *mut [u8; 11]), 11)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u8; 12] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u8; 12], *const u8>(&(*self) as *const [u8; 12]), 12 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u8; 12] {
    fn raw_deserialize(from: &mut Read) -> Result<[u8; 12], Error> {
        unsafe { let mut buffer: [u8; 12] = [0u8; 12];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u8; 12], *mut u8>((&mut buffer) as *mut [u8; 12]), 12)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u8; 13] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u8; 13], *const u8>(&(*self) as *const [u8; 13]), 13 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u8; 13] {
    fn raw_deserialize(from: &mut Read) -> Result<[u8; 13], Error> {
        unsafe { let mut buffer: [u8; 13] = [0u8; 13];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u8; 13], *mut u8>((&mut buffer) as *mut [u8; 13]), 13)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u8; 14] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u8; 14], *const u8>(&(*self) as *const [u8; 14]), 14 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u8; 14] {
    fn raw_deserialize(from: &mut Read) -> Result<[u8; 14], Error> {
        unsafe { let mut buffer: [u8; 14] = [0u8; 14];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u8; 14], *mut u8>((&mut buffer) as *mut [u8; 14]), 14)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u8; 15] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u8; 15], *const u8>(&(*self) as *const [u8; 15]), 15 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u8; 15] {
    fn raw_deserialize(from: &mut Read) -> Result<[u8; 15], Error> {
        unsafe { let mut buffer: [u8; 15] = [0u8; 15];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u8; 15], *mut u8>((&mut buffer) as *mut [u8; 15]), 15)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u8; 16] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u8; 16], *const u8>(&(*self) as *const [u8; 16]), 16 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u8; 16] {
    fn raw_deserialize(from: &mut Read) -> Result<[u8; 16], Error> {
        unsafe { let mut buffer: [u8; 16] = [0u8; 16];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u8; 16], *mut u8>((&mut buffer) as *mut [u8; 16]), 16)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u8; 17] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u8; 17], *const u8>(&(*self) as *const [u8; 17]), 17 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u8; 17] {
    fn raw_deserialize(from: &mut Read) -> Result<[u8; 17], Error> {
        unsafe { let mut buffer: [u8; 17] = [0u8; 17];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u8; 17], *mut u8>((&mut buffer) as *mut [u8; 17]), 17)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u8; 18] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u8; 18], *const u8>(&(*self) as *const [u8; 18]), 18 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u8; 18] {
    fn raw_deserialize(from: &mut Read) -> Result<[u8; 18], Error> {
        unsafe { let mut buffer: [u8; 18] = [0u8; 18];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u8; 18], *mut u8>((&mut buffer) as *mut [u8; 18]), 18)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u8; 19] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u8; 19], *const u8>(&(*self) as *const [u8; 19]), 19 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u8; 19] {
    fn raw_deserialize(from: &mut Read) -> Result<[u8; 19], Error> {
        unsafe { let mut buffer: [u8; 19] = [0u8; 19];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u8; 19], *mut u8>((&mut buffer) as *mut [u8; 19]), 19)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u8; 20] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u8; 20], *const u8>(&(*self) as *const [u8; 20]), 20 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u8; 20] {
    fn raw_deserialize(from: &mut Read) -> Result<[u8; 20], Error> {
        unsafe { let mut buffer: [u8; 20] = [0u8; 20];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u8; 20], *mut u8>((&mut buffer) as *mut [u8; 20]), 20)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u8; 21] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u8; 21], *const u8>(&(*self) as *const [u8; 21]), 21 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u8; 21] {
    fn raw_deserialize(from: &mut Read) -> Result<[u8; 21], Error> {
        unsafe { let mut buffer: [u8; 21] = [0u8; 21];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u8; 21], *mut u8>((&mut buffer) as *mut [u8; 21]), 21)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u8; 22] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u8; 22], *const u8>(&(*self) as *const [u8; 22]), 22 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u8; 22] {
    fn raw_deserialize(from: &mut Read) -> Result<[u8; 22], Error> {
        unsafe { let mut buffer: [u8; 22] = [0u8; 22];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u8; 22], *mut u8>((&mut buffer) as *mut [u8; 22]), 22)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u8; 23] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u8; 23], *const u8>(&(*self) as *const [u8; 23]), 23 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u8; 23] {
    fn raw_deserialize(from: &mut Read) -> Result<[u8; 23], Error> {
        unsafe { let mut buffer: [u8; 23] = [0u8; 23];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u8; 23], *mut u8>((&mut buffer) as *mut [u8; 23]), 23)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u8; 24] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u8; 24], *const u8>(&(*self) as *const [u8; 24]), 24 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u8; 24] {
    fn raw_deserialize(from: &mut Read) -> Result<[u8; 24], Error> {
        unsafe { let mut buffer: [u8; 24] = [0u8; 24];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u8; 24], *mut u8>((&mut buffer) as *mut [u8; 24]), 24)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u8; 25] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u8; 25], *const u8>(&(*self) as *const [u8; 25]), 25 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u8; 25] {
    fn raw_deserialize(from: &mut Read) -> Result<[u8; 25], Error> {
        unsafe { let mut buffer: [u8; 25] = [0u8; 25];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u8; 25], *mut u8>((&mut buffer) as *mut [u8; 25]), 25)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u8; 26] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u8; 26], *const u8>(&(*self) as *const [u8; 26]), 26 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u8; 26] {
    fn raw_deserialize(from: &mut Read) -> Result<[u8; 26], Error> {
        unsafe { let mut buffer: [u8; 26] = [0u8; 26];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u8; 26], *mut u8>((&mut buffer) as *mut [u8; 26]), 26)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u8; 27] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u8; 27], *const u8>(&(*self) as *const [u8; 27]), 27 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u8; 27] {
    fn raw_deserialize(from: &mut Read) -> Result<[u8; 27], Error> {
        unsafe { let mut buffer: [u8; 27] = [0u8; 27];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u8; 27], *mut u8>((&mut buffer) as *mut [u8; 27]), 27)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u8; 28] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u8; 28], *const u8>(&(*self) as *const [u8; 28]), 28 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u8; 28] {
    fn raw_deserialize(from: &mut Read) -> Result<[u8; 28], Error> {
        unsafe { let mut buffer: [u8; 28] = [0u8; 28];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u8; 28], *mut u8>((&mut buffer) as *mut [u8; 28]), 28)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u8; 29] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u8; 29], *const u8>(&(*self) as *const [u8; 29]), 29 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u8; 29] {
    fn raw_deserialize(from: &mut Read) -> Result<[u8; 29], Error> {
        unsafe { let mut buffer: [u8; 29] = [0u8; 29];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u8; 29], *mut u8>((&mut buffer) as *mut [u8; 29]), 29)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u8; 30] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u8; 30], *const u8>(&(*self) as *const [u8; 30]), 30 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u8; 30] {
    fn raw_deserialize(from: &mut Read) -> Result<[u8; 30], Error> {
        unsafe { let mut buffer: [u8; 30] = [0u8; 30];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u8; 30], *mut u8>((&mut buffer) as *mut [u8; 30]), 30)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u8; 31] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u8; 31], *const u8>(&(*self) as *const [u8; 31]), 31 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u8; 31] {
    fn raw_deserialize(from: &mut Read) -> Result<[u8; 31], Error> {
        unsafe { let mut buffer: [u8; 31] = [0u8; 31];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u8; 31], *mut u8>((&mut buffer) as *mut [u8; 31]), 31)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u8; 32] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u8; 32], *const u8>(&(*self) as *const [u8; 32]), 32 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u8; 32] {
    fn raw_deserialize(from: &mut Read) -> Result<[u8; 32], Error> {
        unsafe { let mut buffer: [u8; 32] = [0u8; 32];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u8; 32], *mut u8>((&mut buffer) as *mut [u8; 32]), 32)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i8; 1] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i8; 1], *const u8>(&(*self) as *const [i8; 1]), 1 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i8; 1] {
    fn raw_deserialize(from: &mut Read) -> Result<[i8; 1], Error> {
        unsafe { let mut buffer: [i8; 1] = [0i8; 1];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i8; 1], *mut u8>((&mut buffer) as *mut [i8; 1]), 1)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i8; 2] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i8; 2], *const u8>(&(*self) as *const [i8; 2]), 2 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i8; 2] {
    fn raw_deserialize(from: &mut Read) -> Result<[i8; 2], Error> {
        unsafe { let mut buffer: [i8; 2] = [0i8; 2];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i8; 2], *mut u8>((&mut buffer) as *mut [i8; 2]), 2)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i8; 3] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i8; 3], *const u8>(&(*self) as *const [i8; 3]), 3 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i8; 3] {
    fn raw_deserialize(from: &mut Read) -> Result<[i8; 3], Error> {
        unsafe { let mut buffer: [i8; 3] = [0i8; 3];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i8; 3], *mut u8>((&mut buffer) as *mut [i8; 3]), 3)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i8; 4] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i8; 4], *const u8>(&(*self) as *const [i8; 4]), 4 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i8; 4] {
    fn raw_deserialize(from: &mut Read) -> Result<[i8; 4], Error> {
        unsafe { let mut buffer: [i8; 4] = [0i8; 4];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i8; 4], *mut u8>((&mut buffer) as *mut [i8; 4]), 4)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i8; 5] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i8; 5], *const u8>(&(*self) as *const [i8; 5]), 5 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i8; 5] {
    fn raw_deserialize(from: &mut Read) -> Result<[i8; 5], Error> {
        unsafe { let mut buffer: [i8; 5] = [0i8; 5];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i8; 5], *mut u8>((&mut buffer) as *mut [i8; 5]), 5)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i8; 6] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i8; 6], *const u8>(&(*self) as *const [i8; 6]), 6 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i8; 6] {
    fn raw_deserialize(from: &mut Read) -> Result<[i8; 6], Error> {
        unsafe { let mut buffer: [i8; 6] = [0i8; 6];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i8; 6], *mut u8>((&mut buffer) as *mut [i8; 6]), 6)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i8; 7] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i8; 7], *const u8>(&(*self) as *const [i8; 7]), 7 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i8; 7] {
    fn raw_deserialize(from: &mut Read) -> Result<[i8; 7], Error> {
        unsafe { let mut buffer: [i8; 7] = [0i8; 7];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i8; 7], *mut u8>((&mut buffer) as *mut [i8; 7]), 7)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i8; 8] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i8; 8], *const u8>(&(*self) as *const [i8; 8]), 8 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i8; 8] {
    fn raw_deserialize(from: &mut Read) -> Result<[i8; 8], Error> {
        unsafe { let mut buffer: [i8; 8] = [0i8; 8];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i8; 8], *mut u8>((&mut buffer) as *mut [i8; 8]), 8)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i8; 9] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i8; 9], *const u8>(&(*self) as *const [i8; 9]), 9 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i8; 9] {
    fn raw_deserialize(from: &mut Read) -> Result<[i8; 9], Error> {
        unsafe { let mut buffer: [i8; 9] = [0i8; 9];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i8; 9], *mut u8>((&mut buffer) as *mut [i8; 9]), 9)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i8; 10] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i8; 10], *const u8>(&(*self) as *const [i8; 10]), 10 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i8; 10] {
    fn raw_deserialize(from: &mut Read) -> Result<[i8; 10], Error> {
        unsafe { let mut buffer: [i8; 10] = [0i8; 10];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i8; 10], *mut u8>((&mut buffer) as *mut [i8; 10]), 10)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i8; 11] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i8; 11], *const u8>(&(*self) as *const [i8; 11]), 11 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i8; 11] {
    fn raw_deserialize(from: &mut Read) -> Result<[i8; 11], Error> {
        unsafe { let mut buffer: [i8; 11] = [0i8; 11];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i8; 11], *mut u8>((&mut buffer) as *mut [i8; 11]), 11)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i8; 12] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i8; 12], *const u8>(&(*self) as *const [i8; 12]), 12 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i8; 12] {
    fn raw_deserialize(from: &mut Read) -> Result<[i8; 12], Error> {
        unsafe { let mut buffer: [i8; 12] = [0i8; 12];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i8; 12], *mut u8>((&mut buffer) as *mut [i8; 12]), 12)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i8; 13] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i8; 13], *const u8>(&(*self) as *const [i8; 13]), 13 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i8; 13] {
    fn raw_deserialize(from: &mut Read) -> Result<[i8; 13], Error> {
        unsafe { let mut buffer: [i8; 13] = [0i8; 13];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i8; 13], *mut u8>((&mut buffer) as *mut [i8; 13]), 13)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i8; 14] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i8; 14], *const u8>(&(*self) as *const [i8; 14]), 14 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i8; 14] {
    fn raw_deserialize(from: &mut Read) -> Result<[i8; 14], Error> {
        unsafe { let mut buffer: [i8; 14] = [0i8; 14];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i8; 14], *mut u8>((&mut buffer) as *mut [i8; 14]), 14)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i8; 15] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i8; 15], *const u8>(&(*self) as *const [i8; 15]), 15 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i8; 15] {
    fn raw_deserialize(from: &mut Read) -> Result<[i8; 15], Error> {
        unsafe { let mut buffer: [i8; 15] = [0i8; 15];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i8; 15], *mut u8>((&mut buffer) as *mut [i8; 15]), 15)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i8; 16] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i8; 16], *const u8>(&(*self) as *const [i8; 16]), 16 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i8; 16] {
    fn raw_deserialize(from: &mut Read) -> Result<[i8; 16], Error> {
        unsafe { let mut buffer: [i8; 16] = [0i8; 16];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i8; 16], *mut u8>((&mut buffer) as *mut [i8; 16]), 16)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i8; 17] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i8; 17], *const u8>(&(*self) as *const [i8; 17]), 17 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i8; 17] {
    fn raw_deserialize(from: &mut Read) -> Result<[i8; 17], Error> {
        unsafe { let mut buffer: [i8; 17] = [0i8; 17];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i8; 17], *mut u8>((&mut buffer) as *mut [i8; 17]), 17)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i8; 18] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i8; 18], *const u8>(&(*self) as *const [i8; 18]), 18 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i8; 18] {
    fn raw_deserialize(from: &mut Read) -> Result<[i8; 18], Error> {
        unsafe { let mut buffer: [i8; 18] = [0i8; 18];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i8; 18], *mut u8>((&mut buffer) as *mut [i8; 18]), 18)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i8; 19] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i8; 19], *const u8>(&(*self) as *const [i8; 19]), 19 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i8; 19] {
    fn raw_deserialize(from: &mut Read) -> Result<[i8; 19], Error> {
        unsafe { let mut buffer: [i8; 19] = [0i8; 19];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i8; 19], *mut u8>((&mut buffer) as *mut [i8; 19]), 19)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i8; 20] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i8; 20], *const u8>(&(*self) as *const [i8; 20]), 20 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i8; 20] {
    fn raw_deserialize(from: &mut Read) -> Result<[i8; 20], Error> {
        unsafe { let mut buffer: [i8; 20] = [0i8; 20];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i8; 20], *mut u8>((&mut buffer) as *mut [i8; 20]), 20)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i8; 21] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i8; 21], *const u8>(&(*self) as *const [i8; 21]), 21 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i8; 21] {
    fn raw_deserialize(from: &mut Read) -> Result<[i8; 21], Error> {
        unsafe { let mut buffer: [i8; 21] = [0i8; 21];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i8; 21], *mut u8>((&mut buffer) as *mut [i8; 21]), 21)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i8; 22] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i8; 22], *const u8>(&(*self) as *const [i8; 22]), 22 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i8; 22] {
    fn raw_deserialize(from: &mut Read) -> Result<[i8; 22], Error> {
        unsafe { let mut buffer: [i8; 22] = [0i8; 22];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i8; 22], *mut u8>((&mut buffer) as *mut [i8; 22]), 22)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i8; 23] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i8; 23], *const u8>(&(*self) as *const [i8; 23]), 23 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i8; 23] {
    fn raw_deserialize(from: &mut Read) -> Result<[i8; 23], Error> {
        unsafe { let mut buffer: [i8; 23] = [0i8; 23];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i8; 23], *mut u8>((&mut buffer) as *mut [i8; 23]), 23)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i8; 24] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i8; 24], *const u8>(&(*self) as *const [i8; 24]), 24 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i8; 24] {
    fn raw_deserialize(from: &mut Read) -> Result<[i8; 24], Error> {
        unsafe { let mut buffer: [i8; 24] = [0i8; 24];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i8; 24], *mut u8>((&mut buffer) as *mut [i8; 24]), 24)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i8; 25] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i8; 25], *const u8>(&(*self) as *const [i8; 25]), 25 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i8; 25] {
    fn raw_deserialize(from: &mut Read) -> Result<[i8; 25], Error> {
        unsafe { let mut buffer: [i8; 25] = [0i8; 25];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i8; 25], *mut u8>((&mut buffer) as *mut [i8; 25]), 25)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i8; 26] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i8; 26], *const u8>(&(*self) as *const [i8; 26]), 26 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i8; 26] {
    fn raw_deserialize(from: &mut Read) -> Result<[i8; 26], Error> {
        unsafe { let mut buffer: [i8; 26] = [0i8; 26];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i8; 26], *mut u8>((&mut buffer) as *mut [i8; 26]), 26)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i8; 27] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i8; 27], *const u8>(&(*self) as *const [i8; 27]), 27 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i8; 27] {
    fn raw_deserialize(from: &mut Read) -> Result<[i8; 27], Error> {
        unsafe { let mut buffer: [i8; 27] = [0i8; 27];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i8; 27], *mut u8>((&mut buffer) as *mut [i8; 27]), 27)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i8; 28] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i8; 28], *const u8>(&(*self) as *const [i8; 28]), 28 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i8; 28] {
    fn raw_deserialize(from: &mut Read) -> Result<[i8; 28], Error> {
        unsafe { let mut buffer: [i8; 28] = [0i8; 28];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i8; 28], *mut u8>((&mut buffer) as *mut [i8; 28]), 28)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i8; 29] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i8; 29], *const u8>(&(*self) as *const [i8; 29]), 29 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i8; 29] {
    fn raw_deserialize(from: &mut Read) -> Result<[i8; 29], Error> {
        unsafe { let mut buffer: [i8; 29] = [0i8; 29];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i8; 29], *mut u8>((&mut buffer) as *mut [i8; 29]), 29)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i8; 30] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i8; 30], *const u8>(&(*self) as *const [i8; 30]), 30 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i8; 30] {
    fn raw_deserialize(from: &mut Read) -> Result<[i8; 30], Error> {
        unsafe { let mut buffer: [i8; 30] = [0i8; 30];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i8; 30], *mut u8>((&mut buffer) as *mut [i8; 30]), 30)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i8; 31] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i8; 31], *const u8>(&(*self) as *const [i8; 31]), 31 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i8; 31] {
    fn raw_deserialize(from: &mut Read) -> Result<[i8; 31], Error> {
        unsafe { let mut buffer: [i8; 31] = [0i8; 31];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i8; 31], *mut u8>((&mut buffer) as *mut [i8; 31]), 31)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i8; 32] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i8; 32], *const u8>(&(*self) as *const [i8; 32]), 32 * 1)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i8; 32] {
    fn raw_deserialize(from: &mut Read) -> Result<[i8; 32], Error> {
        unsafe { let mut buffer: [i8; 32] = [0i8; 32];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i8; 32], *mut u8>((&mut buffer) as *mut [i8; 32]), 32)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u16; 1] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u16; 1], *const u8>(&(*self) as *const [u16; 1]), 1 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u16; 1] {
    fn raw_deserialize(from: &mut Read) -> Result<[u16; 1], Error> {
        unsafe { let mut buffer: [u16; 1] = [0u16; 1];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u16; 1], *mut u8>((&mut buffer) as *mut [u16; 1]), 2)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u16; 2] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u16; 2], *const u8>(&(*self) as *const [u16; 2]), 2 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u16; 2] {
    fn raw_deserialize(from: &mut Read) -> Result<[u16; 2], Error> {
        unsafe { let mut buffer: [u16; 2] = [0u16; 2];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u16; 2], *mut u8>((&mut buffer) as *mut [u16; 2]), 4)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u16; 3] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u16; 3], *const u8>(&(*self) as *const [u16; 3]), 3 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u16; 3] {
    fn raw_deserialize(from: &mut Read) -> Result<[u16; 3], Error> {
        unsafe { let mut buffer: [u16; 3] = [0u16; 3];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u16; 3], *mut u8>((&mut buffer) as *mut [u16; 3]), 6)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u16; 4] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u16; 4], *const u8>(&(*self) as *const [u16; 4]), 4 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u16; 4] {
    fn raw_deserialize(from: &mut Read) -> Result<[u16; 4], Error> {
        unsafe { let mut buffer: [u16; 4] = [0u16; 4];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u16; 4], *mut u8>((&mut buffer) as *mut [u16; 4]), 8)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u16; 5] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u16; 5], *const u8>(&(*self) as *const [u16; 5]), 5 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u16; 5] {
    fn raw_deserialize(from: &mut Read) -> Result<[u16; 5], Error> {
        unsafe { let mut buffer: [u16; 5] = [0u16; 5];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u16; 5], *mut u8>((&mut buffer) as *mut [u16; 5]), 10)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u16; 6] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u16; 6], *const u8>(&(*self) as *const [u16; 6]), 6 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u16; 6] {
    fn raw_deserialize(from: &mut Read) -> Result<[u16; 6], Error> {
        unsafe { let mut buffer: [u16; 6] = [0u16; 6];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u16; 6], *mut u8>((&mut buffer) as *mut [u16; 6]), 12)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u16; 7] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u16; 7], *const u8>(&(*self) as *const [u16; 7]), 7 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u16; 7] {
    fn raw_deserialize(from: &mut Read) -> Result<[u16; 7], Error> {
        unsafe { let mut buffer: [u16; 7] = [0u16; 7];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u16; 7], *mut u8>((&mut buffer) as *mut [u16; 7]), 14)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u16; 8] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u16; 8], *const u8>(&(*self) as *const [u16; 8]), 8 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u16; 8] {
    fn raw_deserialize(from: &mut Read) -> Result<[u16; 8], Error> {
        unsafe { let mut buffer: [u16; 8] = [0u16; 8];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u16; 8], *mut u8>((&mut buffer) as *mut [u16; 8]), 16)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u16; 9] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u16; 9], *const u8>(&(*self) as *const [u16; 9]), 9 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u16; 9] {
    fn raw_deserialize(from: &mut Read) -> Result<[u16; 9], Error> {
        unsafe { let mut buffer: [u16; 9] = [0u16; 9];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u16; 9], *mut u8>((&mut buffer) as *mut [u16; 9]), 18)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u16; 10] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u16; 10], *const u8>(&(*self) as *const [u16; 10]), 10 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u16; 10] {
    fn raw_deserialize(from: &mut Read) -> Result<[u16; 10], Error> {
        unsafe { let mut buffer: [u16; 10] = [0u16; 10];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u16; 10], *mut u8>((&mut buffer) as *mut [u16; 10]), 20)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u16; 11] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u16; 11], *const u8>(&(*self) as *const [u16; 11]), 11 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u16; 11] {
    fn raw_deserialize(from: &mut Read) -> Result<[u16; 11], Error> {
        unsafe { let mut buffer: [u16; 11] = [0u16; 11];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u16; 11], *mut u8>((&mut buffer) as *mut [u16; 11]), 22)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u16; 12] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u16; 12], *const u8>(&(*self) as *const [u16; 12]), 12 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u16; 12] {
    fn raw_deserialize(from: &mut Read) -> Result<[u16; 12], Error> {
        unsafe { let mut buffer: [u16; 12] = [0u16; 12];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u16; 12], *mut u8>((&mut buffer) as *mut [u16; 12]), 24)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u16; 13] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u16; 13], *const u8>(&(*self) as *const [u16; 13]), 13 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u16; 13] {
    fn raw_deserialize(from: &mut Read) -> Result<[u16; 13], Error> {
        unsafe { let mut buffer: [u16; 13] = [0u16; 13];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u16; 13], *mut u8>((&mut buffer) as *mut [u16; 13]), 26)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u16; 14] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u16; 14], *const u8>(&(*self) as *const [u16; 14]), 14 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u16; 14] {
    fn raw_deserialize(from: &mut Read) -> Result<[u16; 14], Error> {
        unsafe { let mut buffer: [u16; 14] = [0u16; 14];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u16; 14], *mut u8>((&mut buffer) as *mut [u16; 14]), 28)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u16; 15] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u16; 15], *const u8>(&(*self) as *const [u16; 15]), 15 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u16; 15] {
    fn raw_deserialize(from: &mut Read) -> Result<[u16; 15], Error> {
        unsafe { let mut buffer: [u16; 15] = [0u16; 15];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u16; 15], *mut u8>((&mut buffer) as *mut [u16; 15]), 30)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u16; 16] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u16; 16], *const u8>(&(*self) as *const [u16; 16]), 16 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u16; 16] {
    fn raw_deserialize(from: &mut Read) -> Result<[u16; 16], Error> {
        unsafe { let mut buffer: [u16; 16] = [0u16; 16];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u16; 16], *mut u8>((&mut buffer) as *mut [u16; 16]), 32)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u16; 17] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u16; 17], *const u8>(&(*self) as *const [u16; 17]), 17 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u16; 17] {
    fn raw_deserialize(from: &mut Read) -> Result<[u16; 17], Error> {
        unsafe { let mut buffer: [u16; 17] = [0u16; 17];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u16; 17], *mut u8>((&mut buffer) as *mut [u16; 17]), 34)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u16; 18] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u16; 18], *const u8>(&(*self) as *const [u16; 18]), 18 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u16; 18] {
    fn raw_deserialize(from: &mut Read) -> Result<[u16; 18], Error> {
        unsafe { let mut buffer: [u16; 18] = [0u16; 18];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u16; 18], *mut u8>((&mut buffer) as *mut [u16; 18]), 36)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u16; 19] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u16; 19], *const u8>(&(*self) as *const [u16; 19]), 19 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u16; 19] {
    fn raw_deserialize(from: &mut Read) -> Result<[u16; 19], Error> {
        unsafe { let mut buffer: [u16; 19] = [0u16; 19];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u16; 19], *mut u8>((&mut buffer) as *mut [u16; 19]), 38)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u16; 20] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u16; 20], *const u8>(&(*self) as *const [u16; 20]), 20 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u16; 20] {
    fn raw_deserialize(from: &mut Read) -> Result<[u16; 20], Error> {
        unsafe { let mut buffer: [u16; 20] = [0u16; 20];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u16; 20], *mut u8>((&mut buffer) as *mut [u16; 20]), 40)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u16; 21] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u16; 21], *const u8>(&(*self) as *const [u16; 21]), 21 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u16; 21] {
    fn raw_deserialize(from: &mut Read) -> Result<[u16; 21], Error> {
        unsafe { let mut buffer: [u16; 21] = [0u16; 21];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u16; 21], *mut u8>((&mut buffer) as *mut [u16; 21]), 42)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u16; 22] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u16; 22], *const u8>(&(*self) as *const [u16; 22]), 22 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u16; 22] {
    fn raw_deserialize(from: &mut Read) -> Result<[u16; 22], Error> {
        unsafe { let mut buffer: [u16; 22] = [0u16; 22];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u16; 22], *mut u8>((&mut buffer) as *mut [u16; 22]), 44)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u16; 23] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u16; 23], *const u8>(&(*self) as *const [u16; 23]), 23 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u16; 23] {
    fn raw_deserialize(from: &mut Read) -> Result<[u16; 23], Error> {
        unsafe { let mut buffer: [u16; 23] = [0u16; 23];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u16; 23], *mut u8>((&mut buffer) as *mut [u16; 23]), 46)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u16; 24] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u16; 24], *const u8>(&(*self) as *const [u16; 24]), 24 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u16; 24] {
    fn raw_deserialize(from: &mut Read) -> Result<[u16; 24], Error> {
        unsafe { let mut buffer: [u16; 24] = [0u16; 24];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u16; 24], *mut u8>((&mut buffer) as *mut [u16; 24]), 48)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u16; 25] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u16; 25], *const u8>(&(*self) as *const [u16; 25]), 25 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u16; 25] {
    fn raw_deserialize(from: &mut Read) -> Result<[u16; 25], Error> {
        unsafe { let mut buffer: [u16; 25] = [0u16; 25];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u16; 25], *mut u8>((&mut buffer) as *mut [u16; 25]), 50)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u16; 26] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u16; 26], *const u8>(&(*self) as *const [u16; 26]), 26 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u16; 26] {
    fn raw_deserialize(from: &mut Read) -> Result<[u16; 26], Error> {
        unsafe { let mut buffer: [u16; 26] = [0u16; 26];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u16; 26], *mut u8>((&mut buffer) as *mut [u16; 26]), 52)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u16; 27] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u16; 27], *const u8>(&(*self) as *const [u16; 27]), 27 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u16; 27] {
    fn raw_deserialize(from: &mut Read) -> Result<[u16; 27], Error> {
        unsafe { let mut buffer: [u16; 27] = [0u16; 27];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u16; 27], *mut u8>((&mut buffer) as *mut [u16; 27]), 54)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u16; 28] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u16; 28], *const u8>(&(*self) as *const [u16; 28]), 28 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u16; 28] {
    fn raw_deserialize(from: &mut Read) -> Result<[u16; 28], Error> {
        unsafe { let mut buffer: [u16; 28] = [0u16; 28];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u16; 28], *mut u8>((&mut buffer) as *mut [u16; 28]), 56)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u16; 29] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u16; 29], *const u8>(&(*self) as *const [u16; 29]), 29 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u16; 29] {
    fn raw_deserialize(from: &mut Read) -> Result<[u16; 29], Error> {
        unsafe { let mut buffer: [u16; 29] = [0u16; 29];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u16; 29], *mut u8>((&mut buffer) as *mut [u16; 29]), 58)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u16; 30] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u16; 30], *const u8>(&(*self) as *const [u16; 30]), 30 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u16; 30] {
    fn raw_deserialize(from: &mut Read) -> Result<[u16; 30], Error> {
        unsafe { let mut buffer: [u16; 30] = [0u16; 30];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u16; 30], *mut u8>((&mut buffer) as *mut [u16; 30]), 60)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u16; 31] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u16; 31], *const u8>(&(*self) as *const [u16; 31]), 31 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u16; 31] {
    fn raw_deserialize(from: &mut Read) -> Result<[u16; 31], Error> {
        unsafe { let mut buffer: [u16; 31] = [0u16; 31];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u16; 31], *mut u8>((&mut buffer) as *mut [u16; 31]), 62)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u16; 32] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u16; 32], *const u8>(&(*self) as *const [u16; 32]), 32 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u16; 32] {
    fn raw_deserialize(from: &mut Read) -> Result<[u16; 32], Error> {
        unsafe { let mut buffer: [u16; 32] = [0u16; 32];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u16; 32], *mut u8>((&mut buffer) as *mut [u16; 32]), 64)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i16; 1] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i16; 1], *const u8>(&(*self) as *const [i16; 1]), 1 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i16; 1] {
    fn raw_deserialize(from: &mut Read) -> Result<[i16; 1], Error> {
        unsafe { let mut buffer: [i16; 1] = [0i16; 1];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i16; 1], *mut u8>((&mut buffer) as *mut [i16; 1]), 2)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i16; 2] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i16; 2], *const u8>(&(*self) as *const [i16; 2]), 2 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i16; 2] {
    fn raw_deserialize(from: &mut Read) -> Result<[i16; 2], Error> {
        unsafe { let mut buffer: [i16; 2] = [0i16; 2];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i16; 2], *mut u8>((&mut buffer) as *mut [i16; 2]), 4)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i16; 3] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i16; 3], *const u8>(&(*self) as *const [i16; 3]), 3 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i16; 3] {
    fn raw_deserialize(from: &mut Read) -> Result<[i16; 3], Error> {
        unsafe { let mut buffer: [i16; 3] = [0i16; 3];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i16; 3], *mut u8>((&mut buffer) as *mut [i16; 3]), 6)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i16; 4] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i16; 4], *const u8>(&(*self) as *const [i16; 4]), 4 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i16; 4] {
    fn raw_deserialize(from: &mut Read) -> Result<[i16; 4], Error> {
        unsafe { let mut buffer: [i16; 4] = [0i16; 4];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i16; 4], *mut u8>((&mut buffer) as *mut [i16; 4]), 8)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i16; 5] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i16; 5], *const u8>(&(*self) as *const [i16; 5]), 5 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i16; 5] {
    fn raw_deserialize(from: &mut Read) -> Result<[i16; 5], Error> {
        unsafe { let mut buffer: [i16; 5] = [0i16; 5];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i16; 5], *mut u8>((&mut buffer) as *mut [i16; 5]), 10)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i16; 6] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i16; 6], *const u8>(&(*self) as *const [i16; 6]), 6 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i16; 6] {
    fn raw_deserialize(from: &mut Read) -> Result<[i16; 6], Error> {
        unsafe { let mut buffer: [i16; 6] = [0i16; 6];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i16; 6], *mut u8>((&mut buffer) as *mut [i16; 6]), 12)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i16; 7] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i16; 7], *const u8>(&(*self) as *const [i16; 7]), 7 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i16; 7] {
    fn raw_deserialize(from: &mut Read) -> Result<[i16; 7], Error> {
        unsafe { let mut buffer: [i16; 7] = [0i16; 7];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i16; 7], *mut u8>((&mut buffer) as *mut [i16; 7]), 14)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i16; 8] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i16; 8], *const u8>(&(*self) as *const [i16; 8]), 8 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i16; 8] {
    fn raw_deserialize(from: &mut Read) -> Result<[i16; 8], Error> {
        unsafe { let mut buffer: [i16; 8] = [0i16; 8];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i16; 8], *mut u8>((&mut buffer) as *mut [i16; 8]), 16)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i16; 9] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i16; 9], *const u8>(&(*self) as *const [i16; 9]), 9 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i16; 9] {
    fn raw_deserialize(from: &mut Read) -> Result<[i16; 9], Error> {
        unsafe { let mut buffer: [i16; 9] = [0i16; 9];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i16; 9], *mut u8>((&mut buffer) as *mut [i16; 9]), 18)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i16; 10] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i16; 10], *const u8>(&(*self) as *const [i16; 10]), 10 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i16; 10] {
    fn raw_deserialize(from: &mut Read) -> Result<[i16; 10], Error> {
        unsafe { let mut buffer: [i16; 10] = [0i16; 10];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i16; 10], *mut u8>((&mut buffer) as *mut [i16; 10]), 20)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i16; 11] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i16; 11], *const u8>(&(*self) as *const [i16; 11]), 11 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i16; 11] {
    fn raw_deserialize(from: &mut Read) -> Result<[i16; 11], Error> {
        unsafe { let mut buffer: [i16; 11] = [0i16; 11];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i16; 11], *mut u8>((&mut buffer) as *mut [i16; 11]), 22)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i16; 12] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i16; 12], *const u8>(&(*self) as *const [i16; 12]), 12 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i16; 12] {
    fn raw_deserialize(from: &mut Read) -> Result<[i16; 12], Error> {
        unsafe { let mut buffer: [i16; 12] = [0i16; 12];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i16; 12], *mut u8>((&mut buffer) as *mut [i16; 12]), 24)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i16; 13] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i16; 13], *const u8>(&(*self) as *const [i16; 13]), 13 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i16; 13] {
    fn raw_deserialize(from: &mut Read) -> Result<[i16; 13], Error> {
        unsafe { let mut buffer: [i16; 13] = [0i16; 13];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i16; 13], *mut u8>((&mut buffer) as *mut [i16; 13]), 26)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i16; 14] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i16; 14], *const u8>(&(*self) as *const [i16; 14]), 14 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i16; 14] {
    fn raw_deserialize(from: &mut Read) -> Result<[i16; 14], Error> {
        unsafe { let mut buffer: [i16; 14] = [0i16; 14];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i16; 14], *mut u8>((&mut buffer) as *mut [i16; 14]), 28)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i16; 15] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i16; 15], *const u8>(&(*self) as *const [i16; 15]), 15 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i16; 15] {
    fn raw_deserialize(from: &mut Read) -> Result<[i16; 15], Error> {
        unsafe { let mut buffer: [i16; 15] = [0i16; 15];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i16; 15], *mut u8>((&mut buffer) as *mut [i16; 15]), 30)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i16; 16] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i16; 16], *const u8>(&(*self) as *const [i16; 16]), 16 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i16; 16] {
    fn raw_deserialize(from: &mut Read) -> Result<[i16; 16], Error> {
        unsafe { let mut buffer: [i16; 16] = [0i16; 16];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i16; 16], *mut u8>((&mut buffer) as *mut [i16; 16]), 32)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i16; 17] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i16; 17], *const u8>(&(*self) as *const [i16; 17]), 17 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i16; 17] {
    fn raw_deserialize(from: &mut Read) -> Result<[i16; 17], Error> {
        unsafe { let mut buffer: [i16; 17] = [0i16; 17];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i16; 17], *mut u8>((&mut buffer) as *mut [i16; 17]), 34)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i16; 18] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i16; 18], *const u8>(&(*self) as *const [i16; 18]), 18 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i16; 18] {
    fn raw_deserialize(from: &mut Read) -> Result<[i16; 18], Error> {
        unsafe { let mut buffer: [i16; 18] = [0i16; 18];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i16; 18], *mut u8>((&mut buffer) as *mut [i16; 18]), 36)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i16; 19] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i16; 19], *const u8>(&(*self) as *const [i16; 19]), 19 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i16; 19] {
    fn raw_deserialize(from: &mut Read) -> Result<[i16; 19], Error> {
        unsafe { let mut buffer: [i16; 19] = [0i16; 19];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i16; 19], *mut u8>((&mut buffer) as *mut [i16; 19]), 38)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i16; 20] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i16; 20], *const u8>(&(*self) as *const [i16; 20]), 20 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i16; 20] {
    fn raw_deserialize(from: &mut Read) -> Result<[i16; 20], Error> {
        unsafe { let mut buffer: [i16; 20] = [0i16; 20];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i16; 20], *mut u8>((&mut buffer) as *mut [i16; 20]), 40)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i16; 21] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i16; 21], *const u8>(&(*self) as *const [i16; 21]), 21 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i16; 21] {
    fn raw_deserialize(from: &mut Read) -> Result<[i16; 21], Error> {
        unsafe { let mut buffer: [i16; 21] = [0i16; 21];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i16; 21], *mut u8>((&mut buffer) as *mut [i16; 21]), 42)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i16; 22] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i16; 22], *const u8>(&(*self) as *const [i16; 22]), 22 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i16; 22] {
    fn raw_deserialize(from: &mut Read) -> Result<[i16; 22], Error> {
        unsafe { let mut buffer: [i16; 22] = [0i16; 22];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i16; 22], *mut u8>((&mut buffer) as *mut [i16; 22]), 44)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i16; 23] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i16; 23], *const u8>(&(*self) as *const [i16; 23]), 23 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i16; 23] {
    fn raw_deserialize(from: &mut Read) -> Result<[i16; 23], Error> {
        unsafe { let mut buffer: [i16; 23] = [0i16; 23];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i16; 23], *mut u8>((&mut buffer) as *mut [i16; 23]), 46)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i16; 24] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i16; 24], *const u8>(&(*self) as *const [i16; 24]), 24 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i16; 24] {
    fn raw_deserialize(from: &mut Read) -> Result<[i16; 24], Error> {
        unsafe { let mut buffer: [i16; 24] = [0i16; 24];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i16; 24], *mut u8>((&mut buffer) as *mut [i16; 24]), 48)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i16; 25] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i16; 25], *const u8>(&(*self) as *const [i16; 25]), 25 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i16; 25] {
    fn raw_deserialize(from: &mut Read) -> Result<[i16; 25], Error> {
        unsafe { let mut buffer: [i16; 25] = [0i16; 25];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i16; 25], *mut u8>((&mut buffer) as *mut [i16; 25]), 50)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i16; 26] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i16; 26], *const u8>(&(*self) as *const [i16; 26]), 26 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i16; 26] {
    fn raw_deserialize(from: &mut Read) -> Result<[i16; 26], Error> {
        unsafe { let mut buffer: [i16; 26] = [0i16; 26];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i16; 26], *mut u8>((&mut buffer) as *mut [i16; 26]), 52)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i16; 27] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i16; 27], *const u8>(&(*self) as *const [i16; 27]), 27 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i16; 27] {
    fn raw_deserialize(from: &mut Read) -> Result<[i16; 27], Error> {
        unsafe { let mut buffer: [i16; 27] = [0i16; 27];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i16; 27], *mut u8>((&mut buffer) as *mut [i16; 27]), 54)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i16; 28] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i16; 28], *const u8>(&(*self) as *const [i16; 28]), 28 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i16; 28] {
    fn raw_deserialize(from: &mut Read) -> Result<[i16; 28], Error> {
        unsafe { let mut buffer: [i16; 28] = [0i16; 28];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i16; 28], *mut u8>((&mut buffer) as *mut [i16; 28]), 56)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i16; 29] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i16; 29], *const u8>(&(*self) as *const [i16; 29]), 29 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i16; 29] {
    fn raw_deserialize(from: &mut Read) -> Result<[i16; 29], Error> {
        unsafe { let mut buffer: [i16; 29] = [0i16; 29];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i16; 29], *mut u8>((&mut buffer) as *mut [i16; 29]), 58)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i16; 30] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i16; 30], *const u8>(&(*self) as *const [i16; 30]), 30 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i16; 30] {
    fn raw_deserialize(from: &mut Read) -> Result<[i16; 30], Error> {
        unsafe { let mut buffer: [i16; 30] = [0i16; 30];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i16; 30], *mut u8>((&mut buffer) as *mut [i16; 30]), 60)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i16; 31] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i16; 31], *const u8>(&(*self) as *const [i16; 31]), 31 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i16; 31] {
    fn raw_deserialize(from: &mut Read) -> Result<[i16; 31], Error> {
        unsafe { let mut buffer: [i16; 31] = [0i16; 31];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i16; 31], *mut u8>((&mut buffer) as *mut [i16; 31]), 62)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i16; 32] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i16; 32], *const u8>(&(*self) as *const [i16; 32]), 32 * 2)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i16; 32] {
    fn raw_deserialize(from: &mut Read) -> Result<[i16; 32], Error> {
        unsafe { let mut buffer: [i16; 32] = [0i16; 32];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i16; 32], *mut u8>((&mut buffer) as *mut [i16; 32]), 64)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u32; 1] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u32; 1], *const u8>(&(*self) as *const [u32; 1]), 1 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u32; 1] {
    fn raw_deserialize(from: &mut Read) -> Result<[u32; 1], Error> {
        unsafe { let mut buffer: [u32; 1] = [0u32; 1];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u32; 1], *mut u8>((&mut buffer) as *mut [u32; 1]), 4)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u32; 2] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u32; 2], *const u8>(&(*self) as *const [u32; 2]), 2 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u32; 2] {
    fn raw_deserialize(from: &mut Read) -> Result<[u32; 2], Error> {
        unsafe { let mut buffer: [u32; 2] = [0u32; 2];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u32; 2], *mut u8>((&mut buffer) as *mut [u32; 2]), 8)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u32; 3] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u32; 3], *const u8>(&(*self) as *const [u32; 3]), 3 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u32; 3] {
    fn raw_deserialize(from: &mut Read) -> Result<[u32; 3], Error> {
        unsafe { let mut buffer: [u32; 3] = [0u32; 3];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u32; 3], *mut u8>((&mut buffer) as *mut [u32; 3]), 12)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u32; 4] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u32; 4], *const u8>(&(*self) as *const [u32; 4]), 4 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u32; 4] {
    fn raw_deserialize(from: &mut Read) -> Result<[u32; 4], Error> {
        unsafe { let mut buffer: [u32; 4] = [0u32; 4];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u32; 4], *mut u8>((&mut buffer) as *mut [u32; 4]), 16)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u32; 5] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u32; 5], *const u8>(&(*self) as *const [u32; 5]), 5 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u32; 5] {
    fn raw_deserialize(from: &mut Read) -> Result<[u32; 5], Error> {
        unsafe { let mut buffer: [u32; 5] = [0u32; 5];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u32; 5], *mut u8>((&mut buffer) as *mut [u32; 5]), 20)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u32; 6] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u32; 6], *const u8>(&(*self) as *const [u32; 6]), 6 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u32; 6] {
    fn raw_deserialize(from: &mut Read) -> Result<[u32; 6], Error> {
        unsafe { let mut buffer: [u32; 6] = [0u32; 6];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u32; 6], *mut u8>((&mut buffer) as *mut [u32; 6]), 24)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u32; 7] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u32; 7], *const u8>(&(*self) as *const [u32; 7]), 7 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u32; 7] {
    fn raw_deserialize(from: &mut Read) -> Result<[u32; 7], Error> {
        unsafe { let mut buffer: [u32; 7] = [0u32; 7];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u32; 7], *mut u8>((&mut buffer) as *mut [u32; 7]), 28)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u32; 8] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u32; 8], *const u8>(&(*self) as *const [u32; 8]), 8 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u32; 8] {
    fn raw_deserialize(from: &mut Read) -> Result<[u32; 8], Error> {
        unsafe { let mut buffer: [u32; 8] = [0u32; 8];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u32; 8], *mut u8>((&mut buffer) as *mut [u32; 8]), 32)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u32; 9] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u32; 9], *const u8>(&(*self) as *const [u32; 9]), 9 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u32; 9] {
    fn raw_deserialize(from: &mut Read) -> Result<[u32; 9], Error> {
        unsafe { let mut buffer: [u32; 9] = [0u32; 9];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u32; 9], *mut u8>((&mut buffer) as *mut [u32; 9]), 36)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u32; 10] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u32; 10], *const u8>(&(*self) as *const [u32; 10]), 10 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u32; 10] {
    fn raw_deserialize(from: &mut Read) -> Result<[u32; 10], Error> {
        unsafe { let mut buffer: [u32; 10] = [0u32; 10];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u32; 10], *mut u8>((&mut buffer) as *mut [u32; 10]), 40)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u32; 11] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u32; 11], *const u8>(&(*self) as *const [u32; 11]), 11 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u32; 11] {
    fn raw_deserialize(from: &mut Read) -> Result<[u32; 11], Error> {
        unsafe { let mut buffer: [u32; 11] = [0u32; 11];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u32; 11], *mut u8>((&mut buffer) as *mut [u32; 11]), 44)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u32; 12] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u32; 12], *const u8>(&(*self) as *const [u32; 12]), 12 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u32; 12] {
    fn raw_deserialize(from: &mut Read) -> Result<[u32; 12], Error> {
        unsafe { let mut buffer: [u32; 12] = [0u32; 12];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u32; 12], *mut u8>((&mut buffer) as *mut [u32; 12]), 48)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u32; 13] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u32; 13], *const u8>(&(*self) as *const [u32; 13]), 13 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u32; 13] {
    fn raw_deserialize(from: &mut Read) -> Result<[u32; 13], Error> {
        unsafe { let mut buffer: [u32; 13] = [0u32; 13];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u32; 13], *mut u8>((&mut buffer) as *mut [u32; 13]), 52)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u32; 14] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u32; 14], *const u8>(&(*self) as *const [u32; 14]), 14 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u32; 14] {
    fn raw_deserialize(from: &mut Read) -> Result<[u32; 14], Error> {
        unsafe { let mut buffer: [u32; 14] = [0u32; 14];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u32; 14], *mut u8>((&mut buffer) as *mut [u32; 14]), 56)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u32; 15] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u32; 15], *const u8>(&(*self) as *const [u32; 15]), 15 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u32; 15] {
    fn raw_deserialize(from: &mut Read) -> Result<[u32; 15], Error> {
        unsafe { let mut buffer: [u32; 15] = [0u32; 15];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u32; 15], *mut u8>((&mut buffer) as *mut [u32; 15]), 60)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u32; 16] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u32; 16], *const u8>(&(*self) as *const [u32; 16]), 16 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u32; 16] {
    fn raw_deserialize(from: &mut Read) -> Result<[u32; 16], Error> {
        unsafe { let mut buffer: [u32; 16] = [0u32; 16];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u32; 16], *mut u8>((&mut buffer) as *mut [u32; 16]), 64)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u32; 17] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u32; 17], *const u8>(&(*self) as *const [u32; 17]), 17 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u32; 17] {
    fn raw_deserialize(from: &mut Read) -> Result<[u32; 17], Error> {
        unsafe { let mut buffer: [u32; 17] = [0u32; 17];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u32; 17], *mut u8>((&mut buffer) as *mut [u32; 17]), 68)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u32; 18] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u32; 18], *const u8>(&(*self) as *const [u32; 18]), 18 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u32; 18] {
    fn raw_deserialize(from: &mut Read) -> Result<[u32; 18], Error> {
        unsafe { let mut buffer: [u32; 18] = [0u32; 18];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u32; 18], *mut u8>((&mut buffer) as *mut [u32; 18]), 72)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u32; 19] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u32; 19], *const u8>(&(*self) as *const [u32; 19]), 19 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u32; 19] {
    fn raw_deserialize(from: &mut Read) -> Result<[u32; 19], Error> {
        unsafe { let mut buffer: [u32; 19] = [0u32; 19];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u32; 19], *mut u8>((&mut buffer) as *mut [u32; 19]), 76)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u32; 20] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u32; 20], *const u8>(&(*self) as *const [u32; 20]), 20 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u32; 20] {
    fn raw_deserialize(from: &mut Read) -> Result<[u32; 20], Error> {
        unsafe { let mut buffer: [u32; 20] = [0u32; 20];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u32; 20], *mut u8>((&mut buffer) as *mut [u32; 20]), 80)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u32; 21] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u32; 21], *const u8>(&(*self) as *const [u32; 21]), 21 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u32; 21] {
    fn raw_deserialize(from: &mut Read) -> Result<[u32; 21], Error> {
        unsafe { let mut buffer: [u32; 21] = [0u32; 21];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u32; 21], *mut u8>((&mut buffer) as *mut [u32; 21]), 84)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u32; 22] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u32; 22], *const u8>(&(*self) as *const [u32; 22]), 22 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u32; 22] {
    fn raw_deserialize(from: &mut Read) -> Result<[u32; 22], Error> {
        unsafe { let mut buffer: [u32; 22] = [0u32; 22];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u32; 22], *mut u8>((&mut buffer) as *mut [u32; 22]), 88)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u32; 23] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u32; 23], *const u8>(&(*self) as *const [u32; 23]), 23 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u32; 23] {
    fn raw_deserialize(from: &mut Read) -> Result<[u32; 23], Error> {
        unsafe { let mut buffer: [u32; 23] = [0u32; 23];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u32; 23], *mut u8>((&mut buffer) as *mut [u32; 23]), 92)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u32; 24] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u32; 24], *const u8>(&(*self) as *const [u32; 24]), 24 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u32; 24] {
    fn raw_deserialize(from: &mut Read) -> Result<[u32; 24], Error> {
        unsafe { let mut buffer: [u32; 24] = [0u32; 24];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u32; 24], *mut u8>((&mut buffer) as *mut [u32; 24]), 96)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u32; 25] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u32; 25], *const u8>(&(*self) as *const [u32; 25]), 25 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u32; 25] {
    fn raw_deserialize(from: &mut Read) -> Result<[u32; 25], Error> {
        unsafe { let mut buffer: [u32; 25] = [0u32; 25];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u32; 25], *mut u8>((&mut buffer) as *mut [u32; 25]), 100)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u32; 26] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u32; 26], *const u8>(&(*self) as *const [u32; 26]), 26 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u32; 26] {
    fn raw_deserialize(from: &mut Read) -> Result<[u32; 26], Error> {
        unsafe { let mut buffer: [u32; 26] = [0u32; 26];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u32; 26], *mut u8>((&mut buffer) as *mut [u32; 26]), 104)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u32; 27] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u32; 27], *const u8>(&(*self) as *const [u32; 27]), 27 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u32; 27] {
    fn raw_deserialize(from: &mut Read) -> Result<[u32; 27], Error> {
        unsafe { let mut buffer: [u32; 27] = [0u32; 27];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u32; 27], *mut u8>((&mut buffer) as *mut [u32; 27]), 108)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u32; 28] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u32; 28], *const u8>(&(*self) as *const [u32; 28]), 28 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u32; 28] {
    fn raw_deserialize(from: &mut Read) -> Result<[u32; 28], Error> {
        unsafe { let mut buffer: [u32; 28] = [0u32; 28];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u32; 28], *mut u8>((&mut buffer) as *mut [u32; 28]), 112)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u32; 29] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u32; 29], *const u8>(&(*self) as *const [u32; 29]), 29 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u32; 29] {
    fn raw_deserialize(from: &mut Read) -> Result<[u32; 29], Error> {
        unsafe { let mut buffer: [u32; 29] = [0u32; 29];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u32; 29], *mut u8>((&mut buffer) as *mut [u32; 29]), 116)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u32; 30] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u32; 30], *const u8>(&(*self) as *const [u32; 30]), 30 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u32; 30] {
    fn raw_deserialize(from: &mut Read) -> Result<[u32; 30], Error> {
        unsafe { let mut buffer: [u32; 30] = [0u32; 30];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u32; 30], *mut u8>((&mut buffer) as *mut [u32; 30]), 120)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u32; 31] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u32; 31], *const u8>(&(*self) as *const [u32; 31]), 31 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u32; 31] {
    fn raw_deserialize(from: &mut Read) -> Result<[u32; 31], Error> {
        unsafe { let mut buffer: [u32; 31] = [0u32; 31];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u32; 31], *mut u8>((&mut buffer) as *mut [u32; 31]), 124)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u32; 32] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u32; 32], *const u8>(&(*self) as *const [u32; 32]), 32 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u32; 32] {
    fn raw_deserialize(from: &mut Read) -> Result<[u32; 32], Error> {
        unsafe { let mut buffer: [u32; 32] = [0u32; 32];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u32; 32], *mut u8>((&mut buffer) as *mut [u32; 32]), 128)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i32; 1] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i32; 1], *const u8>(&(*self) as *const [i32; 1]), 1 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i32; 1] {
    fn raw_deserialize(from: &mut Read) -> Result<[i32; 1], Error> {
        unsafe { let mut buffer: [i32; 1] = [0i32; 1];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i32; 1], *mut u8>((&mut buffer) as *mut [i32; 1]), 4)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i32; 2] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i32; 2], *const u8>(&(*self) as *const [i32; 2]), 2 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i32; 2] {
    fn raw_deserialize(from: &mut Read) -> Result<[i32; 2], Error> {
        unsafe { let mut buffer: [i32; 2] = [0i32; 2];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i32; 2], *mut u8>((&mut buffer) as *mut [i32; 2]), 8)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i32; 3] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i32; 3], *const u8>(&(*self) as *const [i32; 3]), 3 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i32; 3] {
    fn raw_deserialize(from: &mut Read) -> Result<[i32; 3], Error> {
        unsafe { let mut buffer: [i32; 3] = [0i32; 3];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i32; 3], *mut u8>((&mut buffer) as *mut [i32; 3]), 12)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i32; 4] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i32; 4], *const u8>(&(*self) as *const [i32; 4]), 4 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i32; 4] {
    fn raw_deserialize(from: &mut Read) -> Result<[i32; 4], Error> {
        unsafe { let mut buffer: [i32; 4] = [0i32; 4];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i32; 4], *mut u8>((&mut buffer) as *mut [i32; 4]), 16)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i32; 5] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i32; 5], *const u8>(&(*self) as *const [i32; 5]), 5 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i32; 5] {
    fn raw_deserialize(from: &mut Read) -> Result<[i32; 5], Error> {
        unsafe { let mut buffer: [i32; 5] = [0i32; 5];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i32; 5], *mut u8>((&mut buffer) as *mut [i32; 5]), 20)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i32; 6] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i32; 6], *const u8>(&(*self) as *const [i32; 6]), 6 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i32; 6] {
    fn raw_deserialize(from: &mut Read) -> Result<[i32; 6], Error> {
        unsafe { let mut buffer: [i32; 6] = [0i32; 6];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i32; 6], *mut u8>((&mut buffer) as *mut [i32; 6]), 24)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i32; 7] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i32; 7], *const u8>(&(*self) as *const [i32; 7]), 7 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i32; 7] {
    fn raw_deserialize(from: &mut Read) -> Result<[i32; 7], Error> {
        unsafe { let mut buffer: [i32; 7] = [0i32; 7];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i32; 7], *mut u8>((&mut buffer) as *mut [i32; 7]), 28)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i32; 8] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i32; 8], *const u8>(&(*self) as *const [i32; 8]), 8 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i32; 8] {
    fn raw_deserialize(from: &mut Read) -> Result<[i32; 8], Error> {
        unsafe { let mut buffer: [i32; 8] = [0i32; 8];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i32; 8], *mut u8>((&mut buffer) as *mut [i32; 8]), 32)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i32; 9] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i32; 9], *const u8>(&(*self) as *const [i32; 9]), 9 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i32; 9] {
    fn raw_deserialize(from: &mut Read) -> Result<[i32; 9], Error> {
        unsafe { let mut buffer: [i32; 9] = [0i32; 9];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i32; 9], *mut u8>((&mut buffer) as *mut [i32; 9]), 36)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i32; 10] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i32; 10], *const u8>(&(*self) as *const [i32; 10]), 10 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i32; 10] {
    fn raw_deserialize(from: &mut Read) -> Result<[i32; 10], Error> {
        unsafe { let mut buffer: [i32; 10] = [0i32; 10];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i32; 10], *mut u8>((&mut buffer) as *mut [i32; 10]), 40)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i32; 11] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i32; 11], *const u8>(&(*self) as *const [i32; 11]), 11 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i32; 11] {
    fn raw_deserialize(from: &mut Read) -> Result<[i32; 11], Error> {
        unsafe { let mut buffer: [i32; 11] = [0i32; 11];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i32; 11], *mut u8>((&mut buffer) as *mut [i32; 11]), 44)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i32; 12] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i32; 12], *const u8>(&(*self) as *const [i32; 12]), 12 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i32; 12] {
    fn raw_deserialize(from: &mut Read) -> Result<[i32; 12], Error> {
        unsafe { let mut buffer: [i32; 12] = [0i32; 12];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i32; 12], *mut u8>((&mut buffer) as *mut [i32; 12]), 48)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i32; 13] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i32; 13], *const u8>(&(*self) as *const [i32; 13]), 13 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i32; 13] {
    fn raw_deserialize(from: &mut Read) -> Result<[i32; 13], Error> {
        unsafe { let mut buffer: [i32; 13] = [0i32; 13];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i32; 13], *mut u8>((&mut buffer) as *mut [i32; 13]), 52)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i32; 14] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i32; 14], *const u8>(&(*self) as *const [i32; 14]), 14 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i32; 14] {
    fn raw_deserialize(from: &mut Read) -> Result<[i32; 14], Error> {
        unsafe { let mut buffer: [i32; 14] = [0i32; 14];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i32; 14], *mut u8>((&mut buffer) as *mut [i32; 14]), 56)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i32; 15] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i32; 15], *const u8>(&(*self) as *const [i32; 15]), 15 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i32; 15] {
    fn raw_deserialize(from: &mut Read) -> Result<[i32; 15], Error> {
        unsafe { let mut buffer: [i32; 15] = [0i32; 15];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i32; 15], *mut u8>((&mut buffer) as *mut [i32; 15]), 60)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i32; 16] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i32; 16], *const u8>(&(*self) as *const [i32; 16]), 16 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i32; 16] {
    fn raw_deserialize(from: &mut Read) -> Result<[i32; 16], Error> {
        unsafe { let mut buffer: [i32; 16] = [0i32; 16];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i32; 16], *mut u8>((&mut buffer) as *mut [i32; 16]), 64)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i32; 17] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i32; 17], *const u8>(&(*self) as *const [i32; 17]), 17 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i32; 17] {
    fn raw_deserialize(from: &mut Read) -> Result<[i32; 17], Error> {
        unsafe { let mut buffer: [i32; 17] = [0i32; 17];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i32; 17], *mut u8>((&mut buffer) as *mut [i32; 17]), 68)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i32; 18] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i32; 18], *const u8>(&(*self) as *const [i32; 18]), 18 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i32; 18] {
    fn raw_deserialize(from: &mut Read) -> Result<[i32; 18], Error> {
        unsafe { let mut buffer: [i32; 18] = [0i32; 18];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i32; 18], *mut u8>((&mut buffer) as *mut [i32; 18]), 72)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i32; 19] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i32; 19], *const u8>(&(*self) as *const [i32; 19]), 19 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i32; 19] {
    fn raw_deserialize(from: &mut Read) -> Result<[i32; 19], Error> {
        unsafe { let mut buffer: [i32; 19] = [0i32; 19];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i32; 19], *mut u8>((&mut buffer) as *mut [i32; 19]), 76)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i32; 20] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i32; 20], *const u8>(&(*self) as *const [i32; 20]), 20 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i32; 20] {
    fn raw_deserialize(from: &mut Read) -> Result<[i32; 20], Error> {
        unsafe { let mut buffer: [i32; 20] = [0i32; 20];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i32; 20], *mut u8>((&mut buffer) as *mut [i32; 20]), 80)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i32; 21] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i32; 21], *const u8>(&(*self) as *const [i32; 21]), 21 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i32; 21] {
    fn raw_deserialize(from: &mut Read) -> Result<[i32; 21], Error> {
        unsafe { let mut buffer: [i32; 21] = [0i32; 21];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i32; 21], *mut u8>((&mut buffer) as *mut [i32; 21]), 84)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i32; 22] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i32; 22], *const u8>(&(*self) as *const [i32; 22]), 22 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i32; 22] {
    fn raw_deserialize(from: &mut Read) -> Result<[i32; 22], Error> {
        unsafe { let mut buffer: [i32; 22] = [0i32; 22];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i32; 22], *mut u8>((&mut buffer) as *mut [i32; 22]), 88)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i32; 23] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i32; 23], *const u8>(&(*self) as *const [i32; 23]), 23 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i32; 23] {
    fn raw_deserialize(from: &mut Read) -> Result<[i32; 23], Error> {
        unsafe { let mut buffer: [i32; 23] = [0i32; 23];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i32; 23], *mut u8>((&mut buffer) as *mut [i32; 23]), 92)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i32; 24] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i32; 24], *const u8>(&(*self) as *const [i32; 24]), 24 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i32; 24] {
    fn raw_deserialize(from: &mut Read) -> Result<[i32; 24], Error> {
        unsafe { let mut buffer: [i32; 24] = [0i32; 24];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i32; 24], *mut u8>((&mut buffer) as *mut [i32; 24]), 96)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i32; 25] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i32; 25], *const u8>(&(*self) as *const [i32; 25]), 25 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i32; 25] {
    fn raw_deserialize(from: &mut Read) -> Result<[i32; 25], Error> {
        unsafe { let mut buffer: [i32; 25] = [0i32; 25];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i32; 25], *mut u8>((&mut buffer) as *mut [i32; 25]), 100)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i32; 26] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i32; 26], *const u8>(&(*self) as *const [i32; 26]), 26 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i32; 26] {
    fn raw_deserialize(from: &mut Read) -> Result<[i32; 26], Error> {
        unsafe { let mut buffer: [i32; 26] = [0i32; 26];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i32; 26], *mut u8>((&mut buffer) as *mut [i32; 26]), 104)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i32; 27] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i32; 27], *const u8>(&(*self) as *const [i32; 27]), 27 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i32; 27] {
    fn raw_deserialize(from: &mut Read) -> Result<[i32; 27], Error> {
        unsafe { let mut buffer: [i32; 27] = [0i32; 27];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i32; 27], *mut u8>((&mut buffer) as *mut [i32; 27]), 108)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i32; 28] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i32; 28], *const u8>(&(*self) as *const [i32; 28]), 28 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i32; 28] {
    fn raw_deserialize(from: &mut Read) -> Result<[i32; 28], Error> {
        unsafe { let mut buffer: [i32; 28] = [0i32; 28];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i32; 28], *mut u8>((&mut buffer) as *mut [i32; 28]), 112)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i32; 29] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i32; 29], *const u8>(&(*self) as *const [i32; 29]), 29 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i32; 29] {
    fn raw_deserialize(from: &mut Read) -> Result<[i32; 29], Error> {
        unsafe { let mut buffer: [i32; 29] = [0i32; 29];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i32; 29], *mut u8>((&mut buffer) as *mut [i32; 29]), 116)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i32; 30] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i32; 30], *const u8>(&(*self) as *const [i32; 30]), 30 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i32; 30] {
    fn raw_deserialize(from: &mut Read) -> Result<[i32; 30], Error> {
        unsafe { let mut buffer: [i32; 30] = [0i32; 30];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i32; 30], *mut u8>((&mut buffer) as *mut [i32; 30]), 120)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i32; 31] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i32; 31], *const u8>(&(*self) as *const [i32; 31]), 31 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i32; 31] {
    fn raw_deserialize(from: &mut Read) -> Result<[i32; 31], Error> {
        unsafe { let mut buffer: [i32; 31] = [0i32; 31];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i32; 31], *mut u8>((&mut buffer) as *mut [i32; 31]), 124)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i32; 32] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i32; 32], *const u8>(&(*self) as *const [i32; 32]), 32 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i32; 32] {
    fn raw_deserialize(from: &mut Read) -> Result<[i32; 32], Error> {
        unsafe { let mut buffer: [i32; 32] = [0i32; 32];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i32; 32], *mut u8>((&mut buffer) as *mut [i32; 32]), 128)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f32; 1] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f32; 1], *const u8>(&(*self) as *const [f32; 1]), 1 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f32; 1] {
    fn raw_deserialize(from: &mut Read) -> Result<[f32; 1], Error> {
        unsafe { let mut buffer: [f32; 1] = [0f32; 1];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f32; 1], *mut u8>((&mut buffer) as *mut [f32; 1]), 4)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f32; 2] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f32; 2], *const u8>(&(*self) as *const [f32; 2]), 2 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f32; 2] {
    fn raw_deserialize(from: &mut Read) -> Result<[f32; 2], Error> {
        unsafe { let mut buffer: [f32; 2] = [0f32; 2];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f32; 2], *mut u8>((&mut buffer) as *mut [f32; 2]), 8)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f32; 3] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f32; 3], *const u8>(&(*self) as *const [f32; 3]), 3 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f32; 3] {
    fn raw_deserialize(from: &mut Read) -> Result<[f32; 3], Error> {
        unsafe { let mut buffer: [f32; 3] = [0f32; 3];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f32; 3], *mut u8>((&mut buffer) as *mut [f32; 3]), 12)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f32; 4] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f32; 4], *const u8>(&(*self) as *const [f32; 4]), 4 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f32; 4] {
    fn raw_deserialize(from: &mut Read) -> Result<[f32; 4], Error> {
        unsafe { let mut buffer: [f32; 4] = [0f32; 4];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f32; 4], *mut u8>((&mut buffer) as *mut [f32; 4]), 16)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f32; 5] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f32; 5], *const u8>(&(*self) as *const [f32; 5]), 5 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f32; 5] {
    fn raw_deserialize(from: &mut Read) -> Result<[f32; 5], Error> {
        unsafe { let mut buffer: [f32; 5] = [0f32; 5];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f32; 5], *mut u8>((&mut buffer) as *mut [f32; 5]), 20)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f32; 6] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f32; 6], *const u8>(&(*self) as *const [f32; 6]), 6 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f32; 6] {
    fn raw_deserialize(from: &mut Read) -> Result<[f32; 6], Error> {
        unsafe { let mut buffer: [f32; 6] = [0f32; 6];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f32; 6], *mut u8>((&mut buffer) as *mut [f32; 6]), 24)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f32; 7] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f32; 7], *const u8>(&(*self) as *const [f32; 7]), 7 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f32; 7] {
    fn raw_deserialize(from: &mut Read) -> Result<[f32; 7], Error> {
        unsafe { let mut buffer: [f32; 7] = [0f32; 7];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f32; 7], *mut u8>((&mut buffer) as *mut [f32; 7]), 28)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f32; 8] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f32; 8], *const u8>(&(*self) as *const [f32; 8]), 8 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f32; 8] {
    fn raw_deserialize(from: &mut Read) -> Result<[f32; 8], Error> {
        unsafe { let mut buffer: [f32; 8] = [0f32; 8];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f32; 8], *mut u8>((&mut buffer) as *mut [f32; 8]), 32)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f32; 9] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f32; 9], *const u8>(&(*self) as *const [f32; 9]), 9 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f32; 9] {
    fn raw_deserialize(from: &mut Read) -> Result<[f32; 9], Error> {
        unsafe { let mut buffer: [f32; 9] = [0f32; 9];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f32; 9], *mut u8>((&mut buffer) as *mut [f32; 9]), 36)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f32; 10] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f32; 10], *const u8>(&(*self) as *const [f32; 10]), 10 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f32; 10] {
    fn raw_deserialize(from: &mut Read) -> Result<[f32; 10], Error> {
        unsafe { let mut buffer: [f32; 10] = [0f32; 10];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f32; 10], *mut u8>((&mut buffer) as *mut [f32; 10]), 40)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f32; 11] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f32; 11], *const u8>(&(*self) as *const [f32; 11]), 11 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f32; 11] {
    fn raw_deserialize(from: &mut Read) -> Result<[f32; 11], Error> {
        unsafe { let mut buffer: [f32; 11] = [0f32; 11];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f32; 11], *mut u8>((&mut buffer) as *mut [f32; 11]), 44)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f32; 12] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f32; 12], *const u8>(&(*self) as *const [f32; 12]), 12 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f32; 12] {
    fn raw_deserialize(from: &mut Read) -> Result<[f32; 12], Error> {
        unsafe { let mut buffer: [f32; 12] = [0f32; 12];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f32; 12], *mut u8>((&mut buffer) as *mut [f32; 12]), 48)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f32; 13] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f32; 13], *const u8>(&(*self) as *const [f32; 13]), 13 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f32; 13] {
    fn raw_deserialize(from: &mut Read) -> Result<[f32; 13], Error> {
        unsafe { let mut buffer: [f32; 13] = [0f32; 13];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f32; 13], *mut u8>((&mut buffer) as *mut [f32; 13]), 52)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f32; 14] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f32; 14], *const u8>(&(*self) as *const [f32; 14]), 14 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f32; 14] {
    fn raw_deserialize(from: &mut Read) -> Result<[f32; 14], Error> {
        unsafe { let mut buffer: [f32; 14] = [0f32; 14];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f32; 14], *mut u8>((&mut buffer) as *mut [f32; 14]), 56)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f32; 15] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f32; 15], *const u8>(&(*self) as *const [f32; 15]), 15 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f32; 15] {
    fn raw_deserialize(from: &mut Read) -> Result<[f32; 15], Error> {
        unsafe { let mut buffer: [f32; 15] = [0f32; 15];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f32; 15], *mut u8>((&mut buffer) as *mut [f32; 15]), 60)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f32; 16] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f32; 16], *const u8>(&(*self) as *const [f32; 16]), 16 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f32; 16] {
    fn raw_deserialize(from: &mut Read) -> Result<[f32; 16], Error> {
        unsafe { let mut buffer: [f32; 16] = [0f32; 16];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f32; 16], *mut u8>((&mut buffer) as *mut [f32; 16]), 64)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f32; 17] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f32; 17], *const u8>(&(*self) as *const [f32; 17]), 17 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f32; 17] {
    fn raw_deserialize(from: &mut Read) -> Result<[f32; 17], Error> {
        unsafe { let mut buffer: [f32; 17] = [0f32; 17];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f32; 17], *mut u8>((&mut buffer) as *mut [f32; 17]), 68)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f32; 18] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f32; 18], *const u8>(&(*self) as *const [f32; 18]), 18 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f32; 18] {
    fn raw_deserialize(from: &mut Read) -> Result<[f32; 18], Error> {
        unsafe { let mut buffer: [f32; 18] = [0f32; 18];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f32; 18], *mut u8>((&mut buffer) as *mut [f32; 18]), 72)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f32; 19] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f32; 19], *const u8>(&(*self) as *const [f32; 19]), 19 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f32; 19] {
    fn raw_deserialize(from: &mut Read) -> Result<[f32; 19], Error> {
        unsafe { let mut buffer: [f32; 19] = [0f32; 19];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f32; 19], *mut u8>((&mut buffer) as *mut [f32; 19]), 76)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f32; 20] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f32; 20], *const u8>(&(*self) as *const [f32; 20]), 20 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f32; 20] {
    fn raw_deserialize(from: &mut Read) -> Result<[f32; 20], Error> {
        unsafe { let mut buffer: [f32; 20] = [0f32; 20];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f32; 20], *mut u8>((&mut buffer) as *mut [f32; 20]), 80)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f32; 21] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f32; 21], *const u8>(&(*self) as *const [f32; 21]), 21 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f32; 21] {
    fn raw_deserialize(from: &mut Read) -> Result<[f32; 21], Error> {
        unsafe { let mut buffer: [f32; 21] = [0f32; 21];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f32; 21], *mut u8>((&mut buffer) as *mut [f32; 21]), 84)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f32; 22] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f32; 22], *const u8>(&(*self) as *const [f32; 22]), 22 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f32; 22] {
    fn raw_deserialize(from: &mut Read) -> Result<[f32; 22], Error> {
        unsafe { let mut buffer: [f32; 22] = [0f32; 22];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f32; 22], *mut u8>((&mut buffer) as *mut [f32; 22]), 88)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f32; 23] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f32; 23], *const u8>(&(*self) as *const [f32; 23]), 23 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f32; 23] {
    fn raw_deserialize(from: &mut Read) -> Result<[f32; 23], Error> {
        unsafe { let mut buffer: [f32; 23] = [0f32; 23];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f32; 23], *mut u8>((&mut buffer) as *mut [f32; 23]), 92)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f32; 24] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f32; 24], *const u8>(&(*self) as *const [f32; 24]), 24 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f32; 24] {
    fn raw_deserialize(from: &mut Read) -> Result<[f32; 24], Error> {
        unsafe { let mut buffer: [f32; 24] = [0f32; 24];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f32; 24], *mut u8>((&mut buffer) as *mut [f32; 24]), 96)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f32; 25] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f32; 25], *const u8>(&(*self) as *const [f32; 25]), 25 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f32; 25] {
    fn raw_deserialize(from: &mut Read) -> Result<[f32; 25], Error> {
        unsafe { let mut buffer: [f32; 25] = [0f32; 25];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f32; 25], *mut u8>((&mut buffer) as *mut [f32; 25]), 100)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f32; 26] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f32; 26], *const u8>(&(*self) as *const [f32; 26]), 26 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f32; 26] {
    fn raw_deserialize(from: &mut Read) -> Result<[f32; 26], Error> {
        unsafe { let mut buffer: [f32; 26] = [0f32; 26];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f32; 26], *mut u8>((&mut buffer) as *mut [f32; 26]), 104)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f32; 27] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f32; 27], *const u8>(&(*self) as *const [f32; 27]), 27 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f32; 27] {
    fn raw_deserialize(from: &mut Read) -> Result<[f32; 27], Error> {
        unsafe { let mut buffer: [f32; 27] = [0f32; 27];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f32; 27], *mut u8>((&mut buffer) as *mut [f32; 27]), 108)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f32; 28] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f32; 28], *const u8>(&(*self) as *const [f32; 28]), 28 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f32; 28] {
    fn raw_deserialize(from: &mut Read) -> Result<[f32; 28], Error> {
        unsafe { let mut buffer: [f32; 28] = [0f32; 28];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f32; 28], *mut u8>((&mut buffer) as *mut [f32; 28]), 112)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f32; 29] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f32; 29], *const u8>(&(*self) as *const [f32; 29]), 29 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f32; 29] {
    fn raw_deserialize(from: &mut Read) -> Result<[f32; 29], Error> {
        unsafe { let mut buffer: [f32; 29] = [0f32; 29];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f32; 29], *mut u8>((&mut buffer) as *mut [f32; 29]), 116)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f32; 30] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f32; 30], *const u8>(&(*self) as *const [f32; 30]), 30 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f32; 30] {
    fn raw_deserialize(from: &mut Read) -> Result<[f32; 30], Error> {
        unsafe { let mut buffer: [f32; 30] = [0f32; 30];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f32; 30], *mut u8>((&mut buffer) as *mut [f32; 30]), 120)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f32; 31] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f32; 31], *const u8>(&(*self) as *const [f32; 31]), 31 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f32; 31] {
    fn raw_deserialize(from: &mut Read) -> Result<[f32; 31], Error> {
        unsafe { let mut buffer: [f32; 31] = [0f32; 31];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f32; 31], *mut u8>((&mut buffer) as *mut [f32; 31]), 124)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f32; 32] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f32; 32], *const u8>(&(*self) as *const [f32; 32]), 32 * 4)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f32; 32] {
    fn raw_deserialize(from: &mut Read) -> Result<[f32; 32], Error> {
        unsafe { let mut buffer: [f32; 32] = [0f32; 32];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f32; 32], *mut u8>((&mut buffer) as *mut [f32; 32]), 128)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u64; 1] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u64; 1], *const u8>(&(*self) as *const [u64; 1]), 1 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u64; 1] {
    fn raw_deserialize(from: &mut Read) -> Result<[u64; 1], Error> {
        unsafe { let mut buffer: [u64; 1] = [0u64; 1];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u64; 1], *mut u8>((&mut buffer) as *mut [u64; 1]), 8)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u64; 2] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u64; 2], *const u8>(&(*self) as *const [u64; 2]), 2 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u64; 2] {
    fn raw_deserialize(from: &mut Read) -> Result<[u64; 2], Error> {
        unsafe { let mut buffer: [u64; 2] = [0u64; 2];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u64; 2], *mut u8>((&mut buffer) as *mut [u64; 2]), 16)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u64; 3] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u64; 3], *const u8>(&(*self) as *const [u64; 3]), 3 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u64; 3] {
    fn raw_deserialize(from: &mut Read) -> Result<[u64; 3], Error> {
        unsafe { let mut buffer: [u64; 3] = [0u64; 3];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u64; 3], *mut u8>((&mut buffer) as *mut [u64; 3]), 24)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u64; 4] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u64; 4], *const u8>(&(*self) as *const [u64; 4]), 4 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u64; 4] {
    fn raw_deserialize(from: &mut Read) -> Result<[u64; 4], Error> {
        unsafe { let mut buffer: [u64; 4] = [0u64; 4];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u64; 4], *mut u8>((&mut buffer) as *mut [u64; 4]), 32)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u64; 5] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u64; 5], *const u8>(&(*self) as *const [u64; 5]), 5 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u64; 5] {
    fn raw_deserialize(from: &mut Read) -> Result<[u64; 5], Error> {
        unsafe { let mut buffer: [u64; 5] = [0u64; 5];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u64; 5], *mut u8>((&mut buffer) as *mut [u64; 5]), 40)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u64; 6] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u64; 6], *const u8>(&(*self) as *const [u64; 6]), 6 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u64; 6] {
    fn raw_deserialize(from: &mut Read) -> Result<[u64; 6], Error> {
        unsafe { let mut buffer: [u64; 6] = [0u64; 6];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u64; 6], *mut u8>((&mut buffer) as *mut [u64; 6]), 48)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u64; 7] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u64; 7], *const u8>(&(*self) as *const [u64; 7]), 7 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u64; 7] {
    fn raw_deserialize(from: &mut Read) -> Result<[u64; 7], Error> {
        unsafe { let mut buffer: [u64; 7] = [0u64; 7];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u64; 7], *mut u8>((&mut buffer) as *mut [u64; 7]), 56)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u64; 8] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u64; 8], *const u8>(&(*self) as *const [u64; 8]), 8 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u64; 8] {
    fn raw_deserialize(from: &mut Read) -> Result<[u64; 8], Error> {
        unsafe { let mut buffer: [u64; 8] = [0u64; 8];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u64; 8], *mut u8>((&mut buffer) as *mut [u64; 8]), 64)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u64; 9] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u64; 9], *const u8>(&(*self) as *const [u64; 9]), 9 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u64; 9] {
    fn raw_deserialize(from: &mut Read) -> Result<[u64; 9], Error> {
        unsafe { let mut buffer: [u64; 9] = [0u64; 9];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u64; 9], *mut u8>((&mut buffer) as *mut [u64; 9]), 72)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u64; 10] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u64; 10], *const u8>(&(*self) as *const [u64; 10]), 10 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u64; 10] {
    fn raw_deserialize(from: &mut Read) -> Result<[u64; 10], Error> {
        unsafe { let mut buffer: [u64; 10] = [0u64; 10];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u64; 10], *mut u8>((&mut buffer) as *mut [u64; 10]), 80)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u64; 11] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u64; 11], *const u8>(&(*self) as *const [u64; 11]), 11 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u64; 11] {
    fn raw_deserialize(from: &mut Read) -> Result<[u64; 11], Error> {
        unsafe { let mut buffer: [u64; 11] = [0u64; 11];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u64; 11], *mut u8>((&mut buffer) as *mut [u64; 11]), 88)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u64; 12] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u64; 12], *const u8>(&(*self) as *const [u64; 12]), 12 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u64; 12] {
    fn raw_deserialize(from: &mut Read) -> Result<[u64; 12], Error> {
        unsafe { let mut buffer: [u64; 12] = [0u64; 12];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u64; 12], *mut u8>((&mut buffer) as *mut [u64; 12]), 96)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u64; 13] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u64; 13], *const u8>(&(*self) as *const [u64; 13]), 13 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u64; 13] {
    fn raw_deserialize(from: &mut Read) -> Result<[u64; 13], Error> {
        unsafe { let mut buffer: [u64; 13] = [0u64; 13];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u64; 13], *mut u8>((&mut buffer) as *mut [u64; 13]), 104)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u64; 14] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u64; 14], *const u8>(&(*self) as *const [u64; 14]), 14 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u64; 14] {
    fn raw_deserialize(from: &mut Read) -> Result<[u64; 14], Error> {
        unsafe { let mut buffer: [u64; 14] = [0u64; 14];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u64; 14], *mut u8>((&mut buffer) as *mut [u64; 14]), 112)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u64; 15] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u64; 15], *const u8>(&(*self) as *const [u64; 15]), 15 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u64; 15] {
    fn raw_deserialize(from: &mut Read) -> Result<[u64; 15], Error> {
        unsafe { let mut buffer: [u64; 15] = [0u64; 15];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u64; 15], *mut u8>((&mut buffer) as *mut [u64; 15]), 120)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u64; 16] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u64; 16], *const u8>(&(*self) as *const [u64; 16]), 16 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u64; 16] {
    fn raw_deserialize(from: &mut Read) -> Result<[u64; 16], Error> {
        unsafe { let mut buffer: [u64; 16] = [0u64; 16];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u64; 16], *mut u8>((&mut buffer) as *mut [u64; 16]), 128)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u64; 17] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u64; 17], *const u8>(&(*self) as *const [u64; 17]), 17 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u64; 17] {
    fn raw_deserialize(from: &mut Read) -> Result<[u64; 17], Error> {
        unsafe { let mut buffer: [u64; 17] = [0u64; 17];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u64; 17], *mut u8>((&mut buffer) as *mut [u64; 17]), 136)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u64; 18] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u64; 18], *const u8>(&(*self) as *const [u64; 18]), 18 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u64; 18] {
    fn raw_deserialize(from: &mut Read) -> Result<[u64; 18], Error> {
        unsafe { let mut buffer: [u64; 18] = [0u64; 18];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u64; 18], *mut u8>((&mut buffer) as *mut [u64; 18]), 144)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u64; 19] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u64; 19], *const u8>(&(*self) as *const [u64; 19]), 19 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u64; 19] {
    fn raw_deserialize(from: &mut Read) -> Result<[u64; 19], Error> {
        unsafe { let mut buffer: [u64; 19] = [0u64; 19];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u64; 19], *mut u8>((&mut buffer) as *mut [u64; 19]), 152)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u64; 20] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u64; 20], *const u8>(&(*self) as *const [u64; 20]), 20 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u64; 20] {
    fn raw_deserialize(from: &mut Read) -> Result<[u64; 20], Error> {
        unsafe { let mut buffer: [u64; 20] = [0u64; 20];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u64; 20], *mut u8>((&mut buffer) as *mut [u64; 20]), 160)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u64; 21] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u64; 21], *const u8>(&(*self) as *const [u64; 21]), 21 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u64; 21] {
    fn raw_deserialize(from: &mut Read) -> Result<[u64; 21], Error> {
        unsafe { let mut buffer: [u64; 21] = [0u64; 21];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u64; 21], *mut u8>((&mut buffer) as *mut [u64; 21]), 168)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u64; 22] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u64; 22], *const u8>(&(*self) as *const [u64; 22]), 22 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u64; 22] {
    fn raw_deserialize(from: &mut Read) -> Result<[u64; 22], Error> {
        unsafe { let mut buffer: [u64; 22] = [0u64; 22];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u64; 22], *mut u8>((&mut buffer) as *mut [u64; 22]), 176)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u64; 23] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u64; 23], *const u8>(&(*self) as *const [u64; 23]), 23 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u64; 23] {
    fn raw_deserialize(from: &mut Read) -> Result<[u64; 23], Error> {
        unsafe { let mut buffer: [u64; 23] = [0u64; 23];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u64; 23], *mut u8>((&mut buffer) as *mut [u64; 23]), 184)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u64; 24] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u64; 24], *const u8>(&(*self) as *const [u64; 24]), 24 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u64; 24] {
    fn raw_deserialize(from: &mut Read) -> Result<[u64; 24], Error> {
        unsafe { let mut buffer: [u64; 24] = [0u64; 24];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u64; 24], *mut u8>((&mut buffer) as *mut [u64; 24]), 192)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u64; 25] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u64; 25], *const u8>(&(*self) as *const [u64; 25]), 25 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u64; 25] {
    fn raw_deserialize(from: &mut Read) -> Result<[u64; 25], Error> {
        unsafe { let mut buffer: [u64; 25] = [0u64; 25];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u64; 25], *mut u8>((&mut buffer) as *mut [u64; 25]), 200)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u64; 26] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u64; 26], *const u8>(&(*self) as *const [u64; 26]), 26 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u64; 26] {
    fn raw_deserialize(from: &mut Read) -> Result<[u64; 26], Error> {
        unsafe { let mut buffer: [u64; 26] = [0u64; 26];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u64; 26], *mut u8>((&mut buffer) as *mut [u64; 26]), 208)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u64; 27] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u64; 27], *const u8>(&(*self) as *const [u64; 27]), 27 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u64; 27] {
    fn raw_deserialize(from: &mut Read) -> Result<[u64; 27], Error> {
        unsafe { let mut buffer: [u64; 27] = [0u64; 27];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u64; 27], *mut u8>((&mut buffer) as *mut [u64; 27]), 216)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u64; 28] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u64; 28], *const u8>(&(*self) as *const [u64; 28]), 28 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u64; 28] {
    fn raw_deserialize(from: &mut Read) -> Result<[u64; 28], Error> {
        unsafe { let mut buffer: [u64; 28] = [0u64; 28];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u64; 28], *mut u8>((&mut buffer) as *mut [u64; 28]), 224)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u64; 29] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u64; 29], *const u8>(&(*self) as *const [u64; 29]), 29 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u64; 29] {
    fn raw_deserialize(from: &mut Read) -> Result<[u64; 29], Error> {
        unsafe { let mut buffer: [u64; 29] = [0u64; 29];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u64; 29], *mut u8>((&mut buffer) as *mut [u64; 29]), 232)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u64; 30] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u64; 30], *const u8>(&(*self) as *const [u64; 30]), 30 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u64; 30] {
    fn raw_deserialize(from: &mut Read) -> Result<[u64; 30], Error> {
        unsafe { let mut buffer: [u64; 30] = [0u64; 30];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u64; 30], *mut u8>((&mut buffer) as *mut [u64; 30]), 240)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u64; 31] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u64; 31], *const u8>(&(*self) as *const [u64; 31]), 31 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u64; 31] {
    fn raw_deserialize(from: &mut Read) -> Result<[u64; 31], Error> {
        unsafe { let mut buffer: [u64; 31] = [0u64; 31];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u64; 31], *mut u8>((&mut buffer) as *mut [u64; 31]), 248)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u64; 32] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u64; 32], *const u8>(&(*self) as *const [u64; 32]), 32 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u64; 32] {
    fn raw_deserialize(from: &mut Read) -> Result<[u64; 32], Error> {
        unsafe { let mut buffer: [u64; 32] = [0u64; 32];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u64; 32], *mut u8>((&mut buffer) as *mut [u64; 32]), 256)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i64; 1] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i64; 1], *const u8>(&(*self) as *const [i64; 1]), 1 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i64; 1] {
    fn raw_deserialize(from: &mut Read) -> Result<[i64; 1], Error> {
        unsafe { let mut buffer: [i64; 1] = [0i64; 1];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i64; 1], *mut u8>((&mut buffer) as *mut [i64; 1]), 8)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i64; 2] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i64; 2], *const u8>(&(*self) as *const [i64; 2]), 2 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i64; 2] {
    fn raw_deserialize(from: &mut Read) -> Result<[i64; 2], Error> {
        unsafe { let mut buffer: [i64; 2] = [0i64; 2];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i64; 2], *mut u8>((&mut buffer) as *mut [i64; 2]), 16)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i64; 3] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i64; 3], *const u8>(&(*self) as *const [i64; 3]), 3 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i64; 3] {
    fn raw_deserialize(from: &mut Read) -> Result<[i64; 3], Error> {
        unsafe { let mut buffer: [i64; 3] = [0i64; 3];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i64; 3], *mut u8>((&mut buffer) as *mut [i64; 3]), 24)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i64; 4] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i64; 4], *const u8>(&(*self) as *const [i64; 4]), 4 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i64; 4] {
    fn raw_deserialize(from: &mut Read) -> Result<[i64; 4], Error> {
        unsafe { let mut buffer: [i64; 4] = [0i64; 4];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i64; 4], *mut u8>((&mut buffer) as *mut [i64; 4]), 32)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i64; 5] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i64; 5], *const u8>(&(*self) as *const [i64; 5]), 5 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i64; 5] {
    fn raw_deserialize(from: &mut Read) -> Result<[i64; 5], Error> {
        unsafe { let mut buffer: [i64; 5] = [0i64; 5];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i64; 5], *mut u8>((&mut buffer) as *mut [i64; 5]), 40)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i64; 6] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i64; 6], *const u8>(&(*self) as *const [i64; 6]), 6 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i64; 6] {
    fn raw_deserialize(from: &mut Read) -> Result<[i64; 6], Error> {
        unsafe { let mut buffer: [i64; 6] = [0i64; 6];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i64; 6], *mut u8>((&mut buffer) as *mut [i64; 6]), 48)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i64; 7] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i64; 7], *const u8>(&(*self) as *const [i64; 7]), 7 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i64; 7] {
    fn raw_deserialize(from: &mut Read) -> Result<[i64; 7], Error> {
        unsafe { let mut buffer: [i64; 7] = [0i64; 7];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i64; 7], *mut u8>((&mut buffer) as *mut [i64; 7]), 56)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i64; 8] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i64; 8], *const u8>(&(*self) as *const [i64; 8]), 8 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i64; 8] {
    fn raw_deserialize(from: &mut Read) -> Result<[i64; 8], Error> {
        unsafe { let mut buffer: [i64; 8] = [0i64; 8];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i64; 8], *mut u8>((&mut buffer) as *mut [i64; 8]), 64)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i64; 9] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i64; 9], *const u8>(&(*self) as *const [i64; 9]), 9 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i64; 9] {
    fn raw_deserialize(from: &mut Read) -> Result<[i64; 9], Error> {
        unsafe { let mut buffer: [i64; 9] = [0i64; 9];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i64; 9], *mut u8>((&mut buffer) as *mut [i64; 9]), 72)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i64; 10] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i64; 10], *const u8>(&(*self) as *const [i64; 10]), 10 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i64; 10] {
    fn raw_deserialize(from: &mut Read) -> Result<[i64; 10], Error> {
        unsafe { let mut buffer: [i64; 10] = [0i64; 10];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i64; 10], *mut u8>((&mut buffer) as *mut [i64; 10]), 80)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i64; 11] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i64; 11], *const u8>(&(*self) as *const [i64; 11]), 11 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i64; 11] {
    fn raw_deserialize(from: &mut Read) -> Result<[i64; 11], Error> {
        unsafe { let mut buffer: [i64; 11] = [0i64; 11];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i64; 11], *mut u8>((&mut buffer) as *mut [i64; 11]), 88)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i64; 12] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i64; 12], *const u8>(&(*self) as *const [i64; 12]), 12 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i64; 12] {
    fn raw_deserialize(from: &mut Read) -> Result<[i64; 12], Error> {
        unsafe { let mut buffer: [i64; 12] = [0i64; 12];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i64; 12], *mut u8>((&mut buffer) as *mut [i64; 12]), 96)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i64; 13] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i64; 13], *const u8>(&(*self) as *const [i64; 13]), 13 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i64; 13] {
    fn raw_deserialize(from: &mut Read) -> Result<[i64; 13], Error> {
        unsafe { let mut buffer: [i64; 13] = [0i64; 13];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i64; 13], *mut u8>((&mut buffer) as *mut [i64; 13]), 104)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i64; 14] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i64; 14], *const u8>(&(*self) as *const [i64; 14]), 14 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i64; 14] {
    fn raw_deserialize(from: &mut Read) -> Result<[i64; 14], Error> {
        unsafe { let mut buffer: [i64; 14] = [0i64; 14];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i64; 14], *mut u8>((&mut buffer) as *mut [i64; 14]), 112)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i64; 15] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i64; 15], *const u8>(&(*self) as *const [i64; 15]), 15 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i64; 15] {
    fn raw_deserialize(from: &mut Read) -> Result<[i64; 15], Error> {
        unsafe { let mut buffer: [i64; 15] = [0i64; 15];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i64; 15], *mut u8>((&mut buffer) as *mut [i64; 15]), 120)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i64; 16] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i64; 16], *const u8>(&(*self) as *const [i64; 16]), 16 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i64; 16] {
    fn raw_deserialize(from: &mut Read) -> Result<[i64; 16], Error> {
        unsafe { let mut buffer: [i64; 16] = [0i64; 16];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i64; 16], *mut u8>((&mut buffer) as *mut [i64; 16]), 128)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i64; 17] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i64; 17], *const u8>(&(*self) as *const [i64; 17]), 17 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i64; 17] {
    fn raw_deserialize(from: &mut Read) -> Result<[i64; 17], Error> {
        unsafe { let mut buffer: [i64; 17] = [0i64; 17];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i64; 17], *mut u8>((&mut buffer) as *mut [i64; 17]), 136)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i64; 18] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i64; 18], *const u8>(&(*self) as *const [i64; 18]), 18 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i64; 18] {
    fn raw_deserialize(from: &mut Read) -> Result<[i64; 18], Error> {
        unsafe { let mut buffer: [i64; 18] = [0i64; 18];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i64; 18], *mut u8>((&mut buffer) as *mut [i64; 18]), 144)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i64; 19] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i64; 19], *const u8>(&(*self) as *const [i64; 19]), 19 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i64; 19] {
    fn raw_deserialize(from: &mut Read) -> Result<[i64; 19], Error> {
        unsafe { let mut buffer: [i64; 19] = [0i64; 19];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i64; 19], *mut u8>((&mut buffer) as *mut [i64; 19]), 152)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i64; 20] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i64; 20], *const u8>(&(*self) as *const [i64; 20]), 20 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i64; 20] {
    fn raw_deserialize(from: &mut Read) -> Result<[i64; 20], Error> {
        unsafe { let mut buffer: [i64; 20] = [0i64; 20];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i64; 20], *mut u8>((&mut buffer) as *mut [i64; 20]), 160)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i64; 21] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i64; 21], *const u8>(&(*self) as *const [i64; 21]), 21 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i64; 21] {
    fn raw_deserialize(from: &mut Read) -> Result<[i64; 21], Error> {
        unsafe { let mut buffer: [i64; 21] = [0i64; 21];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i64; 21], *mut u8>((&mut buffer) as *mut [i64; 21]), 168)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i64; 22] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i64; 22], *const u8>(&(*self) as *const [i64; 22]), 22 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i64; 22] {
    fn raw_deserialize(from: &mut Read) -> Result<[i64; 22], Error> {
        unsafe { let mut buffer: [i64; 22] = [0i64; 22];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i64; 22], *mut u8>((&mut buffer) as *mut [i64; 22]), 176)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i64; 23] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i64; 23], *const u8>(&(*self) as *const [i64; 23]), 23 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i64; 23] {
    fn raw_deserialize(from: &mut Read) -> Result<[i64; 23], Error> {
        unsafe { let mut buffer: [i64; 23] = [0i64; 23];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i64; 23], *mut u8>((&mut buffer) as *mut [i64; 23]), 184)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i64; 24] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i64; 24], *const u8>(&(*self) as *const [i64; 24]), 24 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i64; 24] {
    fn raw_deserialize(from: &mut Read) -> Result<[i64; 24], Error> {
        unsafe { let mut buffer: [i64; 24] = [0i64; 24];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i64; 24], *mut u8>((&mut buffer) as *mut [i64; 24]), 192)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i64; 25] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i64; 25], *const u8>(&(*self) as *const [i64; 25]), 25 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i64; 25] {
    fn raw_deserialize(from: &mut Read) -> Result<[i64; 25], Error> {
        unsafe { let mut buffer: [i64; 25] = [0i64; 25];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i64; 25], *mut u8>((&mut buffer) as *mut [i64; 25]), 200)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i64; 26] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i64; 26], *const u8>(&(*self) as *const [i64; 26]), 26 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i64; 26] {
    fn raw_deserialize(from: &mut Read) -> Result<[i64; 26], Error> {
        unsafe { let mut buffer: [i64; 26] = [0i64; 26];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i64; 26], *mut u8>((&mut buffer) as *mut [i64; 26]), 208)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i64; 27] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i64; 27], *const u8>(&(*self) as *const [i64; 27]), 27 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i64; 27] {
    fn raw_deserialize(from: &mut Read) -> Result<[i64; 27], Error> {
        unsafe { let mut buffer: [i64; 27] = [0i64; 27];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i64; 27], *mut u8>((&mut buffer) as *mut [i64; 27]), 216)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i64; 28] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i64; 28], *const u8>(&(*self) as *const [i64; 28]), 28 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i64; 28] {
    fn raw_deserialize(from: &mut Read) -> Result<[i64; 28], Error> {
        unsafe { let mut buffer: [i64; 28] = [0i64; 28];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i64; 28], *mut u8>((&mut buffer) as *mut [i64; 28]), 224)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i64; 29] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i64; 29], *const u8>(&(*self) as *const [i64; 29]), 29 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i64; 29] {
    fn raw_deserialize(from: &mut Read) -> Result<[i64; 29], Error> {
        unsafe { let mut buffer: [i64; 29] = [0i64; 29];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i64; 29], *mut u8>((&mut buffer) as *mut [i64; 29]), 232)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i64; 30] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i64; 30], *const u8>(&(*self) as *const [i64; 30]), 30 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i64; 30] {
    fn raw_deserialize(from: &mut Read) -> Result<[i64; 30], Error> {
        unsafe { let mut buffer: [i64; 30] = [0i64; 30];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i64; 30], *mut u8>((&mut buffer) as *mut [i64; 30]), 240)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i64; 31] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i64; 31], *const u8>(&(*self) as *const [i64; 31]), 31 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i64; 31] {
    fn raw_deserialize(from: &mut Read) -> Result<[i64; 31], Error> {
        unsafe { let mut buffer: [i64; 31] = [0i64; 31];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i64; 31], *mut u8>((&mut buffer) as *mut [i64; 31]), 248)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i64; 32] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i64; 32], *const u8>(&(*self) as *const [i64; 32]), 32 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i64; 32] {
    fn raw_deserialize(from: &mut Read) -> Result<[i64; 32], Error> {
        unsafe { let mut buffer: [i64; 32] = [0i64; 32];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i64; 32], *mut u8>((&mut buffer) as *mut [i64; 32]), 256)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f64; 1] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f64; 1], *const u8>(&(*self) as *const [f64; 1]), 1 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f64; 1] {
    fn raw_deserialize(from: &mut Read) -> Result<[f64; 1], Error> {
        unsafe { let mut buffer: [f64; 1] = [0f64; 1];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f64; 1], *mut u8>((&mut buffer) as *mut [f64; 1]), 8)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f64; 2] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f64; 2], *const u8>(&(*self) as *const [f64; 2]), 2 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f64; 2] {
    fn raw_deserialize(from: &mut Read) -> Result<[f64; 2], Error> {
        unsafe { let mut buffer: [f64; 2] = [0f64; 2];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f64; 2], *mut u8>((&mut buffer) as *mut [f64; 2]), 16)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f64; 3] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f64; 3], *const u8>(&(*self) as *const [f64; 3]), 3 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f64; 3] {
    fn raw_deserialize(from: &mut Read) -> Result<[f64; 3], Error> {
        unsafe { let mut buffer: [f64; 3] = [0f64; 3];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f64; 3], *mut u8>((&mut buffer) as *mut [f64; 3]), 24)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f64; 4] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f64; 4], *const u8>(&(*self) as *const [f64; 4]), 4 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f64; 4] {
    fn raw_deserialize(from: &mut Read) -> Result<[f64; 4], Error> {
        unsafe { let mut buffer: [f64; 4] = [0f64; 4];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f64; 4], *mut u8>((&mut buffer) as *mut [f64; 4]), 32)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f64; 5] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f64; 5], *const u8>(&(*self) as *const [f64; 5]), 5 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f64; 5] {
    fn raw_deserialize(from: &mut Read) -> Result<[f64; 5], Error> {
        unsafe { let mut buffer: [f64; 5] = [0f64; 5];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f64; 5], *mut u8>((&mut buffer) as *mut [f64; 5]), 40)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f64; 6] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f64; 6], *const u8>(&(*self) as *const [f64; 6]), 6 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f64; 6] {
    fn raw_deserialize(from: &mut Read) -> Result<[f64; 6], Error> {
        unsafe { let mut buffer: [f64; 6] = [0f64; 6];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f64; 6], *mut u8>((&mut buffer) as *mut [f64; 6]), 48)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f64; 7] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f64; 7], *const u8>(&(*self) as *const [f64; 7]), 7 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f64; 7] {
    fn raw_deserialize(from: &mut Read) -> Result<[f64; 7], Error> {
        unsafe { let mut buffer: [f64; 7] = [0f64; 7];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f64; 7], *mut u8>((&mut buffer) as *mut [f64; 7]), 56)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f64; 8] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f64; 8], *const u8>(&(*self) as *const [f64; 8]), 8 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f64; 8] {
    fn raw_deserialize(from: &mut Read) -> Result<[f64; 8], Error> {
        unsafe { let mut buffer: [f64; 8] = [0f64; 8];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f64; 8], *mut u8>((&mut buffer) as *mut [f64; 8]), 64)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f64; 9] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f64; 9], *const u8>(&(*self) as *const [f64; 9]), 9 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f64; 9] {
    fn raw_deserialize(from: &mut Read) -> Result<[f64; 9], Error> {
        unsafe { let mut buffer: [f64; 9] = [0f64; 9];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f64; 9], *mut u8>((&mut buffer) as *mut [f64; 9]), 72)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f64; 10] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f64; 10], *const u8>(&(*self) as *const [f64; 10]), 10 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f64; 10] {
    fn raw_deserialize(from: &mut Read) -> Result<[f64; 10], Error> {
        unsafe { let mut buffer: [f64; 10] = [0f64; 10];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f64; 10], *mut u8>((&mut buffer) as *mut [f64; 10]), 80)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f64; 11] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f64; 11], *const u8>(&(*self) as *const [f64; 11]), 11 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f64; 11] {
    fn raw_deserialize(from: &mut Read) -> Result<[f64; 11], Error> {
        unsafe { let mut buffer: [f64; 11] = [0f64; 11];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f64; 11], *mut u8>((&mut buffer) as *mut [f64; 11]), 88)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f64; 12] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f64; 12], *const u8>(&(*self) as *const [f64; 12]), 12 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f64; 12] {
    fn raw_deserialize(from: &mut Read) -> Result<[f64; 12], Error> {
        unsafe { let mut buffer: [f64; 12] = [0f64; 12];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f64; 12], *mut u8>((&mut buffer) as *mut [f64; 12]), 96)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f64; 13] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f64; 13], *const u8>(&(*self) as *const [f64; 13]), 13 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f64; 13] {
    fn raw_deserialize(from: &mut Read) -> Result<[f64; 13], Error> {
        unsafe { let mut buffer: [f64; 13] = [0f64; 13];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f64; 13], *mut u8>((&mut buffer) as *mut [f64; 13]), 104)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f64; 14] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f64; 14], *const u8>(&(*self) as *const [f64; 14]), 14 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f64; 14] {
    fn raw_deserialize(from: &mut Read) -> Result<[f64; 14], Error> {
        unsafe { let mut buffer: [f64; 14] = [0f64; 14];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f64; 14], *mut u8>((&mut buffer) as *mut [f64; 14]), 112)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f64; 15] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f64; 15], *const u8>(&(*self) as *const [f64; 15]), 15 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f64; 15] {
    fn raw_deserialize(from: &mut Read) -> Result<[f64; 15], Error> {
        unsafe { let mut buffer: [f64; 15] = [0f64; 15];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f64; 15], *mut u8>((&mut buffer) as *mut [f64; 15]), 120)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f64; 16] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f64; 16], *const u8>(&(*self) as *const [f64; 16]), 16 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f64; 16] {
    fn raw_deserialize(from: &mut Read) -> Result<[f64; 16], Error> {
        unsafe { let mut buffer: [f64; 16] = [0f64; 16];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f64; 16], *mut u8>((&mut buffer) as *mut [f64; 16]), 128)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f64; 17] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f64; 17], *const u8>(&(*self) as *const [f64; 17]), 17 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f64; 17] {
    fn raw_deserialize(from: &mut Read) -> Result<[f64; 17], Error> {
        unsafe { let mut buffer: [f64; 17] = [0f64; 17];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f64; 17], *mut u8>((&mut buffer) as *mut [f64; 17]), 136)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f64; 18] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f64; 18], *const u8>(&(*self) as *const [f64; 18]), 18 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f64; 18] {
    fn raw_deserialize(from: &mut Read) -> Result<[f64; 18], Error> {
        unsafe { let mut buffer: [f64; 18] = [0f64; 18];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f64; 18], *mut u8>((&mut buffer) as *mut [f64; 18]), 144)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f64; 19] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f64; 19], *const u8>(&(*self) as *const [f64; 19]), 19 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f64; 19] {
    fn raw_deserialize(from: &mut Read) -> Result<[f64; 19], Error> {
        unsafe { let mut buffer: [f64; 19] = [0f64; 19];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f64; 19], *mut u8>((&mut buffer) as *mut [f64; 19]), 152)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f64; 20] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f64; 20], *const u8>(&(*self) as *const [f64; 20]), 20 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f64; 20] {
    fn raw_deserialize(from: &mut Read) -> Result<[f64; 20], Error> {
        unsafe { let mut buffer: [f64; 20] = [0f64; 20];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f64; 20], *mut u8>((&mut buffer) as *mut [f64; 20]), 160)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f64; 21] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f64; 21], *const u8>(&(*self) as *const [f64; 21]), 21 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f64; 21] {
    fn raw_deserialize(from: &mut Read) -> Result<[f64; 21], Error> {
        unsafe { let mut buffer: [f64; 21] = [0f64; 21];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f64; 21], *mut u8>((&mut buffer) as *mut [f64; 21]), 168)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f64; 22] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f64; 22], *const u8>(&(*self) as *const [f64; 22]), 22 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f64; 22] {
    fn raw_deserialize(from: &mut Read) -> Result<[f64; 22], Error> {
        unsafe { let mut buffer: [f64; 22] = [0f64; 22];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f64; 22], *mut u8>((&mut buffer) as *mut [f64; 22]), 176)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f64; 23] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f64; 23], *const u8>(&(*self) as *const [f64; 23]), 23 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f64; 23] {
    fn raw_deserialize(from: &mut Read) -> Result<[f64; 23], Error> {
        unsafe { let mut buffer: [f64; 23] = [0f64; 23];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f64; 23], *mut u8>((&mut buffer) as *mut [f64; 23]), 184)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f64; 24] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f64; 24], *const u8>(&(*self) as *const [f64; 24]), 24 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f64; 24] {
    fn raw_deserialize(from: &mut Read) -> Result<[f64; 24], Error> {
        unsafe { let mut buffer: [f64; 24] = [0f64; 24];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f64; 24], *mut u8>((&mut buffer) as *mut [f64; 24]), 192)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f64; 25] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f64; 25], *const u8>(&(*self) as *const [f64; 25]), 25 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f64; 25] {
    fn raw_deserialize(from: &mut Read) -> Result<[f64; 25], Error> {
        unsafe { let mut buffer: [f64; 25] = [0f64; 25];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f64; 25], *mut u8>((&mut buffer) as *mut [f64; 25]), 200)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f64; 26] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f64; 26], *const u8>(&(*self) as *const [f64; 26]), 26 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f64; 26] {
    fn raw_deserialize(from: &mut Read) -> Result<[f64; 26], Error> {
        unsafe { let mut buffer: [f64; 26] = [0f64; 26];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f64; 26], *mut u8>((&mut buffer) as *mut [f64; 26]), 208)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f64; 27] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f64; 27], *const u8>(&(*self) as *const [f64; 27]), 27 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f64; 27] {
    fn raw_deserialize(from: &mut Read) -> Result<[f64; 27], Error> {
        unsafe { let mut buffer: [f64; 27] = [0f64; 27];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f64; 27], *mut u8>((&mut buffer) as *mut [f64; 27]), 216)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f64; 28] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f64; 28], *const u8>(&(*self) as *const [f64; 28]), 28 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f64; 28] {
    fn raw_deserialize(from: &mut Read) -> Result<[f64; 28], Error> {
        unsafe { let mut buffer: [f64; 28] = [0f64; 28];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f64; 28], *mut u8>((&mut buffer) as *mut [f64; 28]), 224)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f64; 29] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f64; 29], *const u8>(&(*self) as *const [f64; 29]), 29 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f64; 29] {
    fn raw_deserialize(from: &mut Read) -> Result<[f64; 29], Error> {
        unsafe { let mut buffer: [f64; 29] = [0f64; 29];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f64; 29], *mut u8>((&mut buffer) as *mut [f64; 29]), 232)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f64; 30] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f64; 30], *const u8>(&(*self) as *const [f64; 30]), 30 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f64; 30] {
    fn raw_deserialize(from: &mut Read) -> Result<[f64; 30], Error> {
        unsafe { let mut buffer: [f64; 30] = [0f64; 30];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f64; 30], *mut u8>((&mut buffer) as *mut [f64; 30]), 240)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f64; 31] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f64; 31], *const u8>(&(*self) as *const [f64; 31]), 31 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f64; 31] {
    fn raw_deserialize(from: &mut Read) -> Result<[f64; 31], Error> {
        unsafe { let mut buffer: [f64; 31] = [0f64; 31];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f64; 31], *mut u8>((&mut buffer) as *mut [f64; 31]), 248)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [f64; 32] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ f64; 32], *const u8>(&(*self) as *const [f64; 32]), 32 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [f64; 32] {
    fn raw_deserialize(from: &mut Read) -> Result<[f64; 32], Error> {
        unsafe { let mut buffer: [f64; 32] = [0f64; 32];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ f64; 32], *mut u8>((&mut buffer) as *mut [f64; 32]), 256)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i128; 1] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i128; 1], *const u8>(&(*self) as *const [i128; 1]), 1 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i128; 1] {
    fn raw_deserialize(from: &mut Read) -> Result<[i128; 1], Error> {
        unsafe { let mut buffer: [i128; 1] = [0i128; 1];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i128; 1], *mut u8>((&mut buffer) as *mut [i128; 1]), 8)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i128; 2] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i128; 2], *const u8>(&(*self) as *const [i128; 2]), 2 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i128; 2] {
    fn raw_deserialize(from: &mut Read) -> Result<[i128; 2], Error> {
        unsafe { let mut buffer: [i128; 2] = [0i128; 2];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i128; 2], *mut u8>((&mut buffer) as *mut [i128; 2]), 16)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i128; 3] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i128; 3], *const u8>(&(*self) as *const [i128; 3]), 3 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i128; 3] {
    fn raw_deserialize(from: &mut Read) -> Result<[i128; 3], Error> {
        unsafe { let mut buffer: [i128; 3] = [0i128; 3];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i128; 3], *mut u8>((&mut buffer) as *mut [i128; 3]), 24)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i128; 4] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i128; 4], *const u8>(&(*self) as *const [i128; 4]), 4 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i128; 4] {
    fn raw_deserialize(from: &mut Read) -> Result<[i128; 4], Error> {
        unsafe { let mut buffer: [i128; 4] = [0i128; 4];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i128; 4], *mut u8>((&mut buffer) as *mut [i128; 4]), 32)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i128; 5] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i128; 5], *const u8>(&(*self) as *const [i128; 5]), 5 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i128; 5] {
    fn raw_deserialize(from: &mut Read) -> Result<[i128; 5], Error> {
        unsafe { let mut buffer: [i128; 5] = [0i128; 5];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i128; 5], *mut u8>((&mut buffer) as *mut [i128; 5]), 40)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i128; 6] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i128; 6], *const u8>(&(*self) as *const [i128; 6]), 6 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i128; 6] {
    fn raw_deserialize(from: &mut Read) -> Result<[i128; 6], Error> {
        unsafe { let mut buffer: [i128; 6] = [0i128; 6];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i128; 6], *mut u8>((&mut buffer) as *mut [i128; 6]), 48)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i128; 7] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i128; 7], *const u8>(&(*self) as *const [i128; 7]), 7 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i128; 7] {
    fn raw_deserialize(from: &mut Read) -> Result<[i128; 7], Error> {
        unsafe { let mut buffer: [i128; 7] = [0i128; 7];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i128; 7], *mut u8>((&mut buffer) as *mut [i128; 7]), 56)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i128; 8] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i128; 8], *const u8>(&(*self) as *const [i128; 8]), 8 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i128; 8] {
    fn raw_deserialize(from: &mut Read) -> Result<[i128; 8], Error> {
        unsafe { let mut buffer: [i128; 8] = [0i128; 8];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i128; 8], *mut u8>((&mut buffer) as *mut [i128; 8]), 64)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i128; 9] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i128; 9], *const u8>(&(*self) as *const [i128; 9]), 9 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i128; 9] {
    fn raw_deserialize(from: &mut Read) -> Result<[i128; 9], Error> {
        unsafe { let mut buffer: [i128; 9] = [0i128; 9];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i128; 9], *mut u8>((&mut buffer) as *mut [i128; 9]), 72)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i128; 10] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i128; 10], *const u8>(&(*self) as *const [i128; 10]), 10 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i128; 10] {
    fn raw_deserialize(from: &mut Read) -> Result<[i128; 10], Error> {
        unsafe { let mut buffer: [i128; 10] = [0i128; 10];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i128; 10], *mut u8>((&mut buffer) as *mut [i128; 10]), 80)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i128; 11] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i128; 11], *const u8>(&(*self) as *const [i128; 11]), 11 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i128; 11] {
    fn raw_deserialize(from: &mut Read) -> Result<[i128; 11], Error> {
        unsafe { let mut buffer: [i128; 11] = [0i128; 11];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i128; 11], *mut u8>((&mut buffer) as *mut [i128; 11]), 88)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i128; 12] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i128; 12], *const u8>(&(*self) as *const [i128; 12]), 12 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i128; 12] {
    fn raw_deserialize(from: &mut Read) -> Result<[i128; 12], Error> {
        unsafe { let mut buffer: [i128; 12] = [0i128; 12];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i128; 12], *mut u8>((&mut buffer) as *mut [i128; 12]), 96)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i128; 13] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i128; 13], *const u8>(&(*self) as *const [i128; 13]), 13 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i128; 13] {
    fn raw_deserialize(from: &mut Read) -> Result<[i128; 13], Error> {
        unsafe { let mut buffer: [i128; 13] = [0i128; 13];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i128; 13], *mut u8>((&mut buffer) as *mut [i128; 13]), 104)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i128; 14] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i128; 14], *const u8>(&(*self) as *const [i128; 14]), 14 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i128; 14] {
    fn raw_deserialize(from: &mut Read) -> Result<[i128; 14], Error> {
        unsafe { let mut buffer: [i128; 14] = [0i128; 14];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i128; 14], *mut u8>((&mut buffer) as *mut [i128; 14]), 112)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i128; 15] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i128; 15], *const u8>(&(*self) as *const [i128; 15]), 15 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i128; 15] {
    fn raw_deserialize(from: &mut Read) -> Result<[i128; 15], Error> {
        unsafe { let mut buffer: [i128; 15] = [0i128; 15];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i128; 15], *mut u8>((&mut buffer) as *mut [i128; 15]), 120)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i128; 16] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i128; 16], *const u8>(&(*self) as *const [i128; 16]), 16 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i128; 16] {
    fn raw_deserialize(from: &mut Read) -> Result<[i128; 16], Error> {
        unsafe { let mut buffer: [i128; 16] = [0i128; 16];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i128; 16], *mut u8>((&mut buffer) as *mut [i128; 16]), 128)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i128; 17] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i128; 17], *const u8>(&(*self) as *const [i128; 17]), 17 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i128; 17] {
    fn raw_deserialize(from: &mut Read) -> Result<[i128; 17], Error> {
        unsafe { let mut buffer: [i128; 17] = [0i128; 17];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i128; 17], *mut u8>((&mut buffer) as *mut [i128; 17]), 136)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i128; 18] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i128; 18], *const u8>(&(*self) as *const [i128; 18]), 18 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i128; 18] {
    fn raw_deserialize(from: &mut Read) -> Result<[i128; 18], Error> {
        unsafe { let mut buffer: [i128; 18] = [0i128; 18];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i128; 18], *mut u8>((&mut buffer) as *mut [i128; 18]), 144)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i128; 19] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i128; 19], *const u8>(&(*self) as *const [i128; 19]), 19 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i128; 19] {
    fn raw_deserialize(from: &mut Read) -> Result<[i128; 19], Error> {
        unsafe { let mut buffer: [i128; 19] = [0i128; 19];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i128; 19], *mut u8>((&mut buffer) as *mut [i128; 19]), 152)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i128; 20] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i128; 20], *const u8>(&(*self) as *const [i128; 20]), 20 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i128; 20] {
    fn raw_deserialize(from: &mut Read) -> Result<[i128; 20], Error> {
        unsafe { let mut buffer: [i128; 20] = [0i128; 20];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i128; 20], *mut u8>((&mut buffer) as *mut [i128; 20]), 160)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i128; 21] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i128; 21], *const u8>(&(*self) as *const [i128; 21]), 21 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i128; 21] {
    fn raw_deserialize(from: &mut Read) -> Result<[i128; 21], Error> {
        unsafe { let mut buffer: [i128; 21] = [0i128; 21];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i128; 21], *mut u8>((&mut buffer) as *mut [i128; 21]), 168)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i128; 22] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i128; 22], *const u8>(&(*self) as *const [i128; 22]), 22 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i128; 22] {
    fn raw_deserialize(from: &mut Read) -> Result<[i128; 22], Error> {
        unsafe { let mut buffer: [i128; 22] = [0i128; 22];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i128; 22], *mut u8>((&mut buffer) as *mut [i128; 22]), 176)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i128; 23] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i128; 23], *const u8>(&(*self) as *const [i128; 23]), 23 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i128; 23] {
    fn raw_deserialize(from: &mut Read) -> Result<[i128; 23], Error> {
        unsafe { let mut buffer: [i128; 23] = [0i128; 23];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i128; 23], *mut u8>((&mut buffer) as *mut [i128; 23]), 184)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i128; 24] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i128; 24], *const u8>(&(*self) as *const [i128; 24]), 24 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i128; 24] {
    fn raw_deserialize(from: &mut Read) -> Result<[i128; 24], Error> {
        unsafe { let mut buffer: [i128; 24] = [0i128; 24];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i128; 24], *mut u8>((&mut buffer) as *mut [i128; 24]), 192)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i128; 25] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i128; 25], *const u8>(&(*self) as *const [i128; 25]), 25 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i128; 25] {
    fn raw_deserialize(from: &mut Read) -> Result<[i128; 25], Error> {
        unsafe { let mut buffer: [i128; 25] = [0i128; 25];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i128; 25], *mut u8>((&mut buffer) as *mut [i128; 25]), 200)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i128; 26] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i128; 26], *const u8>(&(*self) as *const [i128; 26]), 26 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i128; 26] {
    fn raw_deserialize(from: &mut Read) -> Result<[i128; 26], Error> {
        unsafe { let mut buffer: [i128; 26] = [0i128; 26];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i128; 26], *mut u8>((&mut buffer) as *mut [i128; 26]), 208)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i128; 27] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i128; 27], *const u8>(&(*self) as *const [i128; 27]), 27 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i128; 27] {
    fn raw_deserialize(from: &mut Read) -> Result<[i128; 27], Error> {
        unsafe { let mut buffer: [i128; 27] = [0i128; 27];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i128; 27], *mut u8>((&mut buffer) as *mut [i128; 27]), 216)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i128; 28] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i128; 28], *const u8>(&(*self) as *const [i128; 28]), 28 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i128; 28] {
    fn raw_deserialize(from: &mut Read) -> Result<[i128; 28], Error> {
        unsafe { let mut buffer: [i128; 28] = [0i128; 28];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i128; 28], *mut u8>((&mut buffer) as *mut [i128; 28]), 224)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i128; 29] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i128; 29], *const u8>(&(*self) as *const [i128; 29]), 29 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i128; 29] {
    fn raw_deserialize(from: &mut Read) -> Result<[i128; 29], Error> {
        unsafe { let mut buffer: [i128; 29] = [0i128; 29];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i128; 29], *mut u8>((&mut buffer) as *mut [i128; 29]), 232)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i128; 30] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i128; 30], *const u8>(&(*self) as *const [i128; 30]), 30 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i128; 30] {
    fn raw_deserialize(from: &mut Read) -> Result<[i128; 30], Error> {
        unsafe { let mut buffer: [i128; 30] = [0i128; 30];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i128; 30], *mut u8>((&mut buffer) as *mut [i128; 30]), 240)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i128; 31] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i128; 31], *const u8>(&(*self) as *const [i128; 31]), 31 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i128; 31] {
    fn raw_deserialize(from: &mut Read) -> Result<[i128; 31], Error> {
        unsafe { let mut buffer: [i128; 31] = [0i128; 31];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i128; 31], *mut u8>((&mut buffer) as *mut [i128; 31]), 248)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [i128; 32] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ i128; 32], *const u8>(&(*self) as *const [i128; 32]), 32 * 8)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [i128; 32] {
    fn raw_deserialize(from: &mut Read) -> Result<[i128; 32], Error> {
        unsafe { let mut buffer: [i128; 32] = [0i128; 32];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ i128; 32], *mut u8>((&mut buffer) as *mut [i128; 32]), 256)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u128; 1] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u128; 1], *const u8>(&(*self) as *const [u128; 1]), 1 * 16)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u128; 1] {
    fn raw_deserialize(from: &mut Read) -> Result<[u128; 1], Error> {
        unsafe { let mut buffer: [u128; 1] = [0u128; 1];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u128; 1], *mut u8>((&mut buffer) as *mut [u128; 1]), 16)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u128; 2] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u128; 2], *const u8>(&(*self) as *const [u128; 2]), 2 * 16)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u128; 2] {
    fn raw_deserialize(from: &mut Read) -> Result<[u128; 2], Error> {
        unsafe { let mut buffer: [u128; 2] = [0u128; 2];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u128; 2], *mut u8>((&mut buffer) as *mut [u128; 2]), 32)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u128; 3] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u128; 3], *const u8>(&(*self) as *const [u128; 3]), 3 * 16)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u128; 3] {
    fn raw_deserialize(from: &mut Read) -> Result<[u128; 3], Error> {
        unsafe { let mut buffer: [u128; 3] = [0u128; 3];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u128; 3], *mut u8>((&mut buffer) as *mut [u128; 3]), 48)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u128; 4] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u128; 4], *const u8>(&(*self) as *const [u128; 4]), 4 * 16)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u128; 4] {
    fn raw_deserialize(from: &mut Read) -> Result<[u128; 4], Error> {
        unsafe { let mut buffer: [u128; 4] = [0u128; 4];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u128; 4], *mut u8>((&mut buffer) as *mut [u128; 4]), 64)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u128; 5] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u128; 5], *const u8>(&(*self) as *const [u128; 5]), 5 * 16)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u128; 5] {
    fn raw_deserialize(from: &mut Read) -> Result<[u128; 5], Error> {
        unsafe { let mut buffer: [u128; 5] = [0u128; 5];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u128; 5], *mut u8>((&mut buffer) as *mut [u128; 5]), 80)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u128; 6] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u128; 6], *const u8>(&(*self) as *const [u128; 6]), 6 * 16)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u128; 6] {
    fn raw_deserialize(from: &mut Read) -> Result<[u128; 6], Error> {
        unsafe { let mut buffer: [u128; 6] = [0u128; 6];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u128; 6], *mut u8>((&mut buffer) as *mut [u128; 6]), 96)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u128; 7] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u128; 7], *const u8>(&(*self) as *const [u128; 7]), 7 * 16)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u128; 7] {
    fn raw_deserialize(from: &mut Read) -> Result<[u128; 7], Error> {
        unsafe { let mut buffer: [u128; 7] = [0u128; 7];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u128; 7], *mut u8>((&mut buffer) as *mut [u128; 7]), 112)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u128; 8] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u128; 8], *const u8>(&(*self) as *const [u128; 8]), 8 * 16)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u128; 8] {
    fn raw_deserialize(from: &mut Read) -> Result<[u128; 8], Error> {
        unsafe { let mut buffer: [u128; 8] = [0u128; 8];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u128; 8], *mut u8>((&mut buffer) as *mut [u128; 8]), 128)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u128; 9] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u128; 9], *const u8>(&(*self) as *const [u128; 9]), 9 * 16)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u128; 9] {
    fn raw_deserialize(from: &mut Read) -> Result<[u128; 9], Error> {
        unsafe { let mut buffer: [u128; 9] = [0u128; 9];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u128; 9], *mut u8>((&mut buffer) as *mut [u128; 9]), 144)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u128; 10] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u128; 10], *const u8>(&(*self) as *const [u128; 10]), 10 * 16)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u128; 10] {
    fn raw_deserialize(from: &mut Read) -> Result<[u128; 10], Error> {
        unsafe { let mut buffer: [u128; 10] = [0u128; 10];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u128; 10], *mut u8>((&mut buffer) as *mut [u128; 10]), 160)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u128; 11] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u128; 11], *const u8>(&(*self) as *const [u128; 11]), 11 * 16)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u128; 11] {
    fn raw_deserialize(from: &mut Read) -> Result<[u128; 11], Error> {
        unsafe { let mut buffer: [u128; 11] = [0u128; 11];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u128; 11], *mut u8>((&mut buffer) as *mut [u128; 11]), 176)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u128; 12] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u128; 12], *const u8>(&(*self) as *const [u128; 12]), 12 * 16)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u128; 12] {
    fn raw_deserialize(from: &mut Read) -> Result<[u128; 12], Error> {
        unsafe { let mut buffer: [u128; 12] = [0u128; 12];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u128; 12], *mut u8>((&mut buffer) as *mut [u128; 12]), 192)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u128; 13] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u128; 13], *const u8>(&(*self) as *const [u128; 13]), 13 * 16)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u128; 13] {
    fn raw_deserialize(from: &mut Read) -> Result<[u128; 13], Error> {
        unsafe { let mut buffer: [u128; 13] = [0u128; 13];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u128; 13], *mut u8>((&mut buffer) as *mut [u128; 13]), 208)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u128; 14] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u128; 14], *const u8>(&(*self) as *const [u128; 14]), 14 * 16)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u128; 14] {
    fn raw_deserialize(from: &mut Read) -> Result<[u128; 14], Error> {
        unsafe { let mut buffer: [u128; 14] = [0u128; 14];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u128; 14], *mut u8>((&mut buffer) as *mut [u128; 14]), 224)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u128; 15] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u128; 15], *const u8>(&(*self) as *const [u128; 15]), 15 * 16)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u128; 15] {
    fn raw_deserialize(from: &mut Read) -> Result<[u128; 15], Error> {
        unsafe { let mut buffer: [u128; 15] = [0u128; 15];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u128; 15], *mut u8>((&mut buffer) as *mut [u128; 15]), 240)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u128; 16] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u128; 16], *const u8>(&(*self) as *const [u128; 16]), 16 * 16)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u128; 16] {
    fn raw_deserialize(from: &mut Read) -> Result<[u128; 16], Error> {
        unsafe { let mut buffer: [u128; 16] = [0u128; 16];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u128; 16], *mut u8>((&mut buffer) as *mut [u128; 16]), 256)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u128; 17] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u128; 17], *const u8>(&(*self) as *const [u128; 17]), 17 * 16)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u128; 17] {
    fn raw_deserialize(from: &mut Read) -> Result<[u128; 17], Error> {
        unsafe { let mut buffer: [u128; 17] = [0u128; 17];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u128; 17], *mut u8>((&mut buffer) as *mut [u128; 17]), 272)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u128; 18] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u128; 18], *const u8>(&(*self) as *const [u128; 18]), 18 * 16)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u128; 18] {
    fn raw_deserialize(from: &mut Read) -> Result<[u128; 18], Error> {
        unsafe { let mut buffer: [u128; 18] = [0u128; 18];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u128; 18], *mut u8>((&mut buffer) as *mut [u128; 18]), 288)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u128; 19] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u128; 19], *const u8>(&(*self) as *const [u128; 19]), 19 * 16)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u128; 19] {
    fn raw_deserialize(from: &mut Read) -> Result<[u128; 19], Error> {
        unsafe { let mut buffer: [u128; 19] = [0u128; 19];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u128; 19], *mut u8>((&mut buffer) as *mut [u128; 19]), 304)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u128; 20] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u128; 20], *const u8>(&(*self) as *const [u128; 20]), 20 * 16)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u128; 20] {
    fn raw_deserialize(from: &mut Read) -> Result<[u128; 20], Error> {
        unsafe { let mut buffer: [u128; 20] = [0u128; 20];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u128; 20], *mut u8>((&mut buffer) as *mut [u128; 20]), 320)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u128; 21] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u128; 21], *const u8>(&(*self) as *const [u128; 21]), 21 * 16)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u128; 21] {
    fn raw_deserialize(from: &mut Read) -> Result<[u128; 21], Error> {
        unsafe { let mut buffer: [u128; 21] = [0u128; 21];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u128; 21], *mut u8>((&mut buffer) as *mut [u128; 21]), 336)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u128; 22] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u128; 22], *const u8>(&(*self) as *const [u128; 22]), 22 * 16)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u128; 22] {
    fn raw_deserialize(from: &mut Read) -> Result<[u128; 22], Error> {
        unsafe { let mut buffer: [u128; 22] = [0u128; 22];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u128; 22], *mut u8>((&mut buffer) as *mut [u128; 22]), 352)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u128; 23] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u128; 23], *const u8>(&(*self) as *const [u128; 23]), 23 * 16)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u128; 23] {
    fn raw_deserialize(from: &mut Read) -> Result<[u128; 23], Error> {
        unsafe { let mut buffer: [u128; 23] = [0u128; 23];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u128; 23], *mut u8>((&mut buffer) as *mut [u128; 23]), 368)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u128; 24] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u128; 24], *const u8>(&(*self) as *const [u128; 24]), 24 * 16)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u128; 24] {
    fn raw_deserialize(from: &mut Read) -> Result<[u128; 24], Error> {
        unsafe { let mut buffer: [u128; 24] = [0u128; 24];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u128; 24], *mut u8>((&mut buffer) as *mut [u128; 24]), 384)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u128; 25] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u128; 25], *const u8>(&(*self) as *const [u128; 25]), 25 * 16)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u128; 25] {
    fn raw_deserialize(from: &mut Read) -> Result<[u128; 25], Error> {
        unsafe { let mut buffer: [u128; 25] = [0u128; 25];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u128; 25], *mut u8>((&mut buffer) as *mut [u128; 25]), 400)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u128; 26] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u128; 26], *const u8>(&(*self) as *const [u128; 26]), 26 * 16)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u128; 26] {
    fn raw_deserialize(from: &mut Read) -> Result<[u128; 26], Error> {
        unsafe { let mut buffer: [u128; 26] = [0u128; 26];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u128; 26], *mut u8>((&mut buffer) as *mut [u128; 26]), 416)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u128; 27] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u128; 27], *const u8>(&(*self) as *const [u128; 27]), 27 * 16)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u128; 27] {
    fn raw_deserialize(from: &mut Read) -> Result<[u128; 27], Error> {
        unsafe { let mut buffer: [u128; 27] = [0u128; 27];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u128; 27], *mut u8>((&mut buffer) as *mut [u128; 27]), 432)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u128; 28] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u128; 28], *const u8>(&(*self) as *const [u128; 28]), 28 * 16)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u128; 28] {
    fn raw_deserialize(from: &mut Read) -> Result<[u128; 28], Error> {
        unsafe { let mut buffer: [u128; 28] = [0u128; 28];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u128; 28], *mut u8>((&mut buffer) as *mut [u128; 28]), 448)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u128; 29] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u128; 29], *const u8>(&(*self) as *const [u128; 29]), 29 * 16)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u128; 29] {
    fn raw_deserialize(from: &mut Read) -> Result<[u128; 29], Error> {
        unsafe { let mut buffer: [u128; 29] = [0u128; 29];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u128; 29], *mut u8>((&mut buffer) as *mut [u128; 29]), 464)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u128; 30] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u128; 30], *const u8>(&(*self) as *const [u128; 30]), 30 * 16)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u128; 30] {
    fn raw_deserialize(from: &mut Read) -> Result<[u128; 30], Error> {
        unsafe { let mut buffer: [u128; 30] = [0u128; 30];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u128; 30], *mut u8>((&mut buffer) as *mut [u128; 30]), 480)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u128; 31] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u128; 31], *const u8>(&(*self) as *const [u128; 31]), 31 * 16)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u128; 31] {
    fn raw_deserialize(from: &mut Read) -> Result<[u128; 31], Error> {
        unsafe { let mut buffer: [u128; 31] = [0u128; 31];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u128; 31], *mut u8>((&mut buffer) as *mut [u128; 31]), 496)));
        }
        Ok(buffer)
        }
    }

}

impl RawSerialize for [u128; 32] {
     fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        let y = unsafe { slice::from_raw_parts(mem::transmute::<*const [ u128; 32], *const u8>(&(*self) as *const [u128; 32]), 32 * 16)};
        check!(to.write_all(y));
        Ok(())    }
}
impl RawDeserialize for [u128; 32] {
    fn raw_deserialize(from: &mut Read) -> Result<[u128; 32], Error> {
        unsafe { let mut buffer: [u128; 32] = [0u128; 32];
        {
            check!(from.read_exact(slice::from_raw_parts_mut( mem::transmute::<*mut [ u128; 32], *mut u8>((&mut buffer) as *mut [u128; 32]), 512)));
        }
        Ok(buffer)
        }
    }

}
