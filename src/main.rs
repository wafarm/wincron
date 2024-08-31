#![cfg_attr(windows, windows_subsystem = "windows")]

use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::crontab::Entry;
use chrono::Local;
use notify::{RecursiveMode, Watcher};

mod crontab;
mod parse;

fn main() {
    let mut entries = parse::parse_file(crontab::path());
    let mut queue = Vec::new();
    let refresh = Arc::new(Mutex::new(false));

    let mut watcher = {
        let refresh = refresh.clone();
        notify::recommended_watcher(move |res| match res {
            Ok(_) => *refresh.lock().unwrap() = true,
            Err(e) => panic!("watch error: {e}"),
        })
        .unwrap()
    };

    watcher
        .watch(crontab::path().as_path(), RecursiveMode::NonRecursive)
        .unwrap();

    loop {
        {
            let mut refresh = refresh.lock().unwrap();
            if *refresh {
                eprintln!("Detected crontab update, reloading file");
                *refresh = false;
                entries = parse::parse_file(crontab::path());
                queue.clear();
            }
        }

        if !queue.is_empty() {
            eprintln!(
                "Running {} scheduled task{}",
                queue.len(),
                if queue.len() == 1 { "" } else { "s" }
            );
            queue.iter().for_each(|x: &Entry| {
                let command = x.command().to_owned();
                thread::spawn(move || {
                    #[cfg(windows)]
                    Command::new("cmd").args(["/c", &command]).spawn().unwrap();
                    #[cfg(unix)]
                    Command::new("sh").args(["-c", &command]).spawn().unwrap();
                });
            });

            queue.clear();
            thread::sleep(Duration::from_millis(500));
        }

        let mut sleep_time = 10000;
        if let Some(entries) = &entries {
            let time = Local::now();
            let mut nearest = None;

            entries.iter().for_each(|x| {
                if nearest.is_none() || x.next_run(&time) < nearest.unwrap() {
                    nearest = Some(x.next_run(&time));
                }
            });

            if let Some(nearest) = nearest {
                let delta = nearest.signed_duration_since(time);
                let until = delta.num_milliseconds();
                if until < 10000 {
                    sleep_time = until;
                    entries
                        .iter()
                        .filter(|x| x.next_run(&time) == nearest)
                        .for_each(|x| queue.push(x.clone()));
                }
            }
        } else {
            eprintln!("Wrong crontab format, doing nothing");
        }

        thread::sleep(Duration::from_millis(sleep_time as u64));
    }
}
