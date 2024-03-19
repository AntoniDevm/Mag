use std::io::{self, Write};

pub fn input(text: &str) -> Result<String, Box<dyn std::error::Error>> {
    let ins = io::stdin();
    let mut outs = io::stdout();
    outs.write(text.as_bytes())?;
    outs.flush()?;
    let mut input = String::new();
    ins.read_line(&mut input)?;
    Ok(input.get(0..input.len() - 1).unwrap().to_string())
}

// pub fn from_lsb(array: &[u8]) -> u64 {
//     let mut out = 0;

//     for (i, byte) in array.iter().enumerate() {
//         out += (*byte << (i * 8) )as u64;
//     }
//     out
// }


