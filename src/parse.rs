use crate::crontab::{Constraint, Entry, Schedule};
use std::borrow::Borrow;
use std::fs::File;
use std::iter::Peekable;
use std::path::PathBuf;
use std::str::Chars;

pub fn read_u8(iter: &mut Peekable<Chars>) -> Option<u8> {
    let mut chars = Vec::with_capacity(4);
    while let Some(c) = iter.peek() {
        if c.is_ascii_digit() {
            chars.push(*c);
            iter.next();
        } else {
            break;
        }
    }

    match chars.iter().collect::<String>().parse() {
        Ok(u8) => Some(u8),
        Err(_) => None,
    }
}

pub fn skip_whitespaces(iter: &mut Peekable<Chars>) {
    while let Some(c) = iter.peek() {
        if *c == ' ' || *c == '\t' {
            iter.next();
        } else {
            break;
        }
    }
}

pub fn next_part(iter: &mut Peekable<Chars>) -> String {
    skip_whitespaces(iter);

    let mut res = Vec::new();
    for c in iter {
        match c {
            ' ' | '\t' => break,
            c => res.push(c),
        }
    }

    res.iter().collect()
}

pub fn parse_file(path: PathBuf) -> Option<Vec<Entry>> {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    if !path.exists() {
        File::create(&path).unwrap();
    }

    let file = std::fs::read_to_string(path).unwrap();
    let mut entries = Vec::new();

    for line in file.lines() {
        let line = line.trim_matches([' ', '\t']);
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let mut iter = line.chars().peekable();
        let minute = Constraint::parse(next_part(&mut iter).borrow(), 60, false)?;
        let hour = Constraint::parse(next_part(&mut iter).borrow(), 24, false)?;
        let month_day = Constraint::parse(next_part(&mut iter).borrow(), 32, true)?;
        let month = Constraint::parse(next_part(&mut iter).borrow(), 13, true)?;
        let week_day = Constraint::parse(next_part(&mut iter).borrow(), 7, false)?;

        skip_whitespaces(&mut iter);
        let command = iter.collect();

        let schedule = Schedule::new(minute, hour, month_day, month, week_day);

        entries.push(Entry::new(command, schedule))
    }

    Some(entries)
}

impl Constraint {
    pub fn parse(input: &str, limit: u8, one_indexed: bool) -> Option<Constraint> {
        let mut valid = vec![false; limit as usize];

        let mut iter = input.chars().peekable();
        loop {
            // first parse range
            let peek = iter.peek()?;
            let mut begin = 0;
            let mut end = 0;
            let mut can_have_step = false;
            let mut step = 1;
            match peek {
                '0'..='9' => {
                    begin = read_u8(&mut iter)?;
                    end = begin;
                    if let Some('-') = iter.peek() {
                        iter.next();
                        end = read_u8(&mut iter)?;
                        can_have_step = true;
                    }
                }
                '*' => {
                    iter.next();
                    begin = if one_indexed { 1 } else { 0 };
                    end = limit - 1;
                    can_have_step = true;
                }
                _ => (),
            }

            // if we allow step next, try parse it
            if can_have_step {
                if let Some('/') = iter.peek() {
                    iter.next();
                    step = read_u8(&mut iter)?;
                }
            }

            if begin >= limit || end >= limit {
                return None;
            }

            if one_indexed && (begin == 0 || end == 0) {
                return None;
            }

            while begin <= end {
                valid[begin as usize] = true;
                begin += step;
            }

            if iter.peek().is_none() {
                break;
            } else if iter.next() != Some(',') {
                return None;
            }
        }

        Some(Constraint::new(valid))
    }
}
