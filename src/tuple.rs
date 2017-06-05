use serialize::RawSerialize;
use deserialize::RawDeserialize;
use std::io::{ Error, Read, Write };

impl<A, B> RawSerialize for (A, B) where A: RawSerialize,
    B: RawSerialize {
    fn raw_serialize(&self, to: &mut Write) -> Result<u64, Error> {
        let mut size = 0;
        let size_x;
        check!(self.0.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.1.raw_serialize(to), size_x);
        size += size_x;
        Ok(size)
    }
}

impl<A, B> RawDeserialize for (A, B) where A: RawDeserialize,
    B: RawDeserialize {
    fn raw_deserialize(from: &mut Read) -> Result<(A, B), Error> {
        let _a;
        check!(A::raw_deserialize(from), _a);
        let _b;
        check!(B::raw_deserialize(from), _b);
        Ok((_a, _b))
    }
}

impl<A, B, C> RawSerialize for (A, B, C) where A: RawSerialize,
    B: RawSerialize,
    C: RawSerialize {
    fn raw_serialize(&self, to: &mut Write) -> Result<u64, Error> {
        let mut size = 0;
        let size_x;
        check!(self.0.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.1.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.2.raw_serialize(to), size_x);
        size += size_x;
        Ok(size)
    }
}

impl<A, B, C> RawDeserialize for (A, B, C) where A: RawDeserialize,
    B: RawDeserialize,
    C: RawDeserialize {
    fn raw_deserialize(from: &mut Read) -> Result<(A, B, C), Error> {
        let _a;
        check!(A::raw_deserialize(from), _a);
        let _b;
        check!(B::raw_deserialize(from), _b);
        let _c;
        check!(C::raw_deserialize(from), _c);
        Ok((_a, _b, _c))
    }
}

impl<A, B, C, D> RawSerialize for (A, B, C, D) where A: RawSerialize,
    B: RawSerialize,
    C: RawSerialize,
    D: RawSerialize {
    fn raw_serialize(&self, to: &mut Write) -> Result<u64, Error> {
        let mut size = 0;
        let size_x;
        check!(self.0.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.1.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.2.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.3.raw_serialize(to), size_x);
        size += size_x;
        Ok(size)
    }
}

impl<A, B, C, D> RawDeserialize for (A, B, C, D) where A: RawDeserialize,
    B: RawDeserialize,
    C: RawDeserialize,
    D: RawDeserialize {
    fn raw_deserialize(from: &mut Read) -> Result<(A, B, C, D), Error> {
        let _a;
        check!(A::raw_deserialize(from), _a);
        let _b;
        check!(B::raw_deserialize(from), _b);
        let _c;
        check!(C::raw_deserialize(from), _c);
        let _d;
        check!(D::raw_deserialize(from), _d);
        Ok((_a, _b, _c, _d))
    }
}

impl<A, B, C, D, E> RawSerialize for (A, B, C, D, E) where A: RawSerialize,
    B: RawSerialize,
    C: RawSerialize,
    D: RawSerialize,
    E: RawSerialize {
    fn raw_serialize(&self, to: &mut Write) -> Result<u64, Error> {
        let mut size = 0;
        let size_x;
        check!(self.0.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.1.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.2.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.3.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.4.raw_serialize(to), size_x);
        size += size_x;
        Ok(size)
    }
}

impl<A, B, C, D, E> RawDeserialize for (A, B, C, D, E) where A: RawDeserialize,
    B: RawDeserialize,
    C: RawDeserialize,
    D: RawDeserialize,
    E: RawDeserialize {
    fn raw_deserialize(from: &mut Read) -> Result<(A, B, C, D, E), Error> {
        let _a;
        check!(A::raw_deserialize(from), _a);
        let _b;
        check!(B::raw_deserialize(from), _b);
        let _c;
        check!(C::raw_deserialize(from), _c);
        let _d;
        check!(D::raw_deserialize(from), _d);
        let _e;
        check!(E::raw_deserialize(from), _e);
        Ok((_a, _b, _c, _d, _e))
    }
}

impl<A, B, C, D, E, F> RawSerialize for (A, B, C, D, E, F) where A: RawSerialize,
    B: RawSerialize,
    C: RawSerialize,
    D: RawSerialize,
    E: RawSerialize,
    F: RawSerialize {
    fn raw_serialize(&self, to: &mut Write) -> Result<u64, Error> {
        let mut size = 0;
        let size_x;
        check!(self.0.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.1.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.2.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.3.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.4.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.5.raw_serialize(to), size_x);
        size += size_x;
        Ok(size)
    }
}

impl<A, B, C, D, E, F> RawDeserialize for (A, B, C, D, E, F) where A: RawDeserialize,
    B: RawDeserialize,
    C: RawDeserialize,
    D: RawDeserialize,
    E: RawDeserialize,
    F: RawDeserialize {
    fn raw_deserialize(from: &mut Read) -> Result<(A, B, C, D, E, F), Error> {
        let _a;
        check!(A::raw_deserialize(from), _a);
        let _b;
        check!(B::raw_deserialize(from), _b);
        let _c;
        check!(C::raw_deserialize(from), _c);
        let _d;
        check!(D::raw_deserialize(from), _d);
        let _e;
        check!(E::raw_deserialize(from), _e);
        let _f;
        check!(F::raw_deserialize(from), _f);
        Ok((_a, _b, _c, _d, _e, _f))
    }
}

impl<A, B, C, D, E, F, G> RawSerialize for (A, B, C, D, E, F, G) where A: RawSerialize,
    B: RawSerialize,
    C: RawSerialize,
    D: RawSerialize,
    E: RawSerialize,
    F: RawSerialize,
    G: RawSerialize {
    fn raw_serialize(&self, to: &mut Write) -> Result<u64, Error> {
        let mut size = 0;
        let size_x;
        check!(self.0.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.1.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.2.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.3.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.4.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.5.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.6.raw_serialize(to), size_x);
        size += size_x;
        Ok(size)
    }
}

impl<A, B, C, D, E, F, G> RawDeserialize for (A, B, C, D, E, F, G) where A: RawDeserialize,
    B: RawDeserialize,
    C: RawDeserialize,
    D: RawDeserialize,
    E: RawDeserialize,
    F: RawDeserialize,
    G: RawDeserialize {
    fn raw_deserialize(from: &mut Read) -> Result<(A, B, C, D, E, F, G), Error> {
        let _a;
        check!(A::raw_deserialize(from), _a);
        let _b;
        check!(B::raw_deserialize(from), _b);
        let _c;
        check!(C::raw_deserialize(from), _c);
        let _d;
        check!(D::raw_deserialize(from), _d);
        let _e;
        check!(E::raw_deserialize(from), _e);
        let _f;
        check!(F::raw_deserialize(from), _f);
        let _g;
        check!(G::raw_deserialize(from), _g);
        Ok((_a, _b, _c, _d, _e, _f, _g))
    }
}

impl<A, B, C, D, E, F, G, H> RawSerialize for (A, B, C, D, E, F, G, H) where A: RawSerialize,
    B: RawSerialize,
    C: RawSerialize,
    D: RawSerialize,
    E: RawSerialize,
    F: RawSerialize,
    G: RawSerialize,
    H: RawSerialize {
    fn raw_serialize(&self, to: &mut Write) -> Result<u64, Error> {
        let mut size = 0;
        let size_x;
        check!(self.0.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.1.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.2.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.3.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.4.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.5.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.6.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.7.raw_serialize(to), size_x);
        size += size_x;
        Ok(size)
    }
}

