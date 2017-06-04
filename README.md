# RawSerde
RawSerde is a serialization library that allows raw serialization and deserialization of non-reference types.

# Example

```rust
    extern crate raw_serde;

    use raw_serde::*;

    #[derive(RawSerialize, RawDeserialize, Debug, PartialEq, Eq)]
    struct test_struct {
	      x: i32,
	      y: i32,
        z: i128
    }

    #[test]
    fn test_vec() {
        let mut v = vec![];
        for _ in 0..10 {
            let mut x = HashMap::<String, i32>::new();
            for i in 0..100 {
                x.insert(i.to_string(), i);
            }
            v.push(x);
        }
        let mut file = OpenOptions::new().read(true).write(true).create(true).open("test_vec.dat").unwrap();
        v.raw_serialize(&mut file).unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();
        let x = Vec::<HashMap<String, i32>>::raw_deserialize(&mut file).unwrap();
        assert!(x == v)
    }
```
