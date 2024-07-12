
```rust
use geonodes_bake_tool::bake_reader::BakeReader;

fn main() {
    let mut reader = BakeReader::new("tests/91383020", &["light", "hit"]);
    let geometry = reader.load_meta().unwrap();

    for (attribute_name, data) in geometry.domain.point.iter() {
        println!("attribute name: {}", attribute_name);
        for frame in data.iter() {
            dbg!(&frame.frame);
            
        }
    }
}
```
