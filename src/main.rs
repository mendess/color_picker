use ansi_term::Color;
use std::{
    env,
    io::{self, stdin, BufRead, BufReader},
    num::ParseIntError,
    str,
};

fn parse(color: &str) -> Result<Color, ParseIntError> {
    let color = color.trim_matches('#');
    match color.len() {
        0 => Ok(Color::RGB(0, 0, 0)),
        1..=2 => Ok(Color::RGB(0, 0, u8::from_str_radix(color, 16)?)),
        3..=4 => {
            let (g, b) = color.split_at(color.len() - 2);
            Ok(Color::RGB(
                0,
                u8::from_str_radix(g, 16)?,
                u8::from_str_radix(b, 16)?,
            ))
        }
        5..=6 => {
            let (rest, b) = color.split_at(color.len() - 2);
            let (r, g) = rest.split_at(rest.len() - 2);
            Ok(Color::RGB(
                u8::from_str_radix(r, 16)?,
                u8::from_str_radix(g, 16)?,
                u8::from_str_radix(b, 16)?,
            ))
        }
        v => panic!("String '{}' too long: {}", color, v),
    }
}

fn main() -> io::Result<()> {
    let stdin = stdin();
    let reader = BufReader::new(stdin.lock());
    let _: io::Result<()> = if env::args().any(|a| a == "-f" || a == "--find") {
        reader.split(b'#').enumerate().try_for_each(|(i, l)| {
            let line = l?;
            let l = str::from_utf8(&line)
                .unwrap_or_else(|e| str::from_utf8(&line[..e.valid_up_to()]).unwrap());
            if i != 0 {
                let c = parse(&l[..6]).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                Ok(print!("{}{}{}", c.paint("#"), c.paint(&l[..6]), &l[6..]))
            } else {
                Ok(print!("{}", l))
            }
        })
    } else {
        reader.lines().try_for_each(|l| {
            let l = l?;
            Ok(println!(
                "{}",
                parse(&l)
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
                    .paint(&l)
            ))
        })
    };
    Ok(())
}

mod test {
    #[test]
    fn test_parse() {
        use super::parse;
        use ansi_term::Color;

        assert!((0..16777215).all(|i| {
            let s = format!("#{:x}", i);
            match parse(&s) {
                Ok(Color::RGB(r, g, b)) => {
                    dbg!(r, g, b);
                    ((r as usize) << 16) + ((g as usize) << 8) + b as usize == i
                }
                e => {
                    println!("{:?}", e);
                    false
                }
            }
        }))
    }

    #[test]
    fn test_parse_padded() {
        use super::parse;
        use ansi_term::Color;

        assert!((0..1048575).all(|i| {
            let s = format!("#{:0>6x}", i);
            match parse(&s) {
                Ok(Color::RGB(r, g, b)) => {
                    dbg!(r, g, b);
                    ((r as usize) << 16) + ((g as usize) << 8) + b as usize == i
                }
                e => {
                    println!("{:?}", e);
                    false
                }
            }
        }))
    }
}