impl<A, B, C, D, E, F, G, H> RawDeserialize for (A, B, C, D, E, F, G, H) where A: RawDeserialize,
    B: RawDeserialize,
    C: RawDeserialize,
    D: RawDeserialize,
    E: RawDeserialize,
    F: RawDeserialize,
    G: RawDeserialize,
    H: RawDeserialize {
    fn raw_deserialize(from: &mut Read) -> Result<(A, B, C, D, E, F, G, H), Error> {
        let _a;
        check!(A::raw_deserialize(from), _a);
        let _b;
        check!(B::raw_deserialize(from), _b);
        let _c;
        check!(C::raw_deserialize(from), _c);
        let _d;
        check!(D::raw_deserialize(from), _d);
        let _e;
        check!(E::raw_deserialize(from), _e);
        let _f;
        check!(F::raw_deserialize(from), _f);
        let _g;
        check!(G::raw_deserialize(from), _g);
        let _h;
        check!(H::raw_deserialize(from), _h);
        Ok((_a, _b, _c, _d, _e, _f, _g, _h))
    }
}

impl<A, B, C, D, E, F, G, H, I> RawSerialize for (A, B, C, D, E, F, G, H, I) where A: RawSerialize,
    B: RawSerialize,
    C: RawSerialize,
    D: RawSerialize,
    E: RawSerialize,
    F: RawSerialize,
    G: RawSerialize,
    H: RawSerialize,
    I: RawSerialize {
    fn raw_serialize(&self, to: &mut Write) -> Result<u64, Error> {
        let mut size = 0;
        let size_x;
        check!(self.0.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.1.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.2.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.3.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.4.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.5.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.6.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.7.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.8.raw_serialize(to), size_x);
        size += size_x;
        Ok(size)
    }
}

impl<A, B, C, D, E, F, G, H, I> RawDeserialize for (A, B, C, D, E, F, G, H, I) where A: RawDeserialize,
    B: RawDeserialize,
    C: RawDeserialize,
    D: RawDeserialize,
    E: RawDeserialize,
    F: RawDeserialize,
    G: RawDeserialize,
    H: RawDeserialize,
    I: RawDeserialize {
    fn raw_deserialize(from: &mut Read) -> Result<(A, B, C, D, E, F, G, H, I), Error> {
        let _a;
        check!(A::raw_deserialize(from), _a);
        let _b;
        check!(B::raw_deserialize(from), _b);
        let _c;
        check!(C::raw_deserialize(from), _c);
        let _d;
        check!(D::raw_deserialize(from), _d);
        let _e;
        check!(E::raw_deserialize(from), _e);
        let _f;
        check!(F::raw_deserialize(from), _f);
        let _g;
        check!(G::raw_deserialize(from), _g);
        let _h;
        check!(H::raw_deserialize(from), _h);
        let _i;
        check!(I::raw_deserialize(from), _i);
        Ok((_a, _b, _c, _d, _e, _f, _g, _h, _i))
    }
}

impl<A, B, C, D, E, F, G, H, I, J> RawSerialize for (A, B, C, D, E, F, G, H, I, J) where A: RawSerialize,
    B: RawSerialize,
    C: RawSerialize,
    D: RawSerialize,
    E: RawSerialize,
    F: RawSerialize,
    G: RawSerialize,
    H: RawSerialize,
    I: RawSerialize,
    J: RawSerialize {
    fn raw_serialize(&self, to: &mut Write) -> Result<u64, Error> {
        let mut size = 0;
        let size_x;
        check!(self.0.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.1.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.2.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.3.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.4.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.5.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.6.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.7.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.8.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.9.raw_serialize(to), size_x);
        size += size_x;
        Ok(size)
    }
}

