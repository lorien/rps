use std::env;
use std::process;

fn parse_number(inp: &str) -> u32 {
    let mut ret = inp;
    let mut mult: u32 = 1;
    let last_char = inp.chars().last().unwrap();
    if last_char.is_digit(10) {
    } else {
         mult = match last_char {
            'k' => 1024,
            'm' => 1024_u32.pow(2),
            'b' => 1024_u32.pow(3),
            _ => panic!(format!(
                "Invalid numeric suffix: {}", last_char
            ))
        };
        ret = &inp[0..(inp.len() - 1)];
    }
    return ret.parse::<u32>().unwrap() * mult;
}

fn format_time(inp: f64) -> String {
    let mut ret = String::from("");
    let mut sec = inp;
    if sec < 1.0 {
        ret.push_str("less a second");
    } else {
        let mut levels = vec![
            (3600, 'h'),
            (60, 'm'),
            (1, 's'),
        ];
        loop {
            if levels.len() == 0 {
                break
            }
            let level = levels.remove(0);
            if sec > level.0 as f64 {
                let val = (sec / level.0 as f64) as u32;
                sec = sec - (val as f64 * level.0 as f64);
                ret.push_str(&format!("{}{} ", val, level.1));
            }
        }
    } 
    ret
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: rps <speed> <amount>");
        process::exit(1);
    }
    let rps: u32 = parse_number(&args[1]);
    let amount: u32 = parse_number(&args[2]);
    //println!("debug: rps: {}", rps);
    //println!("debug: amount: {}", amount);
    println!("{}", format_time(amount as f64 / rps as f64))
}
