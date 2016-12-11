use std::env;
use std::io::{self, Read, Error};
use std::fs::File;

const MEM_SIZE: usize = 1024 * 30;

fn read_program(filename: String) -> Result<String, Error> {
    let mut file = try!(File::open(filename));
    let mut content = String::new();
    try!(file.read_to_string(&mut content));
    Ok(content)
}

fn read_char() -> u8 {
    let mut buffer = vec![0_u8; 1];
    io::stdin().read(&mut buffer).unwrap();
    buffer[0]
}

fn main() {
    let filename = match env::args().skip(1).next() {
        Some(s) => s,
        None => panic!("No input file specified!"),
    };

    let program = read_program(filename).unwrap_or("".to_string());
    let mut mem = vec![0_u8; MEM_SIZE];
    let mut cur: usize = 0;

    let chars = program.chars().collect::<Vec<char>>();
    let mut loops = Vec::new();
    let mut i = 0;

    loop {
        let token = chars[i];
        match token {
            '>' => cur += 1,
            '<' => cur -= 1,
            '+' => mem[cur] = mem[cur].wrapping_add(1),
            '-' => mem[cur] = mem[cur].wrapping_sub(1),
            '.' => print!("{}", mem[cur] as char),
            ',' => mem[cur] = read_char(),
            '[' => loops.push(i),
            ']' => {
                if mem[cur] != 0 {
                    i = loops.pop().unwrap_or(0);
                    continue;
                }
            },
            _ => {},
        }
        i += 1;
        if i >= chars.len() - 1 {
            break;
        }
    }
}