impl<A, B, C, D, E, F, G, H, I, J> RawDeserialize for (A, B, C, D, E, F, G, H, I, J) where A: RawDeserialize,
    B: RawDeserialize,
    C: RawDeserialize,
    D: RawDeserialize,
    E: RawDeserialize,
    F: RawDeserialize,
    G: RawDeserialize,
    H: RawDeserialize,
    I: RawDeserialize,
    J: RawDeserialize {
    fn raw_deserialize(from: &mut Read) -> Result<(A, B, C, D, E, F, G, H, I, J), Error> {
        let _a;
        check!(A::raw_deserialize(from), _a);
        let _b;
        check!(B::raw_deserialize(from), _b);
        let _c;
        check!(C::raw_deserialize(from), _c);
        let _d;
        check!(D::raw_deserialize(from), _d);
        let _e;
        check!(E::raw_deserialize(from), _e);
        let _f;
        check!(F::raw_deserialize(from), _f);
        let _g;
        check!(G::raw_deserialize(from), _g);
        let _h;
        check!(H::raw_deserialize(from), _h);
        let _i;
        check!(I::raw_deserialize(from), _i);
        let _j;
        check!(J::raw_deserialize(from), _j);
        Ok((_a, _b, _c, _d, _e, _f, _g, _h, _i, _j))
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K> RawSerialize for (A, B, C, D, E, F, G, H, I, J, K) where A: RawSerialize,
    B: RawSerialize,
    C: RawSerialize,
    D: RawSerialize,
    E: RawSerialize,
    F: RawSerialize,
    G: RawSerialize,
    H: RawSerialize,
    I: RawSerialize,
    J: RawSerialize,
    K: RawSerialize {
    fn raw_serialize(&self, to: &mut Write) -> Result<u64, Error> {
        let mut size = 0;
        let size_x;
        check!(self.0.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.1.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.2.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.3.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.4.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.5.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.6.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.7.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.8.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.9.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.10.raw_serialize(to), size_x);
        size += size_x;
        Ok(size)
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K> RawDeserialize for (A, B, C, D, E, F, G, H, I, J, K) where A: RawDeserialize,
    B: RawDeserialize,
    C: RawDeserialize,
    D: RawDeserialize,
    E: RawDeserialize,
    F: RawDeserialize,
    G: RawDeserialize,
    H: RawDeserialize,
    I: RawDeserialize,
    J: RawDeserialize,
    K: RawDeserialize {
    fn raw_deserialize(from: &mut Read) -> Result<(A, B, C, D, E, F, G, H, I, J, K), Error> {
        let _a;
        check!(A::raw_deserialize(from), _a);
        let _b;
        check!(B::raw_deserialize(from), _b);
        let _c;
        check!(C::raw_deserialize(from), _c);
        let _d;
        check!(D::raw_deserialize(from), _d);
        let _e;
        check!(E::raw_deserialize(from), _e);
        let _f;
        check!(F::raw_deserialize(from), _f);
        let _g;
        check!(G::raw_deserialize(from), _g);
        let _h;
        check!(H::raw_deserialize(from), _h);
        let _i;
        check!(I::raw_deserialize(from), _i);
        let _j;
        check!(J::raw_deserialize(from), _j);
        let _k;
        check!(K::raw_deserialize(from), _k);
        Ok((_a, _b, _c, _d, _e, _f, _g, _h, _i, _j, _k))
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L> RawSerialize for (A, B, C, D, E, F, G, H, I, J, K, L) where A: RawSerialize,
    B: RawSerialize,
    C: RawSerialize,
    D: RawSerialize,
    E: RawSerialize,
    F: RawSerialize,
    G: RawSerialize,
    H: RawSerialize,
    I: RawSerialize,
    J: RawSerialize,
    K: RawSerialize,
    L: RawSerialize {
    fn raw_serialize(&self, to: &mut Write) -> Result<u64, Error> {
        let mut size = 0;
        let size_x;
        check!(self.0.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.1.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.2.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.3.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.4.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.5.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.6.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.7.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.8.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.9.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.10.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.11.raw_serialize(to), size_x);
        size += size_x;
        Ok(size)
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L> RawDeserialize for (A, B, C, D, E, F, G, H, I, J, K, L) where A: RawDeserialize,
    B: RawDeserialize,
    C: RawDeserialize,
    D: RawDeserialize,
    E: RawDeserialize,
    F: RawDeserialize,
    G: RawDeserialize,
    H: RawDeserialize,
    I: RawDeserialize,
    J: RawDeserialize,
    K: RawDeserialize,
    L: RawDeserialize {
    fn raw_deserialize(from: &mut Read) -> Result<(A, B, C, D, E, F, G, H, I, J, K, L), Error> {
        let _a;
        check!(A::raw_deserialize(from), _a);
        let _b;
        check!(B::raw_deserialize(from), _b);
        let _c;
        check!(C::raw_deserialize(from), _c);
        let _d;
        check!(D::raw_deserialize(from), _d);
        let _e;
        check!(E::raw_deserialize(from), _e);
        let _f;
        check!(F::raw_deserialize(from), _f);
        let _g;
        check!(G::raw_deserialize(from), _g);
        let _h;
        check!(H::raw_deserialize(from), _h);
        let _i;
        check!(I::raw_deserialize(from), _i);
        let _j;
        check!(J::raw_deserialize(from), _j);
        let _k;
        check!(K::raw_deserialize(from), _k);
        let _l;
        check!(L::raw_deserialize(from), _l);
        Ok((_a, _b, _c, _d, _e, _f, _g, _h, _i, _j, _k, _l))
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L, M> RawSerialize for (A, B, C, D, E, F, G, H, I, J, K, L, M) where A: RawSerialize,
    B: RawSerialize,
    C: RawSerialize,
    D: RawSerialize,
    E: RawSerialize,
    F: RawSerialize,
    G: RawSerialize,
    H: RawSerialize,
    I: RawSerialize,
    J: RawSerialize,
    K: RawSerialize,
    L: RawSerialize,
    M: RawSerialize {
    fn raw_serialize(&self, to: &mut Write) -> Result<u64, Error> {
        let mut size = 0;
        let size_x;
        check!(self.0.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.1.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.2.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.3.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.4.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.5.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.6.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.7.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.8.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.9.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.10.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.11.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.12.raw_serialize(to), size_x);
        size += size_x;
        Ok(size)
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L, M> RawDeserialize for (A, B, C, D, E, F, G, H, I, J, K, L, M) where A: RawDeserialize,
    B: RawDeserialize,
    C: RawDeserialize,
    D: RawDeserialize,
    E: RawDeserialize,
    F: RawDeserialize,
    G: RawDeserialize,
    H: RawDeserialize,
    I: RawDeserialize,
    J: RawDeserialize,
    K: RawDeserialize,
    L: RawDeserialize,
    M: RawDeserialize {
    fn raw_deserialize(from: &mut Read) -> Result<(A, B, C, D, E, F, G, H, I, J, K, L, M), Error> {
        let _a;
        check!(A::raw_deserialize(from), _a);
        let _b;
        check!(B::raw_deserialize(from), _b);
        let _c;
        check!(C::raw_deserialize(from), _c);
        let _d;
        check!(D::raw_deserialize(from), _d);
        let _e;
        check!(E::raw_deserialize(from), _e);
        let _f;
        check!(F::raw_deserialize(from), _f);
        let _g;
        check!(G::raw_deserialize(from), _g);
        let _h;
        check!(H::raw_deserialize(from), _h);
        let _i;
        check!(I::raw_deserialize(from), _i);
        let _j;
        check!(J::raw_deserialize(from), _j);
        let _k;
        check!(K::raw_deserialize(from), _k);
        let _l;
        check!(L::raw_deserialize(from), _l);
        let _m;
        check!(M::raw_deserialize(from), _m);
        Ok((_a, _b, _c, _d, _e, _f, _g, _h, _i, _j, _k, _l, _m))
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N> RawSerialize for (A, B, C, D, E, F, G, H, I, J, K, L, M, N) where A: RawSerialize,
    B: RawSerialize,
    C: RawSerialize,
    D: RawSerialize,
    E: RawSerialize,
    F: RawSerialize,
    G: RawSerialize,
    H: RawSerialize,
    I: RawSerialize,
    J: RawSerialize,
    K: RawSerialize,
    L: RawSerialize,
    M: RawSerialize,
    N: RawSerialize {
    fn raw_serialize(&self, to: &mut Write) -> Result<u64, Error> {
        let mut size = 0;
        let size_x;
        check!(self.0.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.1.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.2.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.3.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.4.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.5.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.6.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.7.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.8.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.9.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.10.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.11.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.12.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.13.raw_serialize(to), size_x);
        size += size_x;
        Ok(size)
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N> RawDeserialize for (A, B, C, D, E, F, G, H, I, J, K, L, M, N) where A: RawDeserialize,
    B: RawDeserialize,
    C: RawDeserialize,
    D: RawDeserialize,
    E: RawDeserialize,
    F: RawDeserialize,
    G: RawDeserialize,
    H: RawDeserialize,
    I: RawDeserialize,
    J: RawDeserialize,
    K: RawDeserialize,
    L: RawDeserialize,
    M: RawDeserialize,
    N: RawDeserialize {
    fn raw_deserialize(from: &mut Read) -> Result<(A, B, C, D, E, F, G, H, I, J, K, L, M, N), Error> {
        let _a;
        check!(A::raw_deserialize(from), _a);
        let _b;
        check!(B::raw_deserialize(from), _b);
        let _c;
        check!(C::raw_deserialize(from), _c);
        let _d;
        check!(D::raw_deserialize(from), _d);
        let _e;
        check!(E::raw_deserialize(from), _e);
        let _f;
        check!(F::raw_deserialize(from), _f);
        let _g;
        check!(G::raw_deserialize(from), _g);
        let _h;
        check!(H::raw_deserialize(from), _h);
        let _i;
        check!(I::raw_deserialize(from), _i);
        let _j;
        check!(J::raw_deserialize(from), _j);
        let _k;
        check!(K::raw_deserialize(from), _k);
        let _l;
        check!(L::raw_deserialize(from), _l);
        let _m;
        check!(M::raw_deserialize(from), _m);
        let _n;
        check!(N::raw_deserialize(from), _n);
        Ok((_a, _b, _c, _d, _e, _f, _g, _h, _i, _j, _k, _l, _m, _n))
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O> RawSerialize for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O) where A: RawSerialize,
    B: RawSerialize,
    C: RawSerialize,
    D: RawSerialize,
    E: RawSerialize,
    F: RawSerialize,
    G: RawSerialize,
    H: RawSerialize,
    I: RawSerialize,
    J: RawSerialize,
    K: RawSerialize,
    L: RawSerialize,
    M: RawSerialize,
    N: RawSerialize,
    O: RawSerialize {
    fn raw_serialize(&self, to: &mut Write) -> Result<u64, Error> {
        let mut size = 0;
        let size_x;
        check!(self.0.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.1.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.2.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.3.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.4.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.5.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.6.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.7.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.8.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.9.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.10.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.11.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.12.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.13.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.14.raw_serialize(to), size_x);
        size += size_x;
        Ok(size)
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O> RawDeserialize for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O) where A: RawDeserialize,
    B: RawDeserialize,
    C: RawDeserialize,
    D: RawDeserialize,
    E: RawDeserialize,
    F: RawDeserialize,
    G: RawDeserialize,
    H: RawDeserialize,
    I: RawDeserialize,
    J: RawDeserialize,
    K: RawDeserialize,
    L: RawDeserialize,
    M: RawDeserialize,
    N: RawDeserialize,
    O: RawDeserialize {
    fn raw_deserialize(from: &mut Read) -> Result<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O), Error> {
        let _a;
        check!(A::raw_deserialize(from), _a);
        let _b;
        check!(B::raw_deserialize(from), _b);
        let _c;
        check!(C::raw_deserialize(from), _c);
        let _d;
        check!(D::raw_deserialize(from), _d);
        let _e;
        check!(E::raw_deserialize(from), _e);
        let _f;
        check!(F::raw_deserialize(from), _f);
        let _g;
        check!(G::raw_deserialize(from), _g);
        let _h;
        check!(H::raw_deserialize(from), _h);
        let _i;
        check!(I::raw_deserialize(from), _i);
        let _j;
        check!(J::raw_deserialize(from), _j);
        let _k;
        check!(K::raw_deserialize(from), _k);
        let _l;
        check!(L::raw_deserialize(from), _l);
        let _m;
        check!(M::raw_deserialize(from), _m);
        let _n;
        check!(N::raw_deserialize(from), _n);
        let _o;
        check!(O::raw_deserialize(from), _o);
        Ok((_a, _b, _c, _d, _e, _f, _g, _h, _i, _j, _k, _l, _m, _n, _o))
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P> RawSerialize for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P) where A: RawSerialize,
    B: RawSerialize,
    C: RawSerialize,
    D: RawSerialize,
    E: RawSerialize,
    F: RawSerialize,
    G: RawSerialize,
    H: RawSerialize,
    I: RawSerialize,
    J: RawSerialize,
    K: RawSerialize,
    L: RawSerialize,
    M: RawSerialize,
    N: RawSerialize,
    O: RawSerialize,
    P: RawSerialize {
    fn raw_serialize(&self, to: &mut Write) -> Result<u64, Error> {
        let mut size = 0;
        let size_x;
        check!(self.0.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.1.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.2.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.3.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.4.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.5.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.6.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.7.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.8.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.9.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.10.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.11.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.12.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.13.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.14.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.15.raw_serialize(to), size_x);
        size += size_x;
        Ok(size)
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P> RawDeserialize for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P) where A: RawDeserialize,
    B: RawDeserialize,
    C: RawDeserialize,
    D: RawDeserialize,
    E: RawDeserialize,
    F: RawDeserialize,
    G: RawDeserialize,
    H: RawDeserialize,
    I: RawDeserialize,
    J: RawDeserialize,
    K: RawDeserialize,
    L: RawDeserialize,
    M: RawDeserialize,
    N: RawDeserialize,
    O: RawDeserialize,
    P: RawDeserialize {
    fn raw_deserialize(from: &mut Read) -> Result<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P), Error> {
        let _a;
        check!(A::raw_deserialize(from), _a);
        let _b;
        check!(B::raw_deserialize(from), _b);
        let _c;
        check!(C::raw_deserialize(from), _c);
        let _d;
        check!(D::raw_deserialize(from), _d);
        let _e;
        check!(E::raw_deserialize(from), _e);
        let _f;
        check!(F::raw_deserialize(from), _f);
        let _g;
        check!(G::raw_deserialize(from), _g);
        let _h;
        check!(H::raw_deserialize(from), _h);
        let _i;
        check!(I::raw_deserialize(from), _i);
        let _j;
        check!(J::raw_deserialize(from), _j);
        let _k;
        check!(K::raw_deserialize(from), _k);
        let _l;
        check!(L::raw_deserialize(from), _l);
        let _m;
        check!(M::raw_deserialize(from), _m);
        let _n;
        check!(N::raw_deserialize(from), _n);
        let _o;
        check!(O::raw_deserialize(from), _o);
        let _p;
        check!(P::raw_deserialize(from), _p);
        Ok((_a, _b, _c, _d, _e, _f, _g, _h, _i, _j, _k, _l, _m, _n, _o, _p))
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q> RawSerialize for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q) where A: RawSerialize,
    B: RawSerialize,
    C: RawSerialize,
    D: RawSerialize,
    E: RawSerialize,
    F: RawSerialize,
    G: RawSerialize,
    H: RawSerialize,
    I: RawSerialize,
    J: RawSerialize,
    K: RawSerialize,
    L: RawSerialize,
    M: RawSerialize,
    N: RawSerialize,
    O: RawSerialize,
    P: RawSerialize,
    Q: RawSerialize {
    fn raw_serialize(&self, to: &mut Write) -> Result<u64, Error> {
        let mut size = 0;
        let size_x;
        check!(self.0.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.1.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.2.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.3.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.4.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.5.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.6.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.7.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.8.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.9.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.10.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.11.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.12.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.13.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.14.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.15.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.16.raw_serialize(to), size_x);
        size += size_x;
        Ok(size)
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q> RawDeserialize for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q) where A: RawDeserialize,
    B: RawDeserialize,
    C: RawDeserialize,
    D: RawDeserialize,
    E: RawDeserialize,
    F: RawDeserialize,
    G: RawDeserialize,
    H: RawDeserialize,
    I: RawDeserialize,
    J: RawDeserialize,
    K: RawDeserialize,
    L: RawDeserialize,
    M: RawDeserialize,
    N: RawDeserialize,
    O: RawDeserialize,
    P: RawDeserialize,
    Q: RawDeserialize {
    fn raw_deserialize(from: &mut Read) -> Result<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q), Error> {
        let _a;
        check!(A::raw_deserialize(from), _a);
        let _b;
        check!(B::raw_deserialize(from), _b);
        let _c;
        check!(C::raw_deserialize(from), _c);
        let _d;
        check!(D::raw_deserialize(from), _d);
        let _e;
        check!(E::raw_deserialize(from), _e);
        let _f;
        check!(F::raw_deserialize(from), _f);
        let _g;
        check!(G::raw_deserialize(from), _g);
        let _h;
        check!(H::raw_deserialize(from), _h);
        let _i;
        check!(I::raw_deserialize(from), _i);
        let _j;
        check!(J::raw_deserialize(from), _j);
        let _k;
        check!(K::raw_deserialize(from), _k);
        let _l;
        check!(L::raw_deserialize(from), _l);
        let _m;
        check!(M::raw_deserialize(from), _m);
        let _n;
        check!(N::raw_deserialize(from), _n);
        let _o;
        check!(O::raw_deserialize(from), _o);
        let _p;
        check!(P::raw_deserialize(from), _p);
        let _q;
        check!(Q::raw_deserialize(from), _q);
        Ok((_a, _b, _c, _d, _e, _f, _g, _h, _i, _j, _k, _l, _m, _n, _o, _p, _q))
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R> RawSerialize for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R) where A: RawSerialize,
    B: RawSerialize,
    C: RawSerialize,
    D: RawSerialize,
    E: RawSerialize,
    F: RawSerialize,
    G: RawSerialize,
    H: RawSerialize,
    I: RawSerialize,
    J: RawSerialize,
    K: RawSerialize,
    L: RawSerialize,
    M: RawSerialize,
    N: RawSerialize,
    O: RawSerialize,
    P: RawSerialize,
    Q: RawSerialize,
    R: RawSerialize {
    fn raw_serialize(&self, to: &mut Write) -> Result<u64, Error> {
        let mut size = 0;
        let size_x;
        check!(self.0.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.1.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.2.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.3.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.4.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.5.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.6.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.7.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.8.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.9.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.10.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.11.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.12.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.13.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.14.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.15.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.16.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.17.raw_serialize(to), size_x);
        size += size_x;
        Ok(size)
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R> RawDeserialize for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R) where A: RawDeserialize,
    B: RawDeserialize,
    C: RawDeserialize,
    D: RawDeserialize,
    E: RawDeserialize,
    F: RawDeserialize,
    G: RawDeserialize,
    H: RawDeserialize,
    I: RawDeserialize,
    J: RawDeserialize,
    K: RawDeserialize,
    L: RawDeserialize,
    M: RawDeserialize,
    N: RawDeserialize,
    O: RawDeserialize,
    P: RawDeserialize,
    Q: RawDeserialize,
    R: RawDeserialize {
    fn raw_deserialize(from: &mut Read) -> Result<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R), Error> {
        let _a;
        check!(A::raw_deserialize(from), _a);
        let _b;
        check!(B::raw_deserialize(from), _b);
        let _c;
        check!(C::raw_deserialize(from), _c);
        let _d;
        check!(D::raw_deserialize(from), _d);
        let _e;
        check!(E::raw_deserialize(from), _e);
        let _f;
        check!(F::raw_deserialize(from), _f);
        let _g;
        check!(G::raw_deserialize(from), _g);
        let _h;
        check!(H::raw_deserialize(from), _h);
        let _i;
        check!(I::raw_deserialize(from), _i);
        let _j;
        check!(J::raw_deserialize(from), _j);
        let _k;
        check!(K::raw_deserialize(from), _k);
        let _l;
        check!(L::raw_deserialize(from), _l);
        let _m;
        check!(M::raw_deserialize(from), _m);
        let _n;
        check!(N::raw_deserialize(from), _n);
        let _o;
        check!(O::raw_deserialize(from), _o);
        let _p;
        check!(P::raw_deserialize(from), _p);
        let _q;
        check!(Q::raw_deserialize(from), _q);
        let _r;
        check!(R::raw_deserialize(from), _r);
        Ok((_a, _b, _c, _d, _e, _f, _g, _h, _i, _j, _k, _l, _m, _n, _o, _p, _q, _r))
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S> RawSerialize for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S) where A: RawSerialize,
    B: RawSerialize,
    C: RawSerialize,
    D: RawSerialize,
    E: RawSerialize,
    F: RawSerialize,
    G: RawSerialize,
    H: RawSerialize,
    I: RawSerialize,
    J: RawSerialize,
    K: RawSerialize,
    L: RawSerialize,
    M: RawSerialize,
    N: RawSerialize,
    O: RawSerialize,
    P: RawSerialize,
    Q: RawSerialize,
    R: RawSerialize,
    S: RawSerialize {
    fn raw_serialize(&self, to: &mut Write) -> Result<u64, Error> {
        let mut size = 0;
        let size_x;
        check!(self.0.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.1.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.2.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.3.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.4.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.5.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.6.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.7.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.8.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.9.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.10.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.11.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.12.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.13.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.14.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.15.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.16.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.17.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.18.raw_serialize(to), size_x);
        size += size_x;
        Ok(size)
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S> RawDeserialize for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S) where A: RawDeserialize,
    B: RawDeserialize,
    C: RawDeserialize,
    D: RawDeserialize,
    E: RawDeserialize,
    F: RawDeserialize,
    G: RawDeserialize,
    H: RawDeserialize,
    I: RawDeserialize,
    J: RawDeserialize,
    K: RawDeserialize,
    L: RawDeserialize,
    M: RawDeserialize,
    N: RawDeserialize,
    O: RawDeserialize,
    P: RawDeserialize,
    Q: RawDeserialize,
    R: RawDeserialize,
    S: RawDeserialize {
    fn raw_deserialize(from: &mut Read) -> Result<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S), Error> {
        let _a;
        check!(A::raw_deserialize(from), _a);
        let _b;
        check!(B::raw_deserialize(from), _b);
        let _c;
        check!(C::raw_deserialize(from), _c);
        let _d;
        check!(D::raw_deserialize(from), _d);
        let _e;
        check!(E::raw_deserialize(from), _e);
        let _f;
        check!(F::raw_deserialize(from), _f);
        let _g;
        check!(G::raw_deserialize(from), _g);
        let _h;
        check!(H::raw_deserialize(from), _h);
        let _i;
        check!(I::raw_deserialize(from), _i);
        let _j;
        check!(J::raw_deserialize(from), _j);
        let _k;
        check!(K::raw_deserialize(from), _k);
        let _l;
        check!(L::raw_deserialize(from), _l);
        let _m;
        check!(M::raw_deserialize(from), _m);
        let _n;
        check!(N::raw_deserialize(from), _n);
        let _o;
        check!(O::raw_deserialize(from), _o);
        let _p;
        check!(P::raw_deserialize(from), _p);
        let _q;
        check!(Q::raw_deserialize(from), _q);
        let _r;
        check!(R::raw_deserialize(from), _r);
        let _s;
        check!(S::raw_deserialize(from), _s);
        Ok((_a, _b, _c, _d, _e, _f, _g, _h, _i, _j, _k, _l, _m, _n, _o, _p, _q, _r, _s))
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T> RawSerialize for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T) where A: RawSerialize,
    B: RawSerialize,
    C: RawSerialize,
    D: RawSerialize,
    E: RawSerialize,
    F: RawSerialize,
    G: RawSerialize,
    H: RawSerialize,
    I: RawSerialize,
    J: RawSerialize,
    K: RawSerialize,
    L: RawSerialize,
    M: RawSerialize,
    N: RawSerialize,
    O: RawSerialize,
    P: RawSerialize,
    Q: RawSerialize,
    R: RawSerialize,
    S: RawSerialize,
    T: RawSerialize {
    fn raw_serialize(&self, to: &mut Write) -> Result<u64, Error> {
        let mut size = 0;
        let size_x;
        check!(self.0.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.1.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.2.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.3.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.4.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.5.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.6.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.7.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.8.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.9.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.10.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.11.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.12.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.13.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.14.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.15.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.16.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.17.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.18.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.19.raw_serialize(to), size_x);
        size += size_x;
        Ok(size)
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T> RawDeserialize for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T) where A: RawDeserialize,
    B: RawDeserialize,
    C: RawDeserialize,
    D: RawDeserialize,
    E: RawDeserialize,
    F: RawDeserialize,
    G: RawDeserialize,
    H: RawDeserialize,
    I: RawDeserialize,
    J: RawDeserialize,
    K: RawDeserialize,
    L: RawDeserialize,
    M: RawDeserialize,
    N: RawDeserialize,
    O: RawDeserialize,
    P: RawDeserialize,
    Q: RawDeserialize,
    R: RawDeserialize,
    S: RawDeserialize,
    T: RawDeserialize {
    fn raw_deserialize(from: &mut Read) -> Result<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T), Error> {
        let _a;
        check!(A::raw_deserialize(from), _a);
        let _b;
        check!(B::raw_deserialize(from), _b);
        let _c;
        check!(C::raw_deserialize(from), _c);
        let _d;
        check!(D::raw_deserialize(from), _d);
        let _e;
        check!(E::raw_deserialize(from), _e);
        let _f;
        check!(F::raw_deserialize(from), _f);
        let _g;
        check!(G::raw_deserialize(from), _g);
        let _h;
        check!(H::raw_deserialize(from), _h);
        let _i;
        check!(I::raw_deserialize(from), _i);
        let _j;
        check!(J::raw_deserialize(from), _j);
        let _k;
        check!(K::raw_deserialize(from), _k);
        let _l;
        check!(L::raw_deserialize(from), _l);
        let _m;
        check!(M::raw_deserialize(from), _m);
        let _n;
        check!(N::raw_deserialize(from), _n);
        let _o;
        check!(O::raw_deserialize(from), _o);
        let _p;
        check!(P::raw_deserialize(from), _p);
        let _q;
        check!(Q::raw_deserialize(from), _q);
        let _r;
        check!(R::raw_deserialize(from), _r);
        let _s;
        check!(S::raw_deserialize(from), _s);
        let _t;
        check!(T::raw_deserialize(from), _t);
        Ok((_a, _b, _c, _d, _e, _f, _g, _h, _i, _j, _k, _l, _m, _n, _o, _p, _q, _r, _s, _t))
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U> RawSerialize for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U) where A: RawSerialize,
    B: RawSerialize,
    C: RawSerialize,
    D: RawSerialize,
    E: RawSerialize,
    F: RawSerialize,
    G: RawSerialize,
    H: RawSerialize,
    I: RawSerialize,
    J: RawSerialize,
    K: RawSerialize,
    L: RawSerialize,
    M: RawSerialize,
    N: RawSerialize,
    O: RawSerialize,
    P: RawSerialize,
    Q: RawSerialize,
    R: RawSerialize,
    S: RawSerialize,
    T: RawSerialize,
    U: RawSerialize {
    fn raw_serialize(&self, to: &mut Write) -> Result<u64, Error> {
        let mut size = 0;
        let size_x;
        check!(self.0.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.1.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.2.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.3.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.4.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.5.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.6.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.7.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.8.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.9.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.10.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.11.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.12.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.13.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.14.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.15.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.16.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.17.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.18.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.19.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.20.raw_serialize(to), size_x);
        size += size_x;
        Ok(size)
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U> RawDeserialize for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U) where A: RawDeserialize,
    B: RawDeserialize,
    C: RawDeserialize,
    D: RawDeserialize,
    E: RawDeserialize,
    F: RawDeserialize,
    G: RawDeserialize,
    H: RawDeserialize,
    I: RawDeserialize,
    J: RawDeserialize,
    K: RawDeserialize,
    L: RawDeserialize,
    M: RawDeserialize,
    N: RawDeserialize,
    O: RawDeserialize,
    P: RawDeserialize,
    Q: RawDeserialize,
    R: RawDeserialize,
    S: RawDeserialize,
    T: RawDeserialize,
    U: RawDeserialize {
    fn raw_deserialize(from: &mut Read) -> Result<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U), Error> {
        let _a;
        check!(A::raw_deserialize(from), _a);
        let _b;
        check!(B::raw_deserialize(from), _b);
        let _c;
        check!(C::raw_deserialize(from), _c);
        let _d;
        check!(D::raw_deserialize(from), _d);
        let _e;
        check!(E::raw_deserialize(from), _e);
        let _f;
        check!(F::raw_deserialize(from), _f);
        let _g;
        check!(G::raw_deserialize(from), _g);
        let _h;
        check!(H::raw_deserialize(from), _h);
        let _i;
        check!(I::raw_deserialize(from), _i);
        let _j;
        check!(J::raw_deserialize(from), _j);
        let _k;
        check!(K::raw_deserialize(from), _k);
        let _l;
        check!(L::raw_deserialize(from), _l);
        let _m;
        check!(M::raw_deserialize(from), _m);
        let _n;
        check!(N::raw_deserialize(from), _n);
        let _o;
        check!(O::raw_deserialize(from), _o);
        let _p;
        check!(P::raw_deserialize(from), _p);
        let _q;
        check!(Q::raw_deserialize(from), _q);
        let _r;
        check!(R::raw_deserialize(from), _r);
        let _s;
        check!(S::raw_deserialize(from), _s);
        let _t;
        check!(T::raw_deserialize(from), _t);
        let _u;
        check!(U::raw_deserialize(from), _u);
        Ok((_a, _b, _c, _d, _e, _f, _g, _h, _i, _j, _k, _l, _m, _n, _o, _p, _q, _r, _s, _t, _u))
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V> RawSerialize for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V) where A: RawSerialize,
    B: RawSerialize,
    C: RawSerialize,
    D: RawSerialize,
    E: RawSerialize,
    F: RawSerialize,
    G: RawSerialize,
    H: RawSerialize,
    I: RawSerialize,
    J: RawSerialize,
    K: RawSerialize,
    L: RawSerialize,
    M: RawSerialize,
    N: RawSerialize,
    O: RawSerialize,
    P: RawSerialize,
    Q: RawSerialize,
    R: RawSerialize,
    S: RawSerialize,
    T: RawSerialize,
    U: RawSerialize,
    V: RawSerialize {
    fn raw_serialize(&self, to: &mut Write) -> Result<u64, Error> {
        let mut size = 0;
        let size_x;
        check!(self.0.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.1.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.2.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.3.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.4.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.5.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.6.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.7.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.8.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.9.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.10.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.11.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.12.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.13.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.14.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.15.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.16.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.17.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.18.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.19.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.20.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.21.raw_serialize(to), size_x);
        size += size_x;
        Ok(size)
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V> RawDeserialize for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V) where A: RawDeserialize,
    B: RawDeserialize,
    C: RawDeserialize,
    D: RawDeserialize,
    E: RawDeserialize,
    F: RawDeserialize,
    G: RawDeserialize,
    H: RawDeserialize,
    I: RawDeserialize,
    J: RawDeserialize,
    K: RawDeserialize,
    L: RawDeserialize,
    M: RawDeserialize,
    N: RawDeserialize,
    O: RawDeserialize,
    P: RawDeserialize,
    Q: RawDeserialize,
    R: RawDeserialize,
    S: RawDeserialize,
    T: RawDeserialize,
    U: RawDeserialize,
    V: RawDeserialize {
    fn raw_deserialize(from: &mut Read) -> Result<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V), Error> {
        let _a;
        check!(A::raw_deserialize(from), _a);
        let _b;
        check!(B::raw_deserialize(from), _b);
        let _c;
        check!(C::raw_deserialize(from), _c);
        let _d;
        check!(D::raw_deserialize(from), _d);
        let _e;
        check!(E::raw_deserialize(from), _e);
        let _f;
        check!(F::raw_deserialize(from), _f);
        let _g;
        check!(G::raw_deserialize(from), _g);
        let _h;
        check!(H::raw_deserialize(from), _h);
        let _i;
        check!(I::raw_deserialize(from), _i);
        let _j;
        check!(J::raw_deserialize(from), _j);
        let _k;
        check!(K::raw_deserialize(from), _k);
        let _l;
        check!(L::raw_deserialize(from), _l);
        let _m;
        check!(M::raw_deserialize(from), _m);
        let _n;
        check!(N::raw_deserialize(from), _n);
        let _o;
        check!(O::raw_deserialize(from), _o);
        let _p;
        check!(P::raw_deserialize(from), _p);
        let _q;
        check!(Q::raw_deserialize(from), _q);
        let _r;
        check!(R::raw_deserialize(from), _r);
        let _s;
        check!(S::raw_deserialize(from), _s);
        let _t;
        check!(T::raw_deserialize(from), _t);
        let _u;
        check!(U::raw_deserialize(from), _u);
        let _v;
        check!(V::raw_deserialize(from), _v);
        Ok((_a, _b, _c, _d, _e, _f, _g, _h, _i, _j, _k, _l, _m, _n, _o, _p, _q, _r, _s, _t, _u, _v))
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W> RawSerialize for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W) where A: RawSerialize,
    B: RawSerialize,
    C: RawSerialize,
    D: RawSerialize,
    E: RawSerialize,
    F: RawSerialize,
    G: RawSerialize,
    H: RawSerialize,
    I: RawSerialize,
    J: RawSerialize,
    K: RawSerialize,
    L: RawSerialize,
    M: RawSerialize,
    N: RawSerialize,
    O: RawSerialize,
    P: RawSerialize,
    Q: RawSerialize,
    R: RawSerialize,
    S: RawSerialize,
    T: RawSerialize,
    U: RawSerialize,
    V: RawSerialize,
    W: RawSerialize {
    fn raw_serialize(&self, to: &mut Write) -> Result<u64, Error> {
        let mut size = 0;
        let size_x;
        check!(self.0.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.1.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.2.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.3.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.4.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.5.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.6.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.7.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.8.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.9.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.10.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.11.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.12.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.13.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.14.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.15.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.16.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.17.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.18.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.19.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.20.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.21.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.22.raw_serialize(to), size_x);
        size += size_x;
        Ok(size)
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W> RawDeserialize for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W) where A: RawDeserialize,
    B: RawDeserialize,
    C: RawDeserialize,
    D: RawDeserialize,
    E: RawDeserialize,
    F: RawDeserialize,
    G: RawDeserialize,
    H: RawDeserialize,
    I: RawDeserialize,
    J: RawDeserialize,
    K: RawDeserialize,
    L: RawDeserialize,
    M: RawDeserialize,
    N: RawDeserialize,
    O: RawDeserialize,
    P: RawDeserialize,
    Q: RawDeserialize,
    R: RawDeserialize,
    S: RawDeserialize,
    T: RawDeserialize,
    U: RawDeserialize,
    V: RawDeserialize,
    W: RawDeserialize {
    fn raw_deserialize(from: &mut Read) -> Result<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W), Error> {
        let _a;
        check!(A::raw_deserialize(from), _a);
        let _b;
        check!(B::raw_deserialize(from), _b);
        let _c;
        check!(C::raw_deserialize(from), _c);
        let _d;
        check!(D::raw_deserialize(from), _d);
        let _e;
        check!(E::raw_deserialize(from), _e);
        let _f;
        check!(F::raw_deserialize(from), _f);
        let _g;
        check!(G::raw_deserialize(from), _g);
        let _h;
        check!(H::raw_deserialize(from), _h);
        let _i;
        check!(I::raw_deserialize(from), _i);
        let _j;
        check!(J::raw_deserialize(from), _j);
        let _k;
        check!(K::raw_deserialize(from), _k);
        let _l;
        check!(L::raw_deserialize(from), _l);
        let _m;
        check!(M::raw_deserialize(from), _m);
        let _n;
        check!(N::raw_deserialize(from), _n);
        let _o;
        check!(O::raw_deserialize(from), _o);
        let _p;
        check!(P::raw_deserialize(from), _p);
        let _q;
        check!(Q::raw_deserialize(from), _q);
        let _r;
        check!(R::raw_deserialize(from), _r);
        let _s;
        check!(S::raw_deserialize(from), _s);
        let _t;
        check!(T::raw_deserialize(from), _t);
        let _u;
        check!(U::raw_deserialize(from), _u);
        let _v;
        check!(V::raw_deserialize(from), _v);
        let _w;
        check!(W::raw_deserialize(from), _w);
        Ok((_a, _b, _c, _d, _e, _f, _g, _h, _i, _j, _k, _l, _m, _n, _o, _p, _q, _r, _s, _t, _u, _v, _w))
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X> RawSerialize for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X) where A: RawSerialize,
    B: RawSerialize,
    C: RawSerialize,
    D: RawSerialize,
    E: RawSerialize,
    F: RawSerialize,
    G: RawSerialize,
    H: RawSerialize,
    I: RawSerialize,
    J: RawSerialize,
    K: RawSerialize,
    L: RawSerialize,
    M: RawSerialize,
    N: RawSerialize,
    O: RawSerialize,
    P: RawSerialize,
    Q: RawSerialize,
    R: RawSerialize,
    S: RawSerialize,
    T: RawSerialize,
    U: RawSerialize,
    V: RawSerialize,
    W: RawSerialize,
    X: RawSerialize {
    fn raw_serialize(&self, to: &mut Write) -> Result<u64, Error> {
        let mut size = 0;
        let size_x;
        check!(self.0.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.1.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.2.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.3.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.4.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.5.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.6.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.7.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.8.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.9.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.10.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.11.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.12.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.13.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.14.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.15.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.16.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.17.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.18.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.19.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.20.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.21.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.22.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.23.raw_serialize(to), size_x);
        size += size_x;
        Ok(size)
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X> RawDeserialize for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X) where A: RawDeserialize,
    B: RawDeserialize,
    C: RawDeserialize,
    D: RawDeserialize,
    E: RawDeserialize,
    F: RawDeserialize,
    G: RawDeserialize,
    H: RawDeserialize,
    I: RawDeserialize,
    J: RawDeserialize,
    K: RawDeserialize,
    L: RawDeserialize,
    M: RawDeserialize,
    N: RawDeserialize,
    O: RawDeserialize,
    P: RawDeserialize,
    Q: RawDeserialize,
    R: RawDeserialize,
    S: RawDeserialize,
    T: RawDeserialize,
    U: RawDeserialize,
    V: RawDeserialize,
    W: RawDeserialize,
    X: RawDeserialize {
    fn raw_deserialize(from: &mut Read) -> Result<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X), Error> {
        let _a;
        check!(A::raw_deserialize(from), _a);
        let _b;
        check!(B::raw_deserialize(from), _b);
        let _c;
        check!(C::raw_deserialize(from), _c);
        let _d;
        check!(D::raw_deserialize(from), _d);
        let _e;
        check!(E::raw_deserialize(from), _e);
        let _f;
        check!(F::raw_deserialize(from), _f);
        let _g;
        check!(G::raw_deserialize(from), _g);
        let _h;
        check!(H::raw_deserialize(from), _h);
        let _i;
        check!(I::raw_deserialize(from), _i);
        let _j;
        check!(J::raw_deserialize(from), _j);
        let _k;
        check!(K::raw_deserialize(from), _k);
        let _l;
        check!(L::raw_deserialize(from), _l);
        let _m;
        check!(M::raw_deserialize(from), _m);
        let _n;
        check!(N::raw_deserialize(from), _n);
        let _o;
        check!(O::raw_deserialize(from), _o);
        let _p;
        check!(P::raw_deserialize(from), _p);
        let _q;
        check!(Q::raw_deserialize(from), _q);
        let _r;
        check!(R::raw_deserialize(from), _r);
        let _s;
        check!(S::raw_deserialize(from), _s);
        let _t;
        check!(T::raw_deserialize(from), _t);
        let _u;
        check!(U::raw_deserialize(from), _u);
        let _v;
        check!(V::raw_deserialize(from), _v);
        let _w;
        check!(W::raw_deserialize(from), _w);
        let _x;
        check!(X::raw_deserialize(from), _x);
        Ok((_a, _b, _c, _d, _e, _f, _g, _h, _i, _j, _k, _l, _m, _n, _o, _p, _q, _r, _s, _t, _u, _v, _w, _x))
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y> RawSerialize for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y) where A: RawSerialize,
    B: RawSerialize,
    C: RawSerialize,
    D: RawSerialize,
    E: RawSerialize,
    F: RawSerialize,
    G: RawSerialize,
    H: RawSerialize,
    I: RawSerialize,
    J: RawSerialize,
    K: RawSerialize,
    L: RawSerialize,
    M: RawSerialize,
    N: RawSerialize,
    O: RawSerialize,
    P: RawSerialize,
    Q: RawSerialize,
    R: RawSerialize,
    S: RawSerialize,
    T: RawSerialize,
    U: RawSerialize,
    V: RawSerialize,
    W: RawSerialize,
    X: RawSerialize,
    Y: RawSerialize {
    fn raw_serialize(&self, to: &mut Write) -> Result<u64, Error> {
        let mut size = 0;
        let size_x;
        check!(self.0.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.1.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.2.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.3.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.4.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.5.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.6.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.7.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.8.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.9.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.10.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.11.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.12.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.13.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.14.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.15.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.16.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.17.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.18.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.19.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.20.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.21.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.22.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.23.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.24.raw_serialize(to), size_x);
        size += size_x;
        Ok(size)
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y> RawDeserialize for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y) where A: RawDeserialize,
    B: RawDeserialize,
    C: RawDeserialize,
    D: RawDeserialize,
    E: RawDeserialize,
    F: RawDeserialize,
    G: RawDeserialize,
    H: RawDeserialize,
    I: RawDeserialize,
    J: RawDeserialize,
    K: RawDeserialize,
    L: RawDeserialize,
    M: RawDeserialize,
    N: RawDeserialize,
    O: RawDeserialize,
    P: RawDeserialize,
    Q: RawDeserialize,
    R: RawDeserialize,
    S: RawDeserialize,
    T: RawDeserialize,
    U: RawDeserialize,
    V: RawDeserialize,
    W: RawDeserialize,
    X: RawDeserialize,
    Y: RawDeserialize {
    fn raw_deserialize(from: &mut Read) -> Result<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y), Error> {
        let _a;
        check!(A::raw_deserialize(from), _a);
        let _b;
        check!(B::raw_deserialize(from), _b);
        let _c;
        check!(C::raw_deserialize(from), _c);
        let _d;
        check!(D::raw_deserialize(from), _d);
        let _e;
        check!(E::raw_deserialize(from), _e);
        let _f;
        check!(F::raw_deserialize(from), _f);
        let _g;
        check!(G::raw_deserialize(from), _g);
        let _h;
        check!(H::raw_deserialize(from), _h);
        let _i;
        check!(I::raw_deserialize(from), _i);
        let _j;
        check!(J::raw_deserialize(from), _j);
        let _k;
        check!(K::raw_deserialize(from), _k);
        let _l;
        check!(L::raw_deserialize(from), _l);
        let _m;
        check!(M::raw_deserialize(from), _m);
        let _n;
        check!(N::raw_deserialize(from), _n);
        let _o;
        check!(O::raw_deserialize(from), _o);
        let _p;
        check!(P::raw_deserialize(from), _p);
        let _q;
        check!(Q::raw_deserialize(from), _q);
        let _r;
        check!(R::raw_deserialize(from), _r);
        let _s;
        check!(S::raw_deserialize(from), _s);
        let _t;
        check!(T::raw_deserialize(from), _t);
        let _u;
        check!(U::raw_deserialize(from), _u);
        let _v;
        check!(V::raw_deserialize(from), _v);
        let _w;
        check!(W::raw_deserialize(from), _w);
        let _x;
        check!(X::raw_deserialize(from), _x);
        let _y;
        check!(Y::raw_deserialize(from), _y);
        Ok((_a, _b, _c, _d, _e, _f, _g, _h, _i, _j, _k, _l, _m, _n, _o, _p, _q, _r, _s, _t, _u, _v, _w, _x, _y))
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z> RawSerialize for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z) where A: RawSerialize,
    B: RawSerialize,
    C: RawSerialize,
    D: RawSerialize,
    E: RawSerialize,
    F: RawSerialize,
    G: RawSerialize,
    H: RawSerialize,
    I: RawSerialize,
    J: RawSerialize,
    K: RawSerialize,
    L: RawSerialize,
    M: RawSerialize,
    N: RawSerialize,
    O: RawSerialize,
    P: RawSerialize,
    Q: RawSerialize,
    R: RawSerialize,
    S: RawSerialize,
    T: RawSerialize,
    U: RawSerialize,
    V: RawSerialize,
    W: RawSerialize,
    X: RawSerialize,
    Y: RawSerialize,
    Z: RawSerialize {
    fn raw_serialize(&self, to: &mut Write) -> Result<u64, Error> {
        let mut size = 0;
        let size_x;
        check!(self.0.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.1.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.2.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.3.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.4.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.5.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.6.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.7.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.8.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.9.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.10.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.11.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.12.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.13.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.14.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.15.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.16.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.17.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.18.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.19.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.20.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.21.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.22.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.23.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.24.raw_serialize(to), size_x);
        size += size_x;
        let size_x;
        check!(self.25.raw_serialize(to), size_x);
        size += size_x;
        Ok(size)
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z> RawDeserialize for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z) where A: RawDeserialize,
    B: RawDeserialize,
    C: RawDeserialize,
    D: RawDeserialize,
    E: RawDeserialize,
    F: RawDeserialize,
    G: RawDeserialize,
    H: RawDeserialize,
    I: RawDeserialize,
    J: RawDeserialize,
    K: RawDeserialize,
    L: RawDeserialize,
    M: RawDeserialize,
    N: RawDeserialize,
    O: RawDeserialize,
    P: RawDeserialize,
    Q: RawDeserialize,
    R: RawDeserialize,
    S: RawDeserialize,
    T: RawDeserialize,
    U: RawDeserialize,
    V: RawDeserialize,
    W: RawDeserialize,
    X: RawDeserialize,
    Y: RawDeserialize,
    Z: RawDeserialize {
    fn raw_deserialize(from: &mut Read) -> Result<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z), Error> {
        let _a;
        check!(A::raw_deserialize(from), _a);
        let _b;
        check!(B::raw_deserialize(from), _b);
        let _c;
        check!(C::raw_deserialize(from), _c);
        let _d;
        check!(D::raw_deserialize(from), _d);
        let _e;
        check!(E::raw_deserialize(from), _e);
        let _f;
        check!(F::raw_deserialize(from), _f);
        let _g;
        check!(G::raw_deserialize(from), _g);
        let _h;
        check!(H::raw_deserialize(from), _h);
        let _i;
        check!(I::raw_deserialize(from), _i);
        let _j;
        check!(J::raw_deserialize(from), _j);
        let _k;
        check!(K::raw_deserialize(from), _k);
        let _l;
        check!(L::raw_deserialize(from), _l);
        let _m;
        check!(M::raw_deserialize(from), _m);
        let _n;
        check!(N::raw_deserialize(from), _n);
        let _o;
        check!(O::raw_deserialize(from), _o);
        let _p;
        check!(P::raw_deserialize(from), _p);
        let _q;
        check!(Q::raw_deserialize(from), _q);
        let _r;
        check!(R::raw_deserialize(from), _r);
        let _s;
        check!(S::raw_deserialize(from), _s);
        let _t;
        check!(T::raw_deserialize(from), _t);
        let _u;
        check!(U::raw_deserialize(from), _u);
        let _v;
        check!(V::raw_deserialize(from), _v);
        let _w;
        check!(W::raw_deserialize(from), _w);
        let _x;
        check!(X::raw_deserialize(from), _x);
        let _y;
        check!(Y::raw_deserialize(from), _y);
        let _z;
        check!(Z::raw_deserialize(from), _z);
        Ok((_a, _b, _c, _d, _e, _f, _g, _h, _i, _j, _k, _l, _m, _n, _o, _p, _q, _r, _s, _t, _u, _v, _w, _x, _y, _z))
    }
}
