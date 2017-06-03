use serialize::RawSerialize;
use deserialize::RawDeserialize;
use std::io::{ Error, Read, Write };
use std::collections::*;
use std::hash::Hash;

impl<K, V> RawSerialize for HashMap<K, V> where K: Hash + Eq + RawSerialize, V: RawSerialize {
    fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        check!((self.len() as u64).raw_serialize(to));
        for (k, v) in self.iter() {
            check!(k.raw_serialize(to));
            check!(v.raw_serialize(to));
        }
        Ok(())
    }
}



impl<K, V> RawDeserialize for HashMap<K, V> where K: Hash + Eq + RawDeserialize, V: RawDeserialize {
    fn raw_deserialize(read: &mut Read) -> Result<Self, Error> {
        let len;
        check!(u64::raw_deserialize(read), len);
        let mut hashmap = HashMap::with_capacity(len as usize);
        for _ in 0..len {
            let k;
            check!(K::raw_deserialize(read), k);
            let v;
            check!(V::raw_deserialize(read), v);
            hashmap.insert(k, v);
        }
        Ok(hashmap)
    }
}

impl<K, V> RawSerialize for BTreeMap<K, V> where
    K: RawSerialize + Eq + Ord, V: RawSerialize {
    fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        check!((self.len() as u64).raw_serialize(to));
        for (k, v) in self.iter() {
            check!(k.raw_serialize(to));
            check!(v.raw_serialize(to));
        }
        Ok(())
    }

}

impl<K, V> RawDeserialize for BTreeMap<K, V> where
    K: RawDeserialize + Eq + Ord, V: RawDeserialize {
    fn raw_deserialize(read: &mut Read) -> Result<Self, Error> {
        let len;
        check!(u64::raw_deserialize(read), len);
        let mut hashmap = BTreeMap::new();
        for _ in 0..len {
            let k;
            check!(K::raw_deserialize(read), k);
            let v;
            check!(V::raw_deserialize(read), v);
            hashmap.insert(k, v);
        }
        Ok(hashmap)
    }
}

impl<K> RawSerialize for VecDeque<K> where K: RawSerialize {
    fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        check!((self.len() as u64).raw_serialize(to));
        for k in self.iter() {
            check!(k.raw_serialize(to));
        }
        Ok(())
    }
}


impl<K> RawDeserialize for VecDeque<K> where K: RawDeserialize {
    fn raw_deserialize(read: &mut Read) -> Result<Self, Error> {
        let len;
        check!(u64::raw_deserialize(read), len);
        let mut r = VecDeque::with_capacity(len as usize);
        for _ in 0..len {
            let k;
            check!(K::raw_deserialize(read), k);
            r.push_back(k);
        }
        Ok(r)
    }
}


impl<K> RawSerialize for Vec<K> where K: RawSerialize {
    fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        check!((self.len() as u64).raw_serialize(to));
        for k in self.iter() {
            check!(k.raw_serialize(to));
        }
        Ok(())
    }
}

impl<K> RawDeserialize for Vec<K> where K: RawDeserialize {
    fn raw_deserialize(read: &mut Read) -> Result<Self, Error> {
        let len;
        check!(u64::raw_deserialize(read), len);
        let mut r = Vec::with_capacity(len as usize);
        for _ in 0..len {
            let k;
            check!(K::raw_deserialize(read), k);
            r.push(k);
        }
        Ok(r)
    }
}

impl<K> RawSerialize for LinkedList<K> where K: RawSerialize {
    fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        check!((self.len() as u64).raw_serialize(to));
        for k in self.iter() {
            check!(k.raw_serialize(to));
        }
        Ok(())
    }
}

impl<K> RawDeserialize for LinkedList<K> where K: RawDeserialize {
    fn raw_deserialize(read: &mut Read) -> Result<Self, Error> {
        let len;
        check!(u64::raw_deserialize(read), len);
        let mut r = LinkedList::new();
        for _ in 0..len {
            let k;
            check!(K::raw_deserialize(read), k);
            r.push_back(k);
        }
        Ok(r)
    }
}

impl<K> RawSerialize for BTreeSet<K> where K: Eq + Ord + RawSerialize {
    fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        check!((self.len() as u64).raw_serialize(to));
        for k in self.iter() {
            check!(k.raw_serialize(to));
        }
        Ok(())
    }
}

impl<K> RawDeserialize for BTreeSet<K> where K: Eq + Ord + RawDeserialize {
    fn raw_deserialize(read: &mut Read) -> Result<Self, Error> {
        let len;
        check!(u64::raw_deserialize(read), len);
        let mut r = BTreeSet::new();
        for _ in 0..len {
            let k;
            check!(K::raw_deserialize(read), k);
            r.insert(k);
        }
        Ok(r)
    }
}


impl<K> RawSerialize for HashSet<K> where K: Hash + Eq + RawSerialize {
    fn raw_serialize(&self, to: &mut Write) -> Result<(), Error> {
        check!((self.len() as u64).raw_serialize(to));
        for k in self.iter() {
            check!(k.raw_serialize(to));
        }
        Ok(())
    }
}

impl<K> RawDeserialize for HashSet<K> where K: Hash + Eq + RawDeserialize {
    fn raw_deserialize(read: &mut Read) -> Result<Self, Error> {
        let len;
        check!(u64::raw_deserialize(read), len);
        let mut r = HashSet::new();
        for _ in 0..len {
            let k;
            check!(K::raw_deserialize(read), k);
            r.insert(k);
        }
        Ok(r)
    }
}
