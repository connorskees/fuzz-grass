#[macro_use]
extern crate afl;
use grass;

fn main() {
        fuzz!(|data: &[u8]| {
        if let Ok(s) = std::str::from_utf8(data) {
            let _ = grass::from_string(s.to_string(), &grass::Options::default());                    
        }
    });
}

