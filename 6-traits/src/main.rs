use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::io;
use std::io::BufRead;

#[derive(Clone)]
enum Input {
    Number(u32),
    Float(f32),
    Str(String),
}

impl Input {
    fn cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Input::Number(n1), Input::Number(n2)) => Some(n1.cmp(n2)),
            (Input::Float(n1), Input::Float(n2)) => n1.partial_cmp(n2),
            (Input::Str(s1), Input::Str(s2)) => Some(s1.cmp(s2)),
            _ => None,
        }
    }

    fn prn(&self) {
        match self {
            Input::Number(n) => println!("{}", n),
            Input::Float(n) => println!("{}", n),
            Input::Str(s) => println!("{}", s)
        }
    }
}

fn parse_int(line: &String) -> Option<Input> {
    line.parse::<u32>().map(|n| Input::Number(n)).ok()
}

fn parse_float(line: &String) -> Option<Input> {
    line.parse::<f32>().map(|n| Input::Float(n)).ok()
}

fn parse_str(line: String) -> Input {
    Input::Str(line)
}

fn parse_input(line: String) -> Input {
    parse_int(&line)
        .or_else(|| parse_float(&line))
        .unwrap_or_else(|| parse_str(line))
}

fn main() {
    let mut largest: Option<Input> = None;

    for line in io::stdin().lock().lines() {
        let i = parse_input(line.expect("Failed to read line"));
        largest = match largest {
            None => Some(i),
            Some(lrg) => match i.cmp(&lrg) {
                Some(Ordering::Greater) => Some(i),
                _ => Some(lrg)
            }
        }
    }

    if let Some(input) = largest {
        input.prn()
    }
}
