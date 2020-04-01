use afl;
use grass;

fn main() {
    afl::fuzz(|data: &[u8]| {
        if let Ok(s) = std::str::from_utf8(data) {
            let _ = grass::StyleSheet::new(&s);                    
        }
    });
}

