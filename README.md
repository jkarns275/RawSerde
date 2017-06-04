# RawSerde
RawSerde is a serialization library that allows raw serialization and deserialization of non-reference types.

# Example
## Cargo.toml
```toml
[dependencies]
raw_serde = "*"
```
## example.rs
```rust
    extern crate raw_serde;

    use raw_serde::*;

    #[derive(RawSerialize, RawDeserialize, Debug, PartialEq, Eq)]
    struct test_struct {
	    x: i32,
	    y: i32,
        z: i128
    }

    fn test() {
        // Create some data
        let mut v = vec![];
        for _ in 0..10 {
            let mut x = HashMap::<String, i32>::new();
            for i in 0..100 {
                x.insert(i.to_string(), i);
            }
            v.push(x);
        }
        
        // Create a file to read from and write to
        let mut file = OpenOptions::new().read(true).write(true).create(true).open("test_vec.dat").unwrap();
        
        // Write out vector of hashmaps
        v.raw_serialize(&mut file).unwrap();
        
        // To read the data back from the file, we have to move the file cursor to where we started writing data,
        // which is at position 0 since we started writing to a new file
        file.seek(SeekFrom::Start(0)).unwrap();
        
        // Read the data back, unwrap it
        let x = Vec::<HashMap<String, i32>>::raw_deserialize(&mut file).unwrap();
        assert!(x == v)
    }
```
